// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod proto;

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, ValueNotification};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::future::join_all;
use futures::{Stream, StreamExt};
use proto::{Command, DEVICE_STATUS, SERVICE_UUID};
use std::time::Duration;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Manager as _, State};
use tokio::time;

use crate::proto::{Decode, DeviceStatus, DeviceStatusEvent, Encode, COMMANDS};

#[derive(Debug, Default)]
struct BTAdapters(Arc<Mutex<HashMap<String, Adapter>>>);
impl BTAdapters {
    async fn get_adapter(&self, name: &str) -> Option<Adapter> {
        let adapter_map = self.0.lock().await;
        let adapter = adapter_map.get(name).cloned();
        // If the name doesn't exist. Try and get an adapter that's in the map
        if adapter.is_none() {
            return adapter_map.values().next().cloned();
        }

        adapter
    }
}

#[tauri::command]
async fn get_btle_adapters(state: State<'_, BTAdapters>) -> Result<Vec<String>, ()> {
    let manager = Manager::new().await.ok().unwrap();
    let adapters = manager.adapters().await.ok().unwrap_or_default();

    let mut adapter_map: HashMap<String, Adapter> = HashMap::new();

    for adapter in adapters {
        let Ok(info) = adapter.adapter_info().await else {
            continue;
        };
        adapter_map.insert(info, adapter);
    }

    let response: Vec<String> = adapter_map.keys().cloned().collect();

    let mut state_adapters = state.0.lock().await;
    *state_adapters = adapter_map;

    Ok(response)
}

#[derive(Debug, Default)]
struct BTPeripherals(Arc<Mutex<HashMap<String, Peripheral>>>);
impl BTPeripherals {
    async fn get_peripheral(&self, id: &str) -> Option<Peripheral> {
        self.0.lock().await.get(id).cloned()
    }
}

#[tauri::command]
async fn scan_bedjets(
    adapter_state: State<'_, BTAdapters>,
    peripheral_state: State<'_, BTPeripherals>,
    adapter: String,
) -> Result<Vec<String>, ()> {
    let adapter = adapter_state.get_adapter(&adapter).await.unwrap();
    adapter
        .start_scan(ScanFilter {
            services: vec![SERVICE_UUID],
        })
        .await
        .unwrap();

    // Wait for discovery
    time::sleep(Duration::from_secs(2)).await;
    let peripherals = adapter.peripherals().await.unwrap();
    join_all(peripherals.iter().map(|i| i.discover_services())).await;
    let mut periph_map: HashMap<String, Peripheral> = HashMap::new();
    for peripheral in peripherals.into_iter() {
        periph_map.insert(peripheral.id().to_string(), peripheral);
    }

    let response = periph_map.keys().cloned().collect();
    *peripheral_state.0.lock().await = periph_map;

    Ok(response)
}
#[tauri::command]
async fn connect_bedjet(
    peripheral_state: State<'_, BTPeripherals>,
    handle: AppHandle,
    bedjetid: String,
) -> Result<(), ()> {
    let id = peripheral_state.get_peripheral(&bedjetid).await.unwrap();
    id.connect().await.unwrap();
    id.discover_services().await.unwrap();

    tauri::async_runtime::spawn(async move { handle_notify(id, handle).await });

    Ok(())
}

#[tauri::command]
async fn disconnect_bedjet(
    peripheral_state: State<'_, BTPeripherals>,
    bedjetid: String,
) -> Result<(), ()> {
    let id = peripheral_state.get_peripheral(&bedjetid).await.unwrap();

    id.disconnect().await.unwrap();

    Ok(())
}

#[tauri::command]
async fn send_command(
    peripheral_state: State<'_, BTPeripherals>,
    bedjetid: String,
    command: Command,
) -> Result<(), ()> {
    println!("Got Command: {command:#?}");
    let periph = peripheral_state.get_peripheral(&bedjetid).await.unwrap();

    let command_char = periph
        .clone()
        .characteristics()
        .into_iter()
        .find(|i| i.uuid == COMMANDS)
        .unwrap();

    periph
        .write(
            &command_char,
            &command.encode(),
            btleplug::api::WriteType::WithoutResponse,
        )
        .await.unwrap();
        
    Ok(())
}

type NotificationStream = Pin<Box<dyn Stream<Item = ValueNotification> + Send>>;

async fn handle_notify(bedjet: Peripheral, handle: AppHandle) {
    let status_char = bedjet
        .characteristics()
        .iter()
        .find(|c| c.uuid == DEVICE_STATUS)
        .cloned()
        .unwrap();
    println!("Found Status: {status_char:#?}");

    bedjet
        .subscribe(&status_char)
        .await
        .expect("Failed to subscribe to status");

    let mut msg_stream = bedjet.notifications().await.unwrap();

    while let Some(mut msg) = msg_stream.next().await {
        if msg.value[0] != 0 {
            if let Ok(mut msg_continued) = bedjet.read(&status_char).await {
                msg.value.append(&mut msg_continued);
            }
        }
        let data = &msg.value[1..];

        if let Some(status) = DeviceStatus::decode(data) {
            println!("{status:#?}");
            handle
                .emit_all(
                    "DeviceStatus",
                    DeviceStatusEvent {
                        id: bedjet.id().to_string(),
                        status,
                    },
                )
                .unwrap();
        };
    }
}

fn main() {
    tauri::Builder::default()
        .manage(BTAdapters::default())
        .manage(BTPeripherals::default())
        .invoke_handler(tauri::generate_handler![
            get_btle_adapters,
            scan_bedjets,
            connect_bedjet,
            disconnect_bedjet,
            send_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
