use crate::{
    group_sync_write::GroupSyncWrite,
    port_handler::PortHandler,
    protocol_packet_handler::{Endian, ProtocolPacketHandler},
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

/*
    python use inheritance with PortHandler
    I use group_sync_write for the whole thing
*/

#[derive(Debug)]
pub struct Scscl {
    group_sync_write: GroupSyncWrite,
}

impl Scscl {
    pub fn new(port_handler: PortHandler) -> Self {
        let ph = ProtocolPacketHandler::new(port_handler, Endian::BigEndian);
        let group_sync_write = GroupSyncWrite::new(ph, SCSCL_GOAL_POSITION_L as u32, 6);
        Self { group_sync_write }
    }
}
