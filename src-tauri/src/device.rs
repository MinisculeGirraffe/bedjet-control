use crate::{proto::DeviceStatus, Command, Decode, Encode, InterfaceError};
use btleplug::{
    api::{Characteristic, Peripheral as _, WriteType},
    platform::Peripheral,
};
use futures::StreamExt;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};
use thiserror::Error;
use tokio::sync::watch;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Failed to convert protocol stuff into bytes")]
    InterfaceError(#[from] InterfaceError),
    #[error("Bluetooth Error {0}")]
    BluetoothError(#[from] btleplug::Error),
}

#[derive(Debug)]
/// The primary interface for interacting with the device.
pub struct BedJet {
    peripheral: Peripheral,
    device_status: Characteristic,
    friendly_name: Characteristic,
    wifi_ssid: Characteristic,
    wifi_password: Characteristic,
    command: Characteristic,
    extended_data: Characteristic,
    device_status_send: watch::Sender<Option<DeviceStatus>>,
}

impl BedJet {
    pub const SERVICE_UUID: Uuid = Uuid::from_u128(324577607269236719219879600350580);
    pub const DEVICE_STATUS_UUID: Uuid = Uuid::from_u128(649096160927663446003035620926836);
    pub const FRIENDLY_NAME_UUID: Uuid = Uuid::from_u128(649175389090177710340629164877172);
    pub const WIFI_SSID_UUID: Uuid = Uuid::from_u128(649254617252691974678222708827508);
    pub const WIFI_PASSWORD_UUID: Uuid = Uuid::from_u128(649333845415206239015816252777844);
    pub const COMMANDS_UUID: Uuid = Uuid::from_u128(649413073577720503353409796728180);
    pub const EXTENDED_DATA_UUID: Uuid = Uuid::from_u128(649492301740234767691003340678516);

    pub fn from_peripheral(peripheral: Peripheral) -> Option<Self> {
        let mut map: HashMap<Uuid, Characteristic> = peripheral
            .characteristics()
            .into_iter()
            .map(|c| (c.uuid, c))
            .collect();

        let (device_status_send, _) = watch::channel(None);

        Some(Self {
            peripheral,
            device_status: map.remove(&Self::DEVICE_STATUS_UUID)?,
            friendly_name: map.remove(&Self::FRIENDLY_NAME_UUID)?,
            wifi_ssid: map.remove(&Self::WIFI_PASSWORD_UUID)?,
            wifi_password: map.remove(&Self::WIFI_PASSWORD_UUID)?,
            command: map.remove(&Self::COMMANDS_UUID)?,
            extended_data: map.remove(&Self::EXTENDED_DATA_UUID)?,
            device_status_send,
        })
    }

    async fn handle_notifications(&self) -> Result<(), DeviceError> {
        let mut stream = self.peripheral.notifications().await?;

        while let Some(msg) = stream.next().await {
            let _ = match msg.uuid {
                BedJet::DEVICE_STATUS_UUID => self.handle_device_status(msg.value).await,
                _ => Ok(()),
            };
        }

        Ok(())
    }

    async fn listen_status(&self) -> Result<(), btleplug::Error> {
        self.peripheral.subscribe(&self.device_status).await
    }

    async fn unlisten_status(&self) -> Result<(), btleplug::Error> {
        self.peripheral.unsubscribe(&self.device_status).await
    }
    pub async fn get_status(&self) -> Result<DeviceStatus, watch::error::RecvError> {
        let mut recv = self.device_status_send.subscribe();

        let status = recv
            .wait_for(|val| val.is_some())
            .await?
            .to_owned()
            .expect("Value was checked as Some, and was actually None");

        Ok(status)
    }
    async fn handle_device_status(&self, message: Vec<u8>) -> Result<(), DeviceError> {
        // Calculate this up here before the cursor takes ownership
        let has_enough_bytes = message.get(0).is_some_and(|val| *val == 0);
        let mut cursor = Cursor::new(message);
        // We want to skip that first byte since it's just informational
        cursor.set_position(1);

        let status: DeviceStatus;
        // If the entire packet isn't contained in the message we received
        if !has_enough_bytes {
            // grab the rest of it
            let rest: Vec<u8> = self.peripheral.read(&self.device_status).await?;
            // and decode it
            status = DeviceStatus::read_from(cursor.chain(Cursor::new(rest)))?;
        } else {
            // otherwise just decode it with what we have
            status = DeviceStatus::read_from(cursor)?;
        }

        let _ = self.device_status_send.send_replace(Some(status));
        Ok(())
    }

    pub async fn get_friendly_name(&self) -> String {
        let data = self.peripheral.read(&self.friendly_name).await.unwrap();

        String::from_utf8(data).unwrap()
    }
    pub async fn send_command(&self, command: Command) -> Result<(), DeviceError> {
        let data = command.encode()?;
        self.peripheral
            .write(&self.command, &data, WriteType::WithoutResponse)
            .await?;

        Ok(())
    }
}
