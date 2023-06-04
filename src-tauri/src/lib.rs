use num_traits::FromPrimitive;
use proto::{ButtonCode, CommandClass, DeviceStatus, ParameterCode};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read},
    time::Duration,
};
use thiserror::Error;
use typeshare::typeshare;

use crate::proto::{OperatingMode, ShutDownCode, UpdateStatus};
pub mod device;
pub mod proto;

pub trait Encode
where
    Self: Sized,
{
    fn encode(&self) -> Result<Vec<u8>, InterfaceError> {
        let mut bytes = Vec::new();
        self.write_to(&mut bytes)?;
        Ok(bytes)
    }

    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError>;
}

pub trait Decode: Sized {
    fn read_from<R: Read>(reader: R) -> Result<Self, InterfaceError>;
}

impl Decode for DeviceStatus {
    fn read_from<R: Read>(mut reader: R) -> Result<Self, InterfaceError> {
        let mut packet = [0u8; 27];
        let read_bytes = reader.read(&mut packet)?;

        if read_bytes as u8 != packet[2] - 3 {
            return Err(InterfaceError::NotEnoughData);
        }

        let operating_mode =
            OperatingMode::from_u8(packet[8]).ok_or_else(|| InterfaceError::InvalidParameter)?;
        let shutdown_code =
            ShutDownCode::from_u8(packet[17]).ok_or_else(|| InterfaceError::InvalidParameter)?;
        let update_status =
            UpdateStatus::from_u8(packet[25]).ok_or_else(|| InterfaceError::InvalidParameter)?;

        Ok(Self {
            remaining_hours: packet[3],
            remaining_minutes: packet[4],
            remaining_seconds: packet[5],
            actual_temp: packet[6],
            target_temp: packet[7],
            operating_mode,
            fan_step: packet[9],
            max_duration_hours: packet[10],
            max_duration_minutes: packet[11],
            min_target_temp: packet[12],
            max_target_temp: packet[13],
            ambient_temp: packet[16],
            shutdown_code,
            update_status,
        })
    }
}

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Invalid Data provided to protocol")]
    InvalidParameter,
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),
    #[error("Invalid Data provided to protocol")]
    NotEnoughData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
/// A higher level enum containing the commands that can be sent to the device, and the parameters to those commands
pub enum Command {
    Button(ButtonCode),
    SetTime { hours: u8, minutes: u8 },
    SetTemp(TempParam),
    SetFan(FanParam),
    SetClock { hours: u8, minutes: u8 },
    SetParam(SetParamKind),
}

impl Encode for Command {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        match self {
            Command::Button(code) => {
                writer.write_all(&[CommandClass::Button as u8, *code as u8])?
            }
            Command::SetTime { hours, minutes } => {
                writer.write_all(&[CommandClass::SetTime as u8, *hours, *minutes])?;
            }
            Command::SetTemp(temp) => {
                writer.write_all(&[CommandClass::SetTemp as u8])?;
                temp.write_to(writer)?
            }
            Command::SetFan(fan) => {
                writer.write_all(&[CommandClass::SetFan as u8])?;
                fan.write_to(writer)?
            }
            Command::SetClock { hours, minutes } => {
                writer.write_all(&[CommandClass::SetClock as u8, *hours, *minutes])?
            }
            Command::SetParam(param) => {
                writer.write_all(&[CommandClass::SetParameter as u8])?;
                param.write_to(writer)?
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[typeshare]
pub enum TempParam {
    /// The temperature in degrees Celsius
    Celsius(u8),
    /// The temperature in degrees Fahrenheit
    Fahrenheit(u8),
}

impl Encode for TempParam {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        // The actual value we need to write is stored in units of 0.5 Celsius, so we multiply by 2
        // or convert to Celsius and multiply by 2
        let value = match self {
            TempParam::Celsius(val) => val.saturating_mul(2),
            TempParam::Fahrenheit(val) => val
                .saturating_sub(32)
                .saturating_mul(5)
                .saturating_div(9)
                .saturating_mul(2),
        };
        writer.write_all(&[value])?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[typeshare]
pub enum FanParam {
    Step(u8),
    Percent(u8),
}

impl FanParam {
    fn validate(&self) -> Result<(), InterfaceError> {
        match self {
            FanParam::Step(val) if *val > 19 => Err(InterfaceError::InvalidParameter),
            FanParam::Percent(val) if *val > 100 => Err(InterfaceError::InvalidParameter),
            _ => Ok(()),
        }
    }
}

impl Encode for FanParam {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        self.validate()?;

        let value = match self {
            FanParam::Step(val) => *val,
            FanParam::Percent(val) => val.saturating_div(5).saturating_sub(1),
        };

        writer.write_all(&[value])?;

        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetParamKind {
    /// Cannot contain a String longer than 15 bytes.
    DeviceName(String),
}

impl Encode for SetParamKind {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        match self {
            SetParamKind::DeviceName(name) => {
                // Validate that the string is within the allowed limit
                if name.len() > 15 {
                    return Err(InterfaceError::InvalidParameter);
                }
                // Write the header data
                writer.write_all(&[ParameterCode::DeviceName as u8, 0x10])?;
                // And then write the string
                writer.write_all(name.as_bytes())?;

                // Calculate the number of bytes to zero pad with
                let padding = 16 - name.len();

                // And write those bytes out
                io::copy(&mut io::repeat(0).take(padding as u64), writer)?;
            }
        }
        Ok(())
    }
}

pub struct ParsedDeviceStatus {
    remaining_duration: Duration,
    /// As degrees C
    actual_temp: f32,
    /// As degrees C
    target_temp: f32,
    operating_mode: OperatingMode,
    ///As a percent 0 - 100
    fan_step: u8,
    max_duration: Duration,
    min_target_temp: f32,
    max_target_temp: f32,
    ambient_temp: f32,
    shutdown_code: ShutDownCode,
    update_status: UpdateStatus,
}

impl From<DeviceStatus> for ParsedDeviceStatus {
    fn from(value: DeviceStatus) -> Self {
        let remaining_duration = Duration::from_secs(
            (value.remaining_hours as u64 * 3600)
                + (value.remaining_minutes as u64 * 60)
                + (value.remaining_seconds as u64),
        );

        let max_duration = Duration::from_secs(
            (value.max_duration_hours as u64 * 3600) + value.max_duration_minutes as u64 * 60,
        );
        Self {
            remaining_duration,
            actual_temp: value.actual_temp as f32 / 2.0,
            target_temp: value.target_temp as f32 / 2.0,
            operating_mode: value.operating_mode,
            fan_step: value.fan_step.saturating_add(1).saturating_mul(5),
            max_duration,
            min_target_temp: value.min_target_temp as f32 / 2.0,
            max_target_temp: value.max_target_temp as f32 / 2.0,
            ambient_temp: value.ambient_temp as f32 / 2.0,
            shutdown_code: value.shutdown_code,
            update_status: value.update_status,
        }
    }
}
