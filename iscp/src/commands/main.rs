use super::{to_signed_hex_str, TONE_MAX_LEVEL, TONE_MIN_LEVEL, VOLUME_MAX_LEVEL};
use crate::Device;
use std::io::Result;

pub struct MainCommands<'b> {
    device: &'b Device,
}

impl<'b> MainCommands<'b> {
    pub fn from(device: &Device) -> MainCommands {
        MainCommands { device }
    }

    pub fn power_off(&self) -> Result<()> {
        self.device.raw("PWR", "00")
    }

    pub fn power_on(&self) -> Result<()> {
        self.device.raw("PWR", "01")
    }

    pub fn unmute(&self) -> Result<()> {
        self.device.raw("AMT", "00")
    }

    pub fn mute(&self) -> Result<()> {
        self.device.raw("AMT", "01")
    }

    pub fn set_volume(&self, mut level: u8) -> Result<()> {
        if level > VOLUME_MAX_LEVEL {
            level = VOLUME_MAX_LEVEL;
        }
        self.device.raw("MVL", format!("{:02X}", level).as_str())
    }

    pub fn set_volume_up(&self) -> Result<()> {
        self.device.raw("MVL", "UP")
    }

    pub fn set_volume_down(&self) -> Result<()> {
        self.device.raw("MVL", "DOWN")
    }

    pub fn set_tone_front_bass(&self, level: i8) -> Result<()> {
        self.device.raw(
            "TFR",
            format!(
                "B{}",
                to_signed_hex_str(level, TONE_MAX_LEVEL, TONE_MIN_LEVEL)
            )
            .as_str(),
        )
    }

    pub fn set_tone_front_treble(&self, level: i8) -> Result<()> {
        self.device.raw(
            "TFR",
            format!(
                "T{}",
                to_signed_hex_str(level, TONE_MAX_LEVEL, TONE_MIN_LEVEL)
            )
            .as_str(),
        )
    }

    pub fn set_tone_front_bass_up(&self) -> Result<()> {
        self.device.raw("TFR", "BUP")
    }

    pub fn set_tone_front_bass_down(&self) -> Result<()> {
        self.device.raw("TFR", "BDOWN")
    }

    pub fn set_tone_front_treble_up(&self) -> Result<()> {
        self.device.raw("TFR", "TUP")
    }

    pub fn set_tone_front_treble_down(&self) -> Result<()> {
        self.device.raw("TFR", "TDOWN")
    }
}
