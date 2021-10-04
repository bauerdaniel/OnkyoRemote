use std::error::Error;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

use super::{Device, IscpMessage, DEVICE_MAC_MAX_LEN, ISCP_MSG_DST_BROADCAST};

const DISCOVER_PARAMS_SEPARATOR: char = '/';

static DISCOVER_IP: &str = "0.0.0.0";
static DISCOVER_PORT: &str = "60128";

pub fn discover(duration: Duration) -> Result<Vec<Device>, Box<dyn Error>> {
    let addr = format!("{}:{}", DISCOVER_IP, DISCOVER_PORT);
    let socket = UdpSocket::bind(addr)?;
    socket.set_read_timeout(Some(duration))?;
    socket.set_broadcast(true)?;

    let discover_messsage = get_discover_message();
    let discover_messsage_bytes = discover_messsage.bytes();
    let broadcast_addrs = get_broadcast_addresses();
    for broadcast_ip in broadcast_addrs {
        let broadcast_addr = format!("{}:{}", broadcast_ip, DISCOVER_PORT);
        println!("Sending broadcast message to: {}", broadcast_addr);
        socket.send_to(discover_messsage_bytes.as_ref(), broadcast_addr)?;
    }

    let mut devices = Vec::new();
    let mut buf = [0; 1024];
    while let Ok((n, addr)) = socket.recv_from(&mut buf) {
        if n == discover_messsage.len_bytes() {
            continue;
        }

        if let Some(msg) = IscpMessage::from_slice(&buf[..n]) {
            let params: Vec<&str> = msg.parameter.split(DISCOVER_PARAMS_SEPARATOR).collect();
            let model = String::from(*params.get(0).unwrap());
            let port = String::from(*params.get(1).unwrap());
            let area = String::from(*params.get(2).unwrap());
            let mac = String::from(*params.get(3).unwrap());
            let len_mac = if mac.len() < DEVICE_MAC_MAX_LEN {
                mac.len()
            } else {
                DEVICE_MAC_MAX_LEN
            };
            let mac = String::from(mac.get(..len_mac).unwrap());
            let device_addr = format!("{}:{}", addr.ip(), port);
            devices.push(Device::from(device_addr, model, area, mac));
        }
    }
    Ok(devices)
}

fn get_discover_message() -> IscpMessage {
    IscpMessage {
        destination: ISCP_MSG_DST_BROADCAST,
        command: String::from("ECN"),
        parameter: String::from("QSTN"),
    }
}

fn get_broadcast_addresses() -> Vec<Ipv4Addr> {
    let mut broadcast_addresses = Vec::new();
    for iface in if_addrs::get_if_addrs().unwrap() {
        match iface.addr {
            if_addrs::IfAddr::V4(ref ifv4_addr) => match ifv4_addr.broadcast {
                Some(addr) => broadcast_addresses.push(addr),
                _ => {}
            },
            _ => {}
        }
    }
    broadcast_addresses
}
