use crate::{port_handler::PortHandler, scservo_def::COMM};

const TXPACKET_MAX_LEN: usize = 250;
const RXPACKET_MAX_LEN: usize = 250;

// for Protocol Packet
const HEADER0: u8 = 0;
const HEADER1: u8 = 1;
const ID: u8 = 2;
const LENGTH: u8 = 3;
const INSTRUCTION: u8 = 4;
const ERROR: u8 = 4;
const PARAMETER0: u8 = 5;

// Protocal Error bit
const ERRBIT_VOLTAGE: u8 = 1;
const ERRBIT_ANGLE: u8 = 2;
const ERRBIT_OVERHEAT: u8 = 4;
const ERRBIT_OVERELE: u8 = 8;
const ERRBIT_OVERLOAD: u8 = 32;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Endian {
    BigEndian,
    SmallEndian,
}

impl std::fmt::Display for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endian::BigEndian => write!(f, "BigEndian"),
            Endian::SmallEndian => write!(f, "SmallEndian"),
        }
    }
}

#[derive(Debug)]
pub struct ProtocolPacketHandler {
    port_handler: PortHandler,
    scs_end: Endian,
}

impl ProtocolPacketHandler {
    // structor
    pub fn new(port_handler: PortHandler, scs_end: Endian) -> Self {
        Self {
            port_handler: PortHandler::new(&port_handler.get_port_name()),
            scs_end: scs_end,
        }
    }

    pub fn scs_getend(&self) -> Endian {
        return self.scs_end.clone();
    }

    pub fn scs_setend(&mut self, end: Endian) {
        self.scs_end = end;
    }

    pub fn scs_tohost(&self, a: i32, b: i32) -> i32 {
        if (a & (1 << b)) > 0 {
            return -(a & !(1 << b));
        } else {
            return a;
        }
    }

    pub fn scs_toscs(&self, a: i32, b: i32) -> i32 {
        if a < 0 {
            return -a | (1 << b);
        } else {
            return a;
        }
    }

    pub fn scs_makeword(&self, a: i32, b: i32) -> i32 {
        match self.scs_end {
            Endian::SmallEndian => (a & 0xFF) | ((b & 0xFF) << 8),
            Endian::BigEndian => (b & 0xFF) | ((a & 0xFF) << 8),
        }
    }

    pub fn scs_makedword(&self, a: i32, b: i32) -> i32 {
        (a & 0xFFFF) | (b & 0xFFFF) << 16
    }

    pub fn scs_loword(&self, l: i32) -> i32 {
        l & 0xFFFF
    }

    pub fn scs_hiword(&self, h: i32) -> i32 {
        (h >> 16) & 0xFFFF
    }

    /*
        functions..... to many
    */

    pub fn get_protocol_version(&self) -> String {
        "1.0".to_string()
    }

    pub fn get_tx_rx_result(&self, result: COMM) -> String {
        match result {
            COMM::Success => "[TxRxResult] Communication success!".to_string(),
            _ => "".to_string(),
        }
    }
}
