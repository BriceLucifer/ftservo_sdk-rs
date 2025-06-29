use crate::{
    port_handler::PortHandler,
    scservo_def::{BROADCAST_ID, COMM, INST},
};
use std::io::ErrorKind;

const TXPACKET_MAX_LEN: usize = 250;
const RXPACKET_MAX_LEN: usize = 250;

// 协议包常量
const HEADER0: usize = 0;
const HEADER1: usize = 1;
const ID: usize = 2;
const LENGTH: usize = 3;
const INSTRUCTION: usize = 4;
const ERROR: usize = 4;
const PARAMETER0: usize = 5;

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
    pub port_handler: PortHandler,
    scs_end: Endian,
}

// 在现有代码基础上添加缺失的功能

impl ProtocolPacketHandler {
    pub fn new(port_handler: PortHandler, scs_end: Endian) -> Self {
        Self {
            port_handler: PortHandler::new(&port_handler.get_port_name()),
            scs_end,
        }
    }

    pub fn scs_getend(&self) -> Endian {
        self.scs_end.clone()
    }

    pub fn scs_setend(&mut self, end: Endian) {
        self.scs_end = end;
    }

    pub fn scs_tohost(&self, a: i32, b: i32) -> i32 {
        if (a & (1 << b)) > 0 {
            -(a & !(1 << b))
        } else {
            a
        }
    }

    pub fn scs_toscs(&self, a: i32, b: i32) -> i32 {
        if a < 0 {
            -a | (1 << b)
        } else {
            a
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

    fn calculate_checksum(&self, packet: &[u8]) -> u8 {
        let mut checksum = 0u8;
        for &byte in &packet[2..] {
            checksum = checksum.wrapping_add(byte);
        }
        !checksum
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

        // 修复：将u32数组转换为u8数组进行发送
        let tx_data: Vec<u8> = tx_packet[0..total_packet_length as usize]
            .iter()
            .map(|&x| x as u8)
            .collect();

        self.port_handler.set_packet_timeout(total_packet_length);

        match self.port_handler.write_port(&tx_data) {
            Ok(written) => {
                if written != total_packet_length as usize {
                    self.port_handler.is_using = false;
                    return COMM::TxFail;
                }
            }
            Err(_) => {
                self.port_handler.is_using = false;
                return COMM::TxFail;
            }
        }

        self.port_handler.is_using = false;
        COMM::Success
    }

    pub fn rx_packet(&mut self) -> (Vec<u32>, COMM) {
        let mut rx_packet = vec![0u32; RXPACKET_MAX_LEN];
        let mut rx_length = 0;
        let mut wait_length = 6; // 最小包长度

        loop {
            let mut buffer = [0u8; 1];
            match self.port_handler.read_port(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        rx_packet[rx_length] = buffer[0] as u32;
                        rx_length += 1;

                        // 检查包头
                        if rx_length >= 4 {
                            if rx_packet[0] == 0xFF && rx_packet[1] == 0xFF {
                                wait_length = rx_packet[3] as usize + 4; // 长度字段 + 包头和校验和
                            }
                        }

                        // 检查是否接收完整包
                        if rx_length >= wait_length && wait_length > 6 {
                            break;
                        }

                        if rx_length >= RXPACKET_MAX_LEN {
                            return (vec![], COMM::RxCorrupt);
                        }
                    }
                }
                Err(_) => {
                    if self.port_handler.is_packet_timeout() {
                        return (vec![], COMM::RxTimeout);
                    }
                }
            }
        }

        // 验证校验和
        if rx_length >= 4 {
            let mut checksum = 0u32;
            for i in 2..rx_length - 1 {
                checksum += rx_packet[i];
            }
            checksum = (!checksum) & 0xFF;

            if checksum != rx_packet[rx_length - 1] {
                return (vec![], COMM::RxCorrupt);
            }
        }

        (rx_packet[0..rx_length].to_vec(), COMM::Success)
    }

    pub fn tx_rx_packet(&mut self, tx_packet: &mut Vec<u32>) -> (Vec<u32>, COMM) {
        let tx_result = self.tx_packet(tx_packet);
        if tx_result != COMM::Success {
            return (vec![], tx_result);
        }

        self.rx_packet()
    }

    pub fn ping(&mut self, scs_id: u32) -> COMM {
        let mut tx_packet = vec![0u32; 6];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 2;
        tx_packet[INSTRUCTION] = INST::Ping as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    pub fn action(&mut self, scs_id: u32) -> COMM {
        let mut tx_packet = vec![0u32; 6];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 2;
        tx_packet[INSTRUCTION] = INST::Action as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    pub fn read_1byte_tx_rx(&mut self, scs_id: u32, address: u32) -> (Vec<u32>, COMM) {
        let mut tx_packet = vec![0u32; 8];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 4;
        tx_packet[INSTRUCTION] = INST::Read as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = 1; // 读取1字节
        
        self.tx_rx_packet(&mut tx_packet)
    }

    pub fn read_2byte_tx_rx(&mut self, scs_id: u32, address: u32) -> (Vec<u32>, COMM) {
        let mut tx_packet = vec![0u32; 8];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 4;
        tx_packet[INSTRUCTION] = INST::Read as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = 2; // 读取2字节
        
        self.tx_rx_packet(&mut tx_packet)
    }

    pub fn write_1byte_tx_rx(&mut self, scs_id: u32, address: u32, data: u8) -> COMM {
        let mut tx_packet = vec![0u32; 8];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 4;
        tx_packet[INSTRUCTION] = INST::Write as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = data as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    pub fn write_2byte_tx_rx(&mut self, scs_id: u32, address: u32, data: u16) -> COMM {
        let mut tx_packet = vec![0u32; 9];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 5;
        tx_packet[INSTRUCTION] = INST::Write as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = self.scs_lobyte(data as i32) as u32;
        tx_packet[PARAMETER0 + 2] = self.scs_hibyte(data as i32) as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    pub fn sync_write_tx_only(&mut self, start_address: u32, data_length: u32, param: Vec<u32>, param_length: u32) -> COMM {
        let mut tx_packet = vec![0u32; param_length as usize + 8];
        tx_packet[ID] = BROADCAST_ID as u32;
        tx_packet[LENGTH] = param_length + 4;
        tx_packet[INSTRUCTION] = INST::SyncWrite as u32;
        tx_packet[PARAMETER0] = start_address;
        tx_packet[PARAMETER0 + 1] = data_length;
        
        for (i, &value) in param.iter().enumerate() {
            tx_packet[PARAMETER0 + 2 + i] = value;
        }
        
        self.tx_packet(&mut tx_packet)
    }

    // 修复 sync_read_tx 实现
    pub fn sync_read_tx(&mut self, start_address: u32, data_length: u32, param: Vec<u32>) -> COMM {
        let param_length = param.len() as u32;
        let mut tx_packet = vec![0u32; param_length as usize + 8];
        
        tx_packet[ID] = BROADCAST_ID as u32;
        tx_packet[LENGTH] = param_length + 4;
        tx_packet[INSTRUCTION] = INST::SyncRead as u32;
        tx_packet[PARAMETER0] = start_address;
        tx_packet[PARAMETER0 + 1] = data_length;
        
        for (i, &value) in param.iter().enumerate() {
            tx_packet[PARAMETER0 + 2 + i] = value;
        }
        
        self.tx_packet(&mut tx_packet)
    }

    // 修复 sync_read_rx 实现
    pub fn sync_read_rx(&mut self, expected_ids: &[u32], data_length: u32) -> (COMM, Vec<u32>) {
        let mut all_data = Vec::new();
        
        for &scs_id in expected_ids {
            let (rx_data, result) = self.rx_packet();
            if result != COMM::Success {
                return (result, vec![]);
            }
            
            // 验证ID匹配
            if rx_data.len() > 2 && rx_data[2] == scs_id {
                all_data.extend_from_slice(&rx_data);
            } else {
                return (COMM::RxCorrupt, vec![]);
            }
        }
        
        (COMM::Success, all_data)
    }

    // 添加寄存器写入功能
    pub fn reg_write_1byte(&mut self, scs_id: u32, address: u32, data: u8) -> COMM {
        let mut tx_packet = vec![0u32; 8];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 4;
        tx_packet[INSTRUCTION] = INST::RegWrite as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = data as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    pub fn reg_write_2byte(&mut self, scs_id: u32, address: u32, data: u16) -> COMM {
        let mut tx_packet = vec![0u32; 9];
        tx_packet[ID] = scs_id;
        tx_packet[LENGTH] = 5;
        tx_packet[INSTRUCTION] = INST::RegWrite as u32;
        tx_packet[PARAMETER0] = address;
        tx_packet[PARAMETER0 + 1] = self.scs_lobyte(data as i32) as u32;
        tx_packet[PARAMETER0 + 2] = self.scs_hibyte(data as i32) as u32;
        
        self.tx_packet(&mut tx_packet)
    }

    // 添加批量读取功能
    pub fn bulk_read_tx(&mut self, param: Vec<u32>) -> COMM {
        let param_length = param.len() as u32;
        let mut tx_packet = vec![0u32; param_length as usize + 6];
        
        tx_packet[ID] = BROADCAST_ID as u32;
        tx_packet[LENGTH] = param_length + 2;
        tx_packet[INSTRUCTION] = INST::SyncRead as u32;
        
        for (i, &value) in param.iter().enumerate() {
            tx_packet[PARAMETER0 + i] = value;
        }
        
        self.tx_packet(&mut tx_packet)
    }

    // 获取端口处理器的可变引用
    pub fn get_port_handler_mut(&mut self) -> &mut PortHandler {
        &mut self.port_handler
    }

    // 获取端口处理器的不可变引用
    pub fn get_port_handler(&self) -> &PortHandler {
        &self.port_handler
    }
}
