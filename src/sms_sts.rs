use crate::{
    group_sync_write::GroupSyncWrite,
    port_handler::PortHandler,
    protocol_packet_handler::{Endian, ProtocolPacketHandler},
    scservo_def::COMM,
};

// 波特率定义
pub const SMS_STS_1M: u8 = 0;
pub const SMS_STS_0_5M: u8 = 1;
pub const SMS_STS_250K: u8 = 2;
pub const SMS_STS_128K: u8 = 3;
pub const SMS_STS_115200: u8 = 4;
pub const SMS_STS_76800: u8 = 5;
pub const SMS_STS_57600: u8 = 6;
pub const SMS_STS_38400: u8 = 7;

// 内存表定义
// -------EPROM(只读)--------
pub const SMS_STS_MODEL_L: u8 = 3;
pub const SMS_STS_MODEL_H: u8 = 4;

// -------EPROM(读写)--------
pub const SMS_STS_ID: u8 = 5;
pub const SMS_STS_BAUD_RATE: u8 = 6;
pub const SMS_STS_MIN_ANGLE_LIMIT_L: u8 = 9;
pub const SMS_STS_MIN_ANGLE_LIMIT_H: u8 = 10;
pub const SMS_STS_MAX_ANGLE_LIMIT_L: u8 = 11;
pub const SMS_STS_MAX_ANGLE_LIMIT_H: u8 = 12;
pub const SMS_STS_CW_DEAD: u8 = 26;
pub const SMS_STS_CCW_DEAD: u8 = 27;
pub const SMS_STS_OFS_L: u8 = 31;
pub const SMS_STS_OFS_H: u8 = 32;
pub const SMS_STS_MODE: u8 = 33;

// -------SRAM(读写)--------
pub const SMS_STS_TORQUE_ENABLE: u8 = 40;
pub const SMS_STS_ACC: u8 = 41;
pub const SMS_STS_GOAL_POSITION_L: u8 = 42;
pub const SMS_STS_GOAL_POSITION_H: u8 = 43;
pub const SMS_STS_GOAL_TIME_L: u8 = 44;
pub const SMS_STS_GOAL_TIME_H: u8 = 45;
pub const SMS_STS_GOAL_SPEED_L: u8 = 46;
pub const SMS_STS_GOAL_SPEED_H: u8 = 47;
pub const SMS_STS_LOCK: u8 = 55;

// -------SRAM(只读)--------
pub const SMS_STS_PRESENT_POSITION_L: u8 = 56;
pub const SMS_STS_PRESENT_POSITION_H: u8 = 57;
pub const SMS_STS_PRESENT_SPEED_L: u8 = 58;
pub const SMS_STS_PRESENT_SPEED_H: u8 = 59;
pub const SMS_STS_PRESENT_LOAD_L: u8 = 60;
pub const SMS_STS_PRESENT_LOAD_H: u8 = 61;
pub const SMS_STS_PRESENT_VOLTAGE: u8 = 62;
pub const SMS_STS_PRESENT_TEMPERATURE: u8 = 63;
pub const SMS_STS_MOVING: u8 = 66;
pub const SMS_STS_PRESENT_CURRENT_L: u8 = 69;
pub const SMS_STS_PRESENT_CURRENT_H: u8 = 70;

pub struct SmsSts {
    ph: ProtocolPacketHandler,
    group_sync_write: GroupSyncWrite,
}

impl SmsSts {
    pub fn new(port_handler: PortHandler) -> Self {
        let ph = ProtocolPacketHandler::new(port_handler, Endian::SmallEndian);
        let group_sync_write = GroupSyncWrite::new(
            ProtocolPacketHandler::new(
                PortHandler::new(&ph.port_handler.get_port_name()),
                Endian::SmallEndian,
            ),
            SMS_STS_ACC as u32,
            7,
        );
        
        Self {
            ph,
            group_sync_write,
        }
    }

    // 获取端口处理器的可变引用
    pub fn get_port_handler_mut(&mut self) -> &mut PortHandler {
        &mut self.ph.port_handler
    }

    // 获取端口处理器的不可变引用
    pub fn get_port_handler(&self) -> &PortHandler {
        &self.ph.port_handler
    }

    // 写入位置扩展（包含时间和速度）
    pub fn write_pos_ex(&mut self, scs_id: u32, position: i32, time: u32, speed: u32) -> COMM {
        let mut data = vec![0u32; 7];
        data[0] = 0; // ACC
        data[1] = self.ph.scs_lobyte(position) as u32;
        data[2] = self.ph.scs_hibyte(position) as u32;
        data[3] = self.ph.scs_lobyte(time as i32) as u32;
        data[4] = self.ph.scs_hibyte(time as i32) as u32;
        data[5] = self.ph.scs_lobyte(speed as i32) as u32;
        data[6] = self.ph.scs_hibyte(speed as i32) as u32;
        
        self.group_sync_write.clear_param();
        match self.group_sync_write.add_param(scs_id, data) {
            Ok(_) => self.group_sync_write.tx_packet(),
            Err(_) => COMM::TxError,
        }
    }

    // 读取当前位置
    pub fn read_pos(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SMS_STS_PRESENT_POSITION_L as u32);
        match result {
            COMM::Success => {
                if data.len() >= 7 { // 包头(2) + ID(1) + 长度(1) + 错误(1) + 数据(2)
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
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SMS_STS_PRESENT_SPEED_L as u32);
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

    // 同时读取位置和速度
    pub fn read_pos_speed(&mut self, scs_id: u32) -> Result<(i32, i32), COMM> {
        let pos = self.read_pos(scs_id)?;
        let speed = self.read_speed(scs_id)?;
        Ok((pos, speed))
    }

    // 读取运动状态
    pub fn read_moving(&mut self, scs_id: u32) -> Result<bool, COMM> {
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SMS_STS_MOVING as u32);
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

    // 同步写入多个舵机位置
    pub fn sync_write_pos_ex(&mut self, scs_ids: Vec<u32>, positions: Vec<i32>, times: Vec<u32>, speeds: Vec<u32>) -> COMM {
        if scs_ids.len() != positions.len() || positions.len() != times.len() || times.len() != speeds.len() {
            return COMM::TxError;
        }

        self.group_sync_write.clear_param();
        
        for i in 0..scs_ids.len() {
            let mut data = vec![0u32; 7];
            data[0] = 0; // ACC
            data[1] = self.ph.scs_lobyte(positions[i]) as u32;
            data[2] = self.ph.scs_hibyte(positions[i]) as u32;
            data[3] = self.ph.scs_lobyte(times[i] as i32) as u32;
            data[4] = self.ph.scs_hibyte(times[i] as i32) as u32;
            data[5] = self.ph.scs_lobyte(speeds[i] as i32) as u32;
            data[6] = self.ph.scs_hibyte(speeds[i] as i32) as u32;
            
            if let Err(_) = self.group_sync_write.add_param(scs_ids[i], data) {
                return COMM::TxError;
            }
        }
        
        self.group_sync_write.tx_packet()
    }

    // 寄存器写入位置扩展
    pub fn reg_write_pos_ex(&mut self, scs_id: u32, position: i32, time: u32, speed: u32) -> COMM {
        // 先写入位置
        let pos_result = self.ph.write_2byte_tx_rx(scs_id, SMS_STS_GOAL_POSITION_L as u32, position as u16);
        if pos_result != COMM::Success {
            return pos_result;
        }
        
        // 写入时间
        let time_result = self.ph.write_2byte_tx_rx(scs_id, SMS_STS_GOAL_TIME_L as u32, time as u16);
        if time_result != COMM::Success {
            return time_result;
        }
        
        // 写入速度
        self.ph.write_2byte_tx_rx(scs_id, SMS_STS_GOAL_SPEED_L as u32, speed as u16)
    }

    // 寄存器动作
    pub fn reg_action(&mut self, scs_id: u32) -> COMM {
        self.ph.action(scs_id)
    }

    // 轮式模式
    pub fn wheel_mode(&mut self, scs_id: u32, mode: u8) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_MODE as u32, mode)
    }

    // 写入扭矩使能
    pub fn write_torque_enable(&mut self, scs_id: u32, enable: bool) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_TORQUE_ENABLE as u32, if enable { 1 } else { 0 })
    }

    // 锁定EPROM
    pub fn lock_eprom(&mut self, scs_id: u32) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_LOCK as u32, 1)
    }

    // 解锁EPROM
    pub fn unlock_eprom(&mut self, scs_id: u32) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_LOCK as u32, 0)
    }

    // Ping舵机
    pub fn ping(&mut self, scs_id: u32) -> COMM {
        self.ph.ping(scs_id)
    }

    // 读取电压
    pub fn read_voltage(&mut self, scs_id: u32) -> Result<u8, COMM> {
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SMS_STS_PRESENT_VOLTAGE as u32);
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
        let (data, result) = self.ph.read_1byte_tx_rx(scs_id, SMS_STS_PRESENT_TEMPERATURE as u32);
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

    // 读取负载
    pub fn read_load(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SMS_STS_PRESENT_LOAD_L as u32);
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

    // 读取电流
    pub fn read_current(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (data, result) = self.ph.read_2byte_tx_rx(scs_id, SMS_STS_PRESENT_CURRENT_L as u32);
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

    // 设置ID
    pub fn set_id(&mut self, old_id: u32, new_id: u32) -> COMM {
        self.ph.write_1byte_tx_rx(old_id, SMS_STS_ID as u32, new_id as u8)
    }

    // 设置波特率
    pub fn set_baudrate(&mut self, scs_id: u32, baudrate: u8) -> COMM {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_BAUD_RATE as u32, baudrate)
    }
}
