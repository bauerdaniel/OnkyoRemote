use super::commands::Commands;
use super::IscpMessage;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;

pub const DEVICE_MAC_MAX_LEN: usize = 12;

pub static DEVICE_AREA_EUROEAN_ASIA: &str = "XX";
pub static DEVICE_AREA_NORTH_AMERICA: &str = "DX";
pub static DEVICE_AREA_JAPANESE: &str = "JJ";

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub address: String,
    pub model: String,
    pub area: String,
    pub mac: String,
    #[serde(skip)]
    connection: Option<TcpStream>,
}

impl Device {
    pub fn new() -> Device {
        Device {
            address: String::new(),
            model: String::new(),
            area: String::new(),
            mac: String::new(),
            connection: None,
        }
    }

    pub fn from(address: String, model: String, area: String, mac: String) -> Device {
        Device {
            address,
            model,
            area,
            mac,
            connection: None,
        }
    }

    pub fn from_address(address: &str) -> Device {
        Device {
            address: String::from(address),
            model: String::new(),
            area: String::new(),
            mac: String::new(),
            connection: None,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.connection = Some(TcpStream::connect(self.address.as_str())?);
        Ok(())
    }

    pub fn send(&self, message: IscpMessage) -> Result<()> {
        let mut stream = TcpStream::connect(self.address.as_str())?;
        stream.write(message.bytes().as_ref())?;
        Ok(())
    }

    pub fn receive(&self) {}

    pub fn raw(&self, iscp_command: &str, iscp_parameter: &str) -> Result<()> {
        self.send(IscpMessage::from(iscp_command, iscp_parameter))
    }

    pub fn commands(&self) -> Commands {
        Commands::from(self)
    }
}
