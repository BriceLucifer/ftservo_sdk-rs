use crate::{
    port_handler::PortHandler,
    scservo_def::{BROADCAST_ID, COMM, INST},
};
use std::io::ErrorKind;

const TXPACKET_MAX_LEN: usize = 250;
const RXPACKET_MAX_LEN: usize = 250;

// for Protocol Packet
const HEADER0: usize = 0;
const HEADER1: usize = 1;
const ID: usize = 2;
const LENGTH: usize = 3;
const INSTRUCTION: usize = 4;
const ERROR: usize = 4;
const PARAMETER0: usize = 5;

// Protocal Error bit
// const ERRBIT_VOLTAGE: u8 = 1;
// const ERRBIT_ANGLE: u8 = 2;
// const ERRBIT_OVERHEAT: u8 = 4;
// const ERRBIT_OVERELE: u8 = 8;
// const ERRBIT_OVERLOAD: u8 = 32;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ErrorCode {
    VoltageError = 1,
    AngleError = 2,
    OverheatError = 4,
    OverElementError = 8,
    OverloadError = 32,
    Success = 0,
}

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
            scs_end,
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

    pub fn scs_lobyte(&self, w: i32) -> i32 {
        match self.scs_end {
            Endian::SmallEndian => w & 0xFF,
            Endian::BigEndian => (w >> 8) & 0xFF,
        }
    }

    pub fn scs_hibyte(&self, w: i32) -> i32 {
        match self.scs_end {
            Endian::SmallEndian => (w >> 8) & 0xFF,
            Endian::BigEndian => w & 0xFF,
        }
    }

    pub fn get_protocol_version(&self) -> String {
        "1.0".to_string()
    }

    pub fn get_tx_rx_result(&self, result: COMM) -> String {
        match result {
            COMM::Success => "[TxRxResult] Communication success!".to_string(),
            COMM::PortBusy => "[TxRxResult] port is in use!".to_string(),
            COMM::TxFail => "[TxRxResult] Failed to transmit instruction packet!".to_string(),
            COMM::RxFail => "[TxRxResult] Failed to receive instruction packet!".to_string(),
            COMM::TxError => "[TxRxResult] Incorrect instruction packet!".to_string(),
            COMM::RxWaiting => "[TxRxResult] Now receiving status packet!".to_string(),
            COMM::RxTimeout => "[TxRxResult] There is no status packet!".to_string(),
            COMM::RxCorrupt => "[TxRxResult] Received packet is corrupted!".to_string(),
            COMM::NotAvailable => "[TxRxResult] Feature not available!".to_string(),
        }
    }
    pub fn get_rx_packet_error(&self, error: ErrorCode) -> Result<(), serialport::Error> {
        match error {
            ErrorCode::VoltageError => Err(serialport::Error::new(
                serialport::ErrorKind::InvalidInput,
                "[ServoStatus] Input voltage error!",
            )),
            ErrorCode::AngleError => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::InvalidData),
                "[ServoStatus] Angle error!",
            )),
            ErrorCode::OverheatError => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::InvalidData),
                "[ServoStatus] Overheat error!",
            )),
            ErrorCode::OverElementError => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::InvalidData),
                "[ServoStatus] Over element error!",
            )),
            ErrorCode::OverloadError => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::InvalidData),
                "[ServoStatus] Overload error!",
            )),
            ErrorCode::Success => Ok(()),
        }
    }

    pub fn tx_packet(&mut self, tx_packet: &mut Vec<u32>) -> COMM {
        let mut checksum = 0;
        let total_packet_length = tx_packet[LENGTH] + 4;

        if self.port_handler.is_using {
            return COMM::PortBusy;
        }

        self.port_handler.is_using = true;
        if total_packet_length as usize > TXPACKET_MAX_LEN {
            self.port_handler.is_using = false;
            return COMM::TxError;
        }

        tx_packet[HEADER0] = 0xff;
        tx_packet[HEADER1] = 0xff;

        for idx in 2..(total_packet_length - 1) as usize {
            checksum += tx_packet[idx];
        }
        tx_packet[total_packet_length as usize - 1] = !checksum & 0xff;

        match self.port_handler.clear_port() {
            Ok(_) => {}
            Err(e) => eprintln!("Error clear the port {}", e),
        }

        // write port 参数有问题
        let written_packet_length = self.port_handler.write_port(&[0; 12]).unwrap();
        if total_packet_length as usize != written_packet_length {
            return COMM::TxFail;
        }

        return COMM::Success;
    }
    pub fn rx_packet(&self) {}
    pub fn tx_rx_packet(&self) -> (Vec<u32>, COMM) {
        (vec![], COMM::Success)
    }
    pub fn ping(&self, scs_id: u32) {}
    pub fn action(&self) {}
    pub fn read_tx(&self) {}
    pub fn read_rx(&self) {}
    pub fn read_tx_rx(&self) {}
    pub fn read_1byte_tx(&self) {}
    pub fn read_1byte_rx(&self) {}
    pub fn read_1byte_tx_rx(&self) {}
    pub fn read_2byte_tx(&self) {}
    pub fn read_2byte_rx(&self) {}
    pub fn read_2byte_tx_rx(&self) {}
    pub fn read_4byte_tx(&self) {}
    pub fn read_4byte_rx(&self) {}
    pub fn read_4byte_tx_rx(&self) {}
    pub fn write_tx_only(&self) {}
    pub fn write_tx_rx(&self) {}
    pub fn write_1byte_tx_only(&self) {}
    pub fn write_1byte_tx_rx(&self) {}
    pub fn write_2byte_tx_only(&self) {}
    pub fn write_2byte_tx_rx(&self) {}
    pub fn write_4byte_tx_only(&self) {}
    pub fn write_4byte_tx_rx(&self) {}
    pub fn reg_write_tx_only(&self) {}
    pub fn reg_write_tx_rx(&self) {}
    pub fn sync_read_tx(&self) -> COMM {
        COMM::Success
    }
    pub fn sync_read_rx(&self) -> (COMM, Vec<u32>) {
        return (COMM::Success, Vec::new());
    }
    pub fn sync_write_tx_only(
        &self,
        start_addree: u32,
        data_length: u32,
        param: Vec<u32>,
        param_length: u32,
    ) -> COMM {
        let mut txpacket = Vec::with_capacity(param_length as usize + 8);

        txpacket[ID] = BROADCAST_ID as u32;
        txpacket[LENGTH] = param_length + 8;
        txpacket[INSTRUCTION] = INST::SyncRead as u32;
        txpacket[PARAMETER0 + 0] = start_addree;
        txpacket[PARAMETER0 + 1] = data_length;

        txpacket[(PARAMETER0 + 2)..(PARAMETER0 + 2 + param_length as usize)]
            .copy_from_slice(&param[0..param_length as usize]);

        let (_, result) = self.tx_rx_packet();
        return result;
    }
}
