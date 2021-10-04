use bytes::{Buf, BufMut, Bytes, BytesMut};

pub const ISCP_MSG_DST_RECEIVER: u8 = b'1';
pub const ISCP_MSG_DST_BROADCAST: u8 = b'x';

const ISCP_VERSION: u8 = 0x01;
const ISCP_MSG_START: u8 = b'!';
const ISCP_MSG_END_LF: u8 = 0x0A;
const ISCP_MSG_HEADER_LEN: u32 = 16;
const ISCP_MSG_CMD_LEN: usize = 3;
const ISCP_MSG_MIN_LEN: usize = ISCP_MSG_HEADER_LEN as usize + 2 + ISCP_MSG_CMD_LEN + 1;

static ISCP_MSG_HEADER_MAGIC: &[u8] = b"ISCP";
static ISCP_MSG_HEADER_RESERVED: &[u8] = &[0x00; 3];

#[derive(Debug)]
pub struct IscpMessage {
    pub destination: u8,
    pub command: String,
    pub parameter: String,
}

impl IscpMessage {
    pub fn new() -> IscpMessage {
        IscpMessage {
            destination: ISCP_MSG_DST_RECEIVER,
            command: String::new(),
            parameter: String::new(),
        }
    }

    pub fn from(command: &str, parameter: &str) -> IscpMessage {
        IscpMessage {
            destination: ISCP_MSG_DST_RECEIVER,
            command: String::from(command),
            parameter: String::from(parameter),
        }
    }

    pub fn from_slice(data: &[u8]) -> Option<IscpMessage> {
        let mut packet = Bytes::copy_from_slice(data);
        IscpMessage::from_bytes(&mut packet)
    }

    pub fn from_bytes(packet: &mut Bytes) -> Option<IscpMessage> {
        if packet.len() < ISCP_MSG_MIN_LEN {
            return None;
        }

        let len_magic = ISCP_MSG_HEADER_MAGIC.len();
        if packet.get(..len_magic)? != ISCP_MSG_HEADER_MAGIC {
            return None;
        }
        packet.advance(len_magic);
        let _len_header = packet.get_u32();
        let _len_payload = packet.get_u32();
        let _version = packet.get_u8();
        packet.advance(ISCP_MSG_HEADER_RESERVED.len());
        if packet.get_u8() != ISCP_MSG_START {
            return None;
        }
        let destination = packet.get_u8();
        let command = String::from_utf8(packet.get(..ISCP_MSG_CMD_LEN).unwrap().to_vec()).unwrap();
        packet.advance(ISCP_MSG_CMD_LEN);
        let parameter = String::from_utf8(packet.get(..).unwrap().to_vec()).unwrap();

        Some(IscpMessage {
            destination,
            command,
            parameter,
        })
    }

    pub fn bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(self.len_bytes());
        bytes.put_slice(ISCP_MSG_HEADER_MAGIC);
        bytes.put_u32(ISCP_MSG_HEADER_LEN);
        bytes.put_u32(self.len_payload() as u32);
        bytes.put_u8(ISCP_VERSION);
        bytes.put_slice(ISCP_MSG_HEADER_RESERVED);
        bytes.put_u8(ISCP_MSG_START);
        bytes.put_u8(self.destination);
        bytes.put_slice(self.command.get(..ISCP_MSG_CMD_LEN).unwrap().as_bytes());
        bytes.put_slice(self.parameter.as_bytes());
        bytes.put_u8(ISCP_MSG_END_LF);
        bytes
    }

    pub fn len_payload(&self) -> usize {
        2 + ISCP_MSG_CMD_LEN + self.parameter.len() + 1
    }

    pub fn len_bytes(&self) -> usize {
        ISCP_MSG_HEADER_LEN as usize + self.len_payload()
    }
}
