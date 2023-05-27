use btleplug::{
    api::{Characteristic, Peripheral as _},
    platform::Peripheral,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::Serialize;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

pub const SERVICE_UUID: Uuid = Uuid::from_u128(324577607269236719219879600350580);
pub const DEVICE_STATUS: Uuid = Uuid::from_u128(649096160927663446003035620926836);
pub const FRIENDLY_NAME: Uuid = Uuid::from_u128(649175389090177710340629164877172);
pub const WIFI_SSID: Uuid = Uuid::from_u128(649254617252691974678222708827508);
pub const WIFI_PASSWORD: Uuid = Uuid::from_u128(649333845415206239015816252777844);
pub const COMMANDS: Uuid = Uuid::from_u128(649413073577720503353409796728180);
pub const EXTENDED_DATA: Uuid = Uuid::from_u128(649492301740234767691003340678516);

#[derive(Debug)]
pub struct BedJet {
   pub peripheral: Peripheral,
   pub device_status: Characteristic,
   pub friendly_name: Characteristic,
   pub wifi_ssid: Characteristic,
   pub wifi_password: Characteristic,
   pub commands: Characteristic,
   pub extended_data: Characteristic,
}

impl BedJet {
    pub fn from_peripheral(peripheral: Peripheral) -> Option<Self> {
        let mut map: HashMap<Uuid, Characteristic> = peripheral
            .characteristics()
            .into_iter()
            .map(|c| (c.uuid, c))
            .collect();
        Some(Self {
            peripheral,
            device_status: map.remove(&DEVICE_STATUS)?,
            friendly_name: map.remove(&FRIENDLY_NAME)?,
            wifi_ssid: map.remove(&WIFI_SSID)?,
            wifi_password: map.remove(&WIFI_PASSWORD)?,
            commands: map.remove(&COMMANDS)?,
            extended_data: map.remove(&EXTENDED_DATA)?,
        })
    }

    pub async fn get_friendly_name(&self) -> String {
        let data = self.peripheral.read(&self.friendly_name).await.unwrap();

        String::from_utf8(data).unwrap()
    }
}



#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize)]
pub enum OperatingMode {
    Standby = 0,
    NormalHeat = 1,
    TurboHeat = 2,
    ExtendedHeat = 3,
    Cool = 4,
    Dry = 5,
    Wait = 6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct DeviceStatus {
    /// The total runtime left on the device
    pub remaining_duration: Duration,
    /// Temp in degrees C
    pub actual_temp: u8,
    /// Temp in degrees C
    pub target_temp: u8,
    pub operating_mode: OperatingMode,
    // In percentages
    pub fan_step: u8,
    /// Maximum runtime for the current mode
    pub max_duration: Duration,
    pub min_target_temp: u8,
    pub max_target_temp: u8,
    pub ambient_temp: u8,
    pub shutdown_code: ShutDownCode,
    pub current_update_state: UpdateStatus,
}

impl DeviceStatus {
    fn from_packet(packet: &[u8]) -> Option<Self> {
        let remaining_hours = packet[3] as u64;
        let remaining_mins = packet[4] as u64;
        let remaining_secs = packet[5] as u64;
        let total_secs = (remaining_hours * 3600) + (remaining_mins * 60) + (remaining_secs);

        let remaining_duration = Duration::from_secs(total_secs);
        let operating_mode = OperatingMode::from_u8(packet[8])?;

        let max_hours = packet[10] as u64;
        let max_mins = packet[11] as u64;

        let max_secs = (max_hours * 3600) + (max_mins * 60);

        let max_duration = Duration::from_secs(max_secs);

        let shutdown_code = ShutDownCode::from_u8(packet[17])?;
        let current_update_state = UpdateStatus::from_u8(packet[25])?;

        Some(Self {
            remaining_duration,
            actual_temp: packet[6],
            target_temp: packet[7],
            operating_mode,
            fan_step: packet[9].saturating_add(1).saturating_mul(5),
            max_duration,
            min_target_temp: packet[12],
            max_target_temp: packet[13],
            ambient_temp: packet[16],
            shutdown_code,
            current_update_state,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize)]
pub enum ShutDownCode {
    Normal = 0,
    InvalidADC = 1,
    ThermistorTrackingError = 2,
    FastOverTempTrip = 3,
    SlowOverTempTrip = 4,
    FanFailure = 5,
    HeaterPowerStandby = 6,
    ExtenderThermalTrip = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize)]
pub enum UpdateStatus {
    Idle = 0,
    Starting = 1,
    ConnectingToAP = 2,
    GotIPAddress = 3,
    CheckingConnection = 4,
    CheckingForUpdate = 5,
    Updating = 6,
    RestartingBedJet = 7,
    NoWiFiConfig = 20,
    UnableToConnect = 21,
    DHCPFailure = 22,
    UnableToContactServer = 23,
    ConnectionTestOK = 24,
    ConnectionTestFailed = 25,
    NoUpdateNeeded = 26,
    RadioDisabled = 27,
    RestartingBedJetTerminal = 28,
    UpdateFailed = 29,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize)]
enum ButtonCode {
    Stop = 0x01,
    Cool = 0x02,
    Heat = 0x03,
    Turbo = 0x04,
    Dry = 0x05,
    ExternalHeat = 0x06,
    FanUp = 0x10,
    FanDown = 0x11,
    TempUp1C = 0x12,
    TempDown1C = 0x13,
    TempUp1F = 0x14,
    TempDown1F = 0x15,
    Memory1Recall = 0x20,
    Memory2Recall = 0x21,
    Memory3Recall = 0x22,
    Memory1Store = 0x28,
    Memory2Store = 0x29,
    Memory3Store = 0x2a,
    StartConnectionTest = 0x42,
    StartFirmwareUpdate = 0x43,
    SetLowPowerMode = 0x44,
    SetNormalPowerMode = 0x45,
    EnableRingOfLight = 0x46,
    DisableRingOfLight = 0x47,
    MuteBeeper = 0x48,
    UnmuteBeeper = 0x49,
    ResetToFactorySettings = 0x4c,
    EnableWiFiBT = 0x4d,
    DisableWiFiBT = 0x4e,
    SetConfigCompleteFlag = 0x4f,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
enum ParameterCode {
    DeviceName = 0x00,
    MemoryName1 = 0x01,
    MemoryName2 = 0x02,
    MemoryName3 = 0x03,
    BiorhythmName1 = 0x04,
    BiorhythmName2 = 0x05,
    BiorhythmName3 = 0x06,
    Biorhythm1Fragment1 = 0x07,
    Biorhythm1Fragment2 = 0x08,
    Biorhythm1Fragment3 = 0x09,
    Biorhythm1Fragment4 = 0x0a,
    Biorhythm2Fragment1 = 0x0b,
    Biorhythm2Fragment2 = 0x0c,
    Biorhythm2Fragment3 = 0x0d,
    Biorhythm2Fragment4 = 0x0e,
    Biorhythm3Fragment1 = 0x0f,
    Biorhythm3Fragment2 = 0x10,
    Biorhythm3Fragment3 = 0x11,
    Biorhythm3Fragment4 = 0x12,
    FirmwareVersionCodes = 0x20,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
enum CommandClass {
    Button = 0x01,
    SetTime = 0x02,
    SetTemp = 0x03,
    SetFan = 0x07,
    SetClock = 0x08,
    SetParameter = 0x40,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Command {
    command_class: CommandClass,
    data: Vec<u8>,
}

impl Command {
    fn new(command_class: CommandClass, data: Vec<u8>) -> Self {
        Command {
            command_class,
            data,
        }
    }
}
