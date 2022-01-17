use directories_next::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use structopt::StructOpt;

static CONFIG_FILE_PATH: &str = "Havoc/iscp";
static CONFIG_FILE_NAME: &str = "config.json";

#[derive(StructOpt)]
#[structopt(
    about = "Control your Onkyo receiver over the network",
    author = "Havoc (havoc.dev)"
)]
struct ProgramOptions {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, PartialEq)]
enum Command {
    #[structopt(about = "Discovers available devices in the network")]
    Discover,
    #[structopt(about = "Lists the discovered devices")]
    List,
    #[structopt(about = "Selects a device")]
    Select {
        #[structopt(help = "ID from device list")]
        id: usize,
    },
    #[structopt(about = "Powers on the selected device")]
    On,
    #[structopt(about = "Powers off the selected device")]
    Off,
    #[structopt(about = "Mutes the selected device")]
    Mute,
    #[structopt(about = "Unmutes the selected device")]
    Unmute,
    #[structopt(
        about = "Changes the volume level of the selected device",
        alias = "vol"
    )]
    Volume {
        #[structopt(help = "Volume level (Range from 0 to max)")]
        level: u8,
    },
    #[structopt(about = "Adjusts the speaker tone of the selected device")]
    Tone {
        #[structopt(help = "Speaker (Front)")]
        speaker: String,
        #[structopt(help = "Tone (Bass, Treble)")]
        setting: String,
        #[structopt(help = "Level Shift (Range from -10 to 10)")]
        level: i8,
    },
    #[structopt(about = "Sends a custom ISCP message")]
    Raw {
        #[structopt(help = "ISCP command")]
        command: String,
        #[structopt(help = "ISCP parameter")]
        parameter: String,
    },
}

fn main() {
    let options = ProgramOptions::from_args();
    let mut app = Application::load();

    match options.command {
        Command::Discover => app.discover(),
        Command::List => app.list(),
        Command::Select { id } => app.select(id),
        Command::On => match app.device().commands().main().power_on() {
            Ok(_) => println!("⚡"),
            Err(_) => println!("Failed to send command"),
        },
        Command::Off => match app.device().commands().main().power_off() {
            Ok(_) => println!("❌"),
            Err(_) => println!("Failed to send command"),
        },
        Command::Mute => match app.device().commands().main().mute() {
            Ok(_) => println!("🔈"),
            Err(_) => println!("Failed to send command"),
        },
        Command::Unmute => match app.device().commands().main().unmute() {
            Ok(_) => println!("🔊"),
            Err(_) => println!("Failed to send command"),
        },
        Command::Volume { level } => {
            let checked_level = if level > app.volume_max_level {
                app.volume_max_level
            } else {
                level
            };
            match app.device().commands().main().set_volume(checked_level) {
                Ok(_) => println!("🔉"),
                Err(_) => println!("Failed to send command"),
            }
        }
        Command::Tone {
            speaker,
            setting,
            level,
        } => match speaker.to_lowercase().as_str() {
            "front" => match setting.to_lowercase().as_str() {
                "bass" => match app.device().commands().main().set_tone_front_bass(level) {
                    Ok(_) => println!("Ok"),
                    Err(_) => println!("Failed to send command"),
                },
                "treble" => match app.device().commands().main().set_tone_front_treble(level) {
                    Ok(_) => println!("Ok"),
                    Err(_) => println!("Failed to send command"),
                },
                _ => println!("Invalid argument for tone"),
            },
            _ => println!("Invalid argument for speaker"),
        },
        Command::Raw { command, parameter } => {
            match app.device().raw(command.as_str(), parameter.as_str()) {
                Ok(_) => println!("📡"),
                Err(_) => println!("Failed to send command"),
            }
        }
    }

    app.save();
}

#[derive(Serialize, Deserialize)]
struct Application {
    remote: iscp::Remote,
    selected_device: usize,
    volume_max_level: u8,
}

impl Application {
    pub fn new() -> Application {
        Application {
            remote: iscp::Remote::new(),
            selected_device: 0,
            volume_max_level: 30,
        }
    }

    pub fn load() -> Application {
        let path = BaseDirs::new().unwrap().config_dir().join(CONFIG_FILE_PATH);
        let file = path.join(CONFIG_FILE_NAME);
        if let Ok(json) = fs::read_to_string(file) {
            if let Ok(app) = serde_json::from_str(json.as_str()) {
                return app;
            }
        }
        println!("No valid config file found");
        Application::new()
    }

    pub fn save(&self) {
        let path = BaseDirs::new().unwrap().config_dir().join(CONFIG_FILE_PATH);
        let file = path.join(CONFIG_FILE_NAME);
        match fs::create_dir_all(&path) {
            Ok(_) => match fs::write(&file, serde_json::to_string_pretty(self).unwrap()) {
                Ok(_) => {}
                Err(_) => println!("Failed to save config file"),
            },
            Err(_) => println!("Failed to create config dir"),
        }
    }

    pub fn discover(&mut self) {
        println!("Discovering devices...");
        let timeout = 5;
        println!("Receive timeout is set to {} seconds", timeout);
        self.remote.discover(Duration::from_secs(timeout));
        self.list();
    }

    pub fn list(&self) {
        println!("Available devices:");
        println!();
        for (index, device) in self.remote.devices.iter().enumerate() {
            println!("\t{}: {} at {}", index, device.model, device.address);
        }
        println!();
        println!("Selected device: {}", self.selected_device);
    }

    pub fn select(&mut self, index: usize) {
        if index >= self.remote.devices.len() {
            println!("There is no device with this index");
            return;
        }
        self.selected_device = index;
    }

    pub fn device(&self) -> &iscp::Device {
        self.remote
            .device(self.selected_device)
            .expect("Invalid selected device")
    }
}
