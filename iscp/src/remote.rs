use super::{discover, Device};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;
use std::time::Duration;

static CONFIG_FILE_NAME: &str = "remote.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Remote {
    pub devices: Vec<Device>,
}

impl Remote {
    pub fn new() -> Remote {
        Remote {
            devices: Vec::new(),
        }
    }

    pub fn load() -> Remote {
        Remote::load_from(CONFIG_FILE_NAME)
    }

    pub fn load_from(file: &str) -> Remote {
        if let Ok(json) = fs::read_to_string(file) {
            Remote::deserialize(json.as_str())
        } else {
            Remote::new()
        }
    }

    pub fn deserialize(json: &str) -> Remote {
        serde_json::from_str(json).unwrap()
    }

    pub fn save(&self) -> Result<()> {
        self.save_to(CONFIG_FILE_NAME)
    }

    pub fn save_to(&self, file: &str) -> Result<()> {
        fs::write(file, self.serialize())
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn discover(&mut self, duration: Duration) {
        self.devices = discover(duration).unwrap();
    }

    pub fn raw(device_address: &str, iscp_command: &str, iscp_parameter: &str) -> Result<()> {
        let device = Device::from_address(device_address);
        device.raw(iscp_command, iscp_parameter)
    }

    pub fn device(&self, index: usize) -> Option<&Device> {
        self.devices.get(index)
    }
}
