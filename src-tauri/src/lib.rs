use std::collections::HashMap;
use btleplug::{
    api::{Characteristic, Peripheral as _, WriteType},
    platform::Peripheral,
};
use proto::{ButtonCode, CommandClass, Encode};
use serde::{Deserialize, Serialize};
use tauri::async_runtime::JoinHandle;
use typeshare::typeshare;
use uuid::Uuid;

pub mod proto; 

#[derive(Debug)]
pub struct BedJet {
    pub peripheral: Peripheral,
    pub device_status: Characteristic,
    pub friendly_name: Characteristic,
    pub wifi_ssid: Characteristic,
    pub wifi_password: Characteristic,
    pub commands: Characteristic,
    pub extended_data: Characteristic,
    subscribe_task: Option<JoinHandle<()>>,
}

impl BedJet {
    pub const SERVICE_UUID: Uuid = Uuid::from_u128(324577607269236719219879600350580);
    pub const DEVICE_STATUS: Uuid = Uuid::from_u128(649096160927663446003035620926836);
    pub const FRIENDLY_NAME: Uuid = Uuid::from_u128(649175389090177710340629164877172);
    pub const WIFI_SSID: Uuid = Uuid::from_u128(649254617252691974678222708827508);
    pub const WIFI_PASSWORD: Uuid = Uuid::from_u128(649333845415206239015816252777844);
    pub const COMMANDS: Uuid = Uuid::from_u128(649413073577720503353409796728180);
    pub const EXTENDED_DATA: Uuid = Uuid::from_u128(649492301740234767691003340678516);

    pub fn from_peripheral(peripheral: Peripheral) -> Option<Self> {
        let mut map: HashMap<Uuid, Characteristic> = peripheral
            .characteristics()
            .into_iter()
            .map(|c| (c.uuid, c))
            .collect();

        Some(Self {
            peripheral,
            device_status: map.remove(&Self::DEVICE_STATUS)?,
            friendly_name: map.remove(&Self::FRIENDLY_NAME)?,
            wifi_ssid: map.remove(&Self::WIFI_SSID)?,
            wifi_password: map.remove(&Self::WIFI_PASSWORD)?,
            commands: map.remove(&Self::COMMANDS)?,
            extended_data: map.remove(&Self::EXTENDED_DATA)?,
            subscribe_task: None,
        })
    }

    pub async fn get_friendly_name(&self) -> String {
        let data = self.peripheral.read(&self.friendly_name).await.unwrap();

        String::from_utf8(data).unwrap()
    }
    pub async fn send_command(&self, command: Command) -> Result<(), btleplug::Error> {
        self.peripheral
            .write(
                &self.commands,
                &command.encode(),
                WriteType::WithoutResponse,
            )
            .await?;

        Ok(())
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
pub enum Command {
    Button(ButtonCode),
    SetTime { hours: u8, minutes: u8 },
    SetTemp(Temp),
    SetFan(Fan),
    SetClock { hours: u8, minutes: u8 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[typeshare]
pub enum Temp {
    Celcius(u8),
    Fahrenheit(u8),
}

impl Temp {
    fn encode_byte(&self) -> u8 {
        match self {
            Temp::Celcius(val) => val.saturating_mul(2),
            Temp::Fahrenheit(val) => val
                .saturating_sub(32)
                .saturating_mul(5)
                .saturating_div(9)
                .saturating_mul(2),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[typeshare]
pub enum Fan {
    Step(u8),
    Percent(u8),
}

impl Fan {
    pub fn encode_byte(&self) -> u8 {
        match self {
            Fan::Step(val) => *val,
            Fan::Percent(val) => val.saturating_div(5).saturating_sub(1),
        }
    }
}
impl Encode for Command {
    fn encode(&self) -> Vec<u8> {
        match self {
            Command::Button(code) => vec![CommandClass::Button as u8, *code as u8],
            Command::SetTime { hours, minutes } => {
                vec![CommandClass::SetTime as u8, *hours, *minutes]
            }
            Command::SetTemp(temp) => vec![CommandClass::SetTemp as u8, temp.encode_byte()],
            Command::SetFan(fan) => vec![CommandClass::SetFan as u8, fan.encode_byte()],
            Command::SetClock { hours, minutes } => {
                vec![CommandClass::SetClock as u8, *hours, *minutes]
            }
        }
    }
}
