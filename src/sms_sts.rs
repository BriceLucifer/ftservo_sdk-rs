use crate::{
    group_sync_write::GroupSyncWrite,
    port_handler::PortHandler,
    protocol_packet_handler::{Endian, ProtocolPacketHandler},
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
    group_sync_write: GroupSyncWrite,
}

/*
    python use inheritance with PortHandler
    I use group_sync_write for the whole thing
*/

impl SmsSts {
    pub fn new(port_handler: PortHandler) -> Self {
        /*
            - group_sync_write:
                - protocolPacketHandler:
                    - PortHandler,
                    - Endian
                - start_address,
                - data_length
        */
        Self {
            group_sync_write: GroupSyncWrite::new(
                ProtocolPacketHandler::new(port_handler, Endian::SmallEndian),
                SMS_STS_ACC as u32,
                7,
            ),
        }
    }
    pub fn write_pos_ex(&self) {}
    pub fn read_pos(&self) {}
    pub fn read_speed(&self) {}
    pub fn read_pos_speed(&self) {}
    pub fn read_moving(&self) {}
    pub fn sync_write_pos_ex(&self) {}
    pub fn reg_write_pos_ex(&self) {}
    pub fn reg_action(&self) {}
    pub fn wheel_node(&self) {}
    pub fn write_spec(&self) {}
    pub fn lock_eprom(&self) {}
    pub fn unlock_eprom(&self) {}
}
