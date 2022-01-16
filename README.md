# OnkyoRemote

Control your Onkyo AV receiver over the network using the ISCP protocol.

The project consists of a library that implements the ISCP commands, a CLI app, and an experimental REST interface.

Runs on Linux, macOS and Windows. Written in Rust.


## Installation


### Prerequisites

- [Git](https://git-scm.com "Git")
- [Rust](https://www.rust-lang.org/tools/install "Rust") (2018 edition)


### Build & Install

1. Clone project repository  
`git clone https://github.com/bauerdaniel/OnkyoRemote.git`
2. Change to the source directory  
`cd OnkyoRemote/onkyo`
3. Compile the OnkyoRemote app using Cargo  
`cargo build --release`
4. Copy the built binary to any folder, e.g. your home directory  
`mkdir ~/OnkyoRemote`  
`cp target/release/onkyo ~/OnkyoRemote/`
5. Add the application folder to the PATH environment variable  
`export PATH=~/OnkyoRemote:$PATH`


### Setup

Note: The first time you start up, a configuration file is created to store the device information.

1. Discover available devices on the network  
`onkyo discover`
2. List available devices  
`onkyo list`
3. Select a device to control  
`onkyo select <id>`


## Usage

```
USAGE:
    onkyo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    discover    Discovers available devices in the network
    help        Prints this message or the help of the given subcommand(s)
    list        Lists the discovered devices
    mute        Mutes the selected device
    off         Powers off the selected device
    on          Powers on the selected device
    raw         Sends a custom ISCP message
    select      Selects a device
    tone        Adjusts the speaker tone of the selected device
    unmute      Unmutes the selected device
    volume      Changes the volume level of the selected device
```

#### Examples

```
$ onkyo on
⚡
```

```
$ onkyo vol 15
🔉
```
