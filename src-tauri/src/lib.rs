use proto::{ButtonCode, CommandClass, ParameterCode};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use thiserror::Error;
use typeshare::typeshare;
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
#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Invalid Data provided to protocol")]
    InvalidPameter,
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
/// A higher level enum containing the commands that can be sent to the device, and the parameters to those commands
pub enum Command {
    Button(ButtonCode),
    SetTime { hours: u8, minutes: u8 },
    SetTemp(Temp),
    SetFan(Fan),
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
pub enum Temp {
    /// The temperature in degrees Celsius
    Celsius(u8),
    /// The temperature in degrees Fahrenheit
    Fahrenheit(u8),
}

impl Encode for Temp {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        // The actual value we need to write is stored in units of 0.5 Celsius, so we multiply by 2
        // or convert to Celsius and multiply by 2
        let value = match self {
            Temp::Celsius(val) => val.saturating_mul(2),
            Temp::Fahrenheit(val) => val
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
pub enum Fan {
    Step(u8),
    Percent(u8),
}

impl Fan {
    fn validate(&self) -> Result<(), InterfaceError> {
        match self {
            Fan::Step(val) if *val > 19 => Err(InterfaceError::InvalidPameter),
            Fan::Percent(val) if *val > 100 => Err(InterfaceError::InvalidPameter),
            _ => Ok(()),
        }
    }
}

impl Encode for Fan {
    fn write_to<W: io::Write>(&self, writer: &mut W) -> Result<(), InterfaceError> {
        self.validate()?;

        let value = match self {
            Fan::Step(val) => *val,
            Fan::Percent(val) => val.saturating_div(5).saturating_sub(1),
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
                    return Err(InterfaceError::InvalidPameter);
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
