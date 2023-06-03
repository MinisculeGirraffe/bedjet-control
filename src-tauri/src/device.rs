use std::collections::HashMap;

use btleplug::{
    api::{Characteristic, Peripheral as _, WriteType},
    platform::Peripheral,
};
use tauri::async_runtime::JoinHandle;
use thiserror::Error;
use uuid::Uuid;

use crate::{Command, Encode, InterfaceError};

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
    _subscribe_task: Option<JoinHandle<()>>,
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

        Some(Self {
            peripheral,
            device_status: map.remove(&Self::DEVICE_STATUS_UUID)?,
            friendly_name: map.remove(&Self::FRIENDLY_NAME_UUID)?,
            wifi_ssid: map.remove(&Self::WIFI_PASSWORD_UUID)?,
            wifi_password: map.remove(&Self::WIFI_PASSWORD_UUID)?,
            command: map.remove(&Self::COMMANDS_UUID)?,
            extended_data: map.remove(&Self::EXTENDED_DATA_UUID)?,
            _subscribe_task: None,
        })
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
