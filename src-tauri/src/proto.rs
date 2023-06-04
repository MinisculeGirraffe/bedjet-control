use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
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

#[typeshare]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceStatus {
    /// The total runtime left on the device
    pub remaining_hours: u8,
    pub remaining_minutes: u8,
    pub remaining_seconds: u8,
    /// Stored in units of 0.5 degrees celsius
    pub actual_temp: u8,
    /// Stored in units of 0.5 degrees celsius
    pub target_temp: u8,
    pub operating_mode: OperatingMode,
    /// Represented as a number between 0-19
    pub fan_step: u8,
    /// Maximum runtime for the current mode
    pub max_duration_hours: u8,
    pub max_duration_minutes: u8,
    /// Stored in units of 0.5 degrees celsius
    pub min_target_temp: u8,
    /// Stored in units of 0.5 degrees celsius
    pub max_target_temp: u8,
    /// Stored in units of 0.5 degrees celsius
    pub ambient_temp: u8,
    pub shutdown_code: ShutDownCode,
    pub update_status: UpdateStatus,
}


#[derive(Debug, Clone, PartialEq)]
#[typeshare]
pub struct DeviceStatusEvent {
    pub id: String,
    pub status: DeviceStatus,
}

#[typeshare]
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

#[typeshare]
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

#[typeshare]
#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize, Deserialize,
)]
pub enum ButtonCode {
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
pub enum ParameterCode {
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

#[typeshare]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum CommandClass {
    Button = 0x01,
    SetTime = 0x02,
    SetTemp = 0x03,
    SetFan = 0x07,
    SetClock = 0x08,
    SetParameter = 0x40,
}
