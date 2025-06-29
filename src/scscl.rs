use std::time::Duration;

use crate::{
    group_sync_write::GroupSyncWrite,
    port_handler::PortHandler,
    protocol_packet_handler::{Endian, ProtocolPacketHandler},
    scservo_def::COMM,
};

// 波特率定义
pub const SCSCL_1M: u8 = 0;
pub const SCSCL_0_5M: u8 = 1;
pub const SCSCL_250K: u8 = 2;
pub const SCSCL_128K: u8 = 3;
pub const SCSCL_115200: u8 = 4;
pub const SCSCL_76800: u8 = 5;
pub const SCSCL_57600: u8 = 6;
pub const SCSCL_38400: u8 = 7;

// 内存表定义
// -------EPROM(只读)--------
pub const SCSCL_MODEL_L: u8 = 3;
pub const SCSCL_MODEL_H: u8 = 4;

// -------EPROM(读写)--------
pub const SCSCL_ID: u8 = 5;
pub const SCSCL_BAUD_RATE: u8 = 6;
pub const SCSCL_MIN_ANGLE_LIMIT_L: u8 = 9;
pub const SCSCL_MIN_ANGLE_LIMIT_H: u8 = 10;
pub const SCSCL_MAX_ANGLE_LIMIT_L: u8 = 11;
pub const SCSCL_MAX_ANGLE_LIMIT_H: u8 = 12;
pub const SCSCL_CW_DEAD: u8 = 26;
pub const SCSCL_CCW_DEAD: u8 = 27;
pub const SCSCL_OFS_L: u8 = 31;
pub const SCSCL_OFS_H: u8 = 32;
pub const SCSCL_MODE: u8 = 33;

// -------SRAM(读写)--------
pub const SCSCL_TORQUE_ENABLE: u8 = 40;
pub const SCSCL_ACC: u8 = 41;
pub const SCSCL_GOAL_POSITION_L: u8 = 42;
pub const SCSCL_GOAL_POSITION_H: u8 = 43;
pub const SCSCL_GOAL_TIME_L: u8 = 44;
pub const SCSCL_GOAL_TIME_H: u8 = 45;
pub const SCSCL_GOAL_SPEED_L: u8 = 46;
pub const SCSCL_GOAL_SPEED_H: u8 = 47;
pub const SCSCL_LOCK: u8 = 55;

// -------SRAM(只读)--------
pub const SCSCL_PRESENT_POSITION_L: u8 = 56;
pub const SCSCL_PRESENT_POSITION_H: u8 = 57;
pub const SCSCL_PRESENT_SPEED_L: u8 = 58;
pub const SCSCL_PRESENT_SPEED_H: u8 = 59;
pub const SCSCL_PRESENT_LOAD_L: u8 = 60;
pub const SCSCL_PRESENT_LOAD_H: u8 = 61;
pub const SCSCL_PRESENT_VOLTAGE: u8 = 62;
pub const SCSCL_PRESENT_TEMPERATURE: u8 = 63;
pub const SCSCL_MOVING: u8 = 66;
pub const SCSCL_PRESENT_CURRENT_L: u8 = 69;
pub const SCSCL_PRESENT_CURRENT_H: u8 = 70;

#[derive(Debug)]
pub struct Scscl {
    ph: ProtocolPacketHandler,
    group_sync_write: GroupSyncWrite,
}

impl Scscl {
    pub fn new(port_handler: PortHandler) -> Self {
        let ph = ProtocolPacketHandler::new(port_handler, Endian::BigEndian);
        let group_sync_write = GroupSyncWrite::new(
            ProtocolPacketHandler::new(
                PortHandler::new(&ph.port_handler.get_port_name()),
                Endian::BigEndian,
            ),
            SCSCL_ACC as u32,
            6,
        );
        
        Self {
            ph,
            group_sync_write,
        }
    }

    // 写入位置、时间和速度
    pub fn write_pos(&mut self, scs_id: u32, position: i32, time: u32, speed: u32) -> COMM {
        let mut data = vec![0u32; 6];
        data[0] = 0; // ACC
        data[1] = self.ph.scs_lobyte(position) as u32;
        data[2] = self.ph.scs_hibyte(position) as u32;
        data[3] = self.ph.scs_lobyte(time as i32) as u32;
        data[4] = self.ph.scs_hibyte(time as i32) as u32;
        data[5] = self.ph.scs_lobyte(speed as i32) as u32;
        
        match self.group_sync_write.add_param(scs_id, data) {
            Ok(_) => self.group_sync_write.tx_packet(),
            Err(_) => COMM::TxError,
        }
    }

    // 读取当前位置
    pub fn read_pos(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SCSCL_PRESENT_POSITION_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 {
                    let pos = self.ph.scs_makeword(data[5] as i32, data[6] as i32);
                    Ok(self.ph.scs_tohost(pos, 15))
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取当前速度
    pub fn read_speed(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SCSCL_PRESENT_SPEED_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 {
                    let speed = self.ph.scs_makeword(data[5] as i32, data[6] as i32);
                    Ok(self.ph.scs_tohost(speed, 15))
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取当前负载
    pub fn read_load(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SCSCL_PRESENT_LOAD_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 {
                    let load = self.ph.scs_makeword(data[5] as i32, data[6] as i32);
                    Ok(self.ph.scs_tohost(load, 10))
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取电压
    pub fn read_voltage(&mut self, scs_id: u32) -> Result<u8, COMM> {
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SCSCL_PRESENT_VOLTAGE as u32);
        match result {
            COMM::Success => {
                if data.len() >= 6 {
                    Ok(data[5] as u8)
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取温度
    pub fn read_temperature(&mut self, scs_id: u32) -> Result<u8, COMM> {
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SCSCL_PRESENT_TEMPERATURE as u32);
        match result {
            COMM::Success => {
                if data.len() >= 6 {
                    Ok(data[5] as u8)
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取是否在运动
    pub fn read_moving(&mut self, scs_id: u32) -> Result<bool, COMM> {
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SCSCL_MOVING as u32);
        match result {
            COMM::Success => {
                if data.len() >= 6 {
                    Ok(data[5] != 0)
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 读取电流
    pub fn read_current(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SCSCL_PRESENT_CURRENT_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 {
                    let current = self.ph.scs_makeword(data[5] as i32, data[6] as i32);
                    Ok(self.ph.scs_tohost(current, 15))
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // 同步写入多个舵机位置
    pub fn sync_write_pos(&mut self, scs_ids: Vec<u32>, positions: Vec<i32>, times: Vec<u32>, speeds: Vec<u32>) -> COMM {
        if scs_ids.len() != positions.len() || positions.len() != times.len() || times.len() != speeds.len() {
            return COMM::TxError;
        }

        self.group_sync_write.clear_param();
        
        for i in 0..scs_ids.len() {
            let mut data = vec![0u32; 6];
            data[0] = 0; // ACC
            data[1] = self.ph.scs_lobyte(positions[i]) as u32;
            data[2] = self.ph.scs_hibyte(positions[i]) as u32;
            data[3] = self.ph.scs_lobyte(times[i] as i32) as u32;
            data[4] = self.ph.scs_hibyte(times[i] as i32) as u32;
            data[5] = self.ph.scs_lobyte(speeds[i] as i32) as u32;
            
            if let Err(_) = self.group_sync_write.add_param(scs_ids[i], data) {
                return COMM::TxError;
            }
        }
        
        self.group_sync_write.tx_packet()
    }

    // 设置舵机模式
    pub fn wheel_mode(&mut self, scs_id: u32, mode: u8) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_MODE as u32, mode)
    }

    // 设置扭矩使能
    pub fn write_torque_enable(&mut self, scs_id: u32, enable: bool) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_TORQUE_ENABLE as u32, if enable { 1 } else { 0 })
    }

    // 设置角度限制
    pub fn write_angle_limit(&mut self, scs_id: u32, min_angle: i32, max_angle: i32) -> COMM {
        let result1 = self.ph.write_2byte_tx_rx(scs_id, SCSCL_MIN_ANGLE_LIMIT_L as u32, min_angle as u16);
        if result1 != COMM::Success {
            return result1;
        }
        self.ph.write_2byte_tx_rx(scs_id, SCSCL_MAX_ANGLE_LIMIT_L as u32, max_angle as u16)
    }

    // 设置死区
    pub fn write_dead_zone(&mut self, scs_id: u32, cw_dead: u8, ccw_dead: u8) -> COMM {
        let result1 = self.ph.write_1byte_tx_rx(scs_id, SCSCL_CW_DEAD as u32, cw_dead);
        if result1 != COMM::Success {
            return result1;
        }
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_CCW_DEAD as u32, ccw_dead)
    }

    // 设置偏移量
    pub fn write_offset(&mut self, scs_id: u32, offset: i32) -> COMM {
        self.ph.write_2byte_tx_rx(scs_id, SCSCL_OFS_L as u32, offset as u16)
    }

    // 锁定EPROM
    pub fn lock_eprom(&mut self, scs_id: u32) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_LOCK as u32, 1)
    }

    // 解锁EPROM
    pub fn unlock_eprom(&mut self, scs_id: u32) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_LOCK as u32, 0)
    }

    // 设置波特率
    pub fn write_baudrate(&mut self, scs_id: u32, baudrate: u8) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_BAUD_RATE as u32, baudrate)
    }

    // 设置ID
    pub fn write_id(&mut self, scs_id: u32, new_id: u8) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SCSCL_ID as u32, new_id)
    }

    // 读取模型号
    pub fn read_model(&mut self, scs_id: u32) -> Result<u16, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SCSCL_MODEL_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 {
                    let model = self.ph.scs_makeword(data[5] as i32, data[6] as i32) as u16;
                    Ok(model)
                } else {
                    Err(COMM::RxCorrupt)
                }
            }
            _ => Err(result),
        }
    }

    // ping测试
    pub fn ping(&mut self, scs_id: u32) -> COMM {
        self.ph.ping(scs_id)
    }

    // 寄存器写入
    pub fn reg_write_pos(&mut self, scs_id: u32, position: i32, time: u32, speed: u32) -> COMM {
        // 实现寄存器写入位置控制
        self.ph.write_2byte_tx_rx(scs_id, SCSCL_GOAL_POSITION_L as u32, position as u16)
    }

    // 执行寄存器写入的动作
    pub fn reg_action(&mut self, scs_id: u32) -> COMM {
        self.ph.action(scs_id)
    }
}
