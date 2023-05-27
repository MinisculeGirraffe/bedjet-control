// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod proto;

use std::collections::HashMap;
use std::sync::Arc;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::future::join_all;
use proto::SERVICE_UUID;
use std::time::Duration;
use tauri::async_runtime::Mutex;
use tauri::State;
use tokio::time;

#[derive(Debug, Default)]
struct BTAdapters(Arc<Mutex<HashMap<String, Adapter>>>);
impl BTAdapters {
    async fn get_adapter(&self, name: &str) -> Option<Adapter> {
        let adapter_map = self.0.lock().await;
        let adapter = adapter_map.get(name).cloned();
        // If the name doesnt exist. Try and get an adapter that's in the map
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

fn main() {
    tauri::Builder::default()
        .manage(BTAdapters::default())
        .manage(BTPeripherals::default())
        .invoke_handler(tauri::generate_handler![get_btle_adapters, scan_bedjets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
