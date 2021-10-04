mod main;

use super::Device;

pub use main::MainCommands;

pub const VOLUME_MAX_LEVEL: u8 = 100;
pub const TONE_MAX_LEVEL: i8 = 10;
pub const TONE_MIN_LEVEL: i8 = -10;

pub struct Commands<'a> {
    main: MainCommands<'a>,
}

impl<'a> Commands<'a> {
    pub fn from(device: &Device) -> Commands {
        Commands {
            main: MainCommands::from(device),
        }
    }

    pub fn main(&self) -> &MainCommands {
        &self.main
    }
}

fn to_signed_hex_str(v: i8, max: i8, min: i8) -> String {
    let mut value = if v > max {
        max
    } else if v < min {
        min
    } else {
        v
    };
    let sign = if value < 0 {
        value *= -1;
        "-"
    } else {
        "+"
    };
    format!("{}{:X}", sign, value)
}
