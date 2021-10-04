# OnkyoRemote
Control your Onkyo AV receiver over the network using the ISCP protocol.

The project consists of a library implementing the ISCP commands, a CLI app and an experimental REST interface.

Runs on Linux, macOS and Windows. Written in Rust.

## CLI App
### Usage
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

### Examples
```
$ onkyo on
⚡
```

```
$ onkyo vol 15
🔉
```
