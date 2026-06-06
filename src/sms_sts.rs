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
    pub ph: ProtocolPacketHandler,
    pub group_sync_write: GroupSyncWrite,
}

impl SmsSts {
    pub fn new(port_handler: PortHandler) -> Self {
        let ph = ProtocolPacketHandler::new(port_handler, Endian::SmallEndian);
        let group_sync_write = GroupSyncWrite::new(SMS_STS_ACC as u32, 7);

        Self {
            ph,
            group_sync_write,
        }
    }

    pub fn get_port_handler_mut(&mut self) -> &mut PortHandler {
        &mut self.ph.port_handler
    }

    pub fn get_port_handler(&self) -> &PortHandler {
        &self.ph.port_handler
    }

    pub fn write_pos_ex(&mut self, scs_id: u32, position: i32, speed: i32, acc: u32) -> (COMM, u8) {
        let position = self.ph.scs_toscs(position, 15);
        let txpacket = vec![
            acc,
            self.ph.scs_lobyte(position) as u32,
            self.ph.scs_hibyte(position) as u32,
            0,
            0,
            self.ph.scs_lobyte(speed) as u32,
            self.ph.scs_hibyte(speed) as u32,
        ];
        self.ph
            .write_tx_rx(scs_id, SMS_STS_ACC as u32, txpacket.len() as u32, &txpacket)
    }

    pub fn read_pos(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (pos, result, _) = self
            .ph
            .read_2byte_value(scs_id, SMS_STS_PRESENT_POSITION_L as u32);
        match result {
            COMM::Success => Ok(self.ph.scs_tohost(pos as i32, 15)),
            _ => Err(result),
        }
    }

    pub fn read_speed(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (speed, result, _) = self
            .ph
            .read_2byte_value(scs_id, SMS_STS_PRESENT_SPEED_L as u32);
        match result {
            COMM::Success => Ok(self.ph.scs_tohost(speed as i32, 15)),
            _ => Err(result),
        }
    }

    pub fn read_pos_speed(&mut self, scs_id: u32) -> Result<(i32, i32), COMM> {
        let (pos_speed, result, _) = self
            .ph
            .read_4byte_value(scs_id, SMS_STS_PRESENT_POSITION_L as u32);
        match result {
            COMM::Success => {
                let pos = self.ph.scs_loword(pos_speed as i32);
                let speed = self.ph.scs_hiword(pos_speed as i32);
                Ok((self.ph.scs_tohost(pos, 15), self.ph.scs_tohost(speed, 15)))
            }
            _ => Err(result),
        }
    }

    pub fn read_moving(&mut self, scs_id: u32) -> Result<bool, COMM> {
        let (moving, result, _) = self.ph.read_1byte_value(scs_id, SMS_STS_MOVING as u32);
        match result {
            COMM::Success => Ok(moving != 0),
            _ => Err(result),
        }
    }

    pub fn sync_write_pos_ex(
        &mut self,
        scs_id: u32,
        position: i32,
        speed: i32,
        acc: u32,
    ) -> Result<(), std::io::Error> {
        let position = self.ph.scs_toscs(position, 15);
        let txpacket = vec![
            acc,
            self.ph.scs_lobyte(position) as u32,
            self.ph.scs_hibyte(position) as u32,
            0,
            0,
            self.ph.scs_lobyte(speed) as u32,
            self.ph.scs_hibyte(speed) as u32,
        ];
        self.group_sync_write.add_param(scs_id, txpacket)
    }

    pub fn reg_write_pos_ex(
        &mut self,
        scs_id: u32,
        position: i32,
        speed: i32,
        acc: u32,
    ) -> (COMM, u8) {
        let position = self.ph.scs_toscs(position, 15);
        let txpacket = vec![
            acc,
            self.ph.scs_lobyte(position) as u32,
            self.ph.scs_hibyte(position) as u32,
            0,
            0,
            self.ph.scs_lobyte(speed) as u32,
            self.ph.scs_hibyte(speed) as u32,
        ];
        self.ph
            .reg_write_tx_rx(scs_id, SMS_STS_ACC as u32, txpacket.len() as u32, &txpacket)
    }

    pub fn reg_action(&mut self) -> COMM {
        use crate::scservo_def::BROADCAST_ID;
        self.ph.action(BROADCAST_ID as u32)
    }

    pub fn wheel_mode(&mut self, scs_id: u32) -> (COMM, u8) {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_MODE as u32, 1)
    }

    pub fn write_spec(&mut self, scs_id: u32, speed: i32, acc: u32) -> (COMM, u8) {
        let speed = self.ph.scs_toscs(speed, 15);
        let txpacket = vec![
            acc,
            0,
            0,
            0,
            0,
            self.ph.scs_lobyte(speed) as u32,
            self.ph.scs_hibyte(speed) as u32,
        ];
        self.ph
            .write_tx_rx(scs_id, SMS_STS_ACC as u32, txpacket.len() as u32, &txpacket)
    }

    pub fn write_torque_enable(&mut self, scs_id: u32, enable: bool) -> (COMM, u8) {
        self.ph.write_1byte_tx_rx(
            scs_id,
            SMS_STS_TORQUE_ENABLE as u32,
            if enable { 1 } else { 0 },
        )
    }

    pub fn lock_eprom(&mut self, scs_id: u32) -> (COMM, u8) {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_LOCK as u32, 1)
    }

    pub fn unlock_eprom(&mut self, scs_id: u32) -> (COMM, u8) {
        self.ph.write_1byte_tx_rx(scs_id, SMS_STS_LOCK as u32, 0)
    }

    pub fn ping(&mut self, scs_id: u32) -> COMM {
        let (_, result, _) = self.ph.ping(scs_id);
        result
    }

    pub fn ping_model(&mut self, scs_id: u32) -> Result<u16, COMM> {
        let (model, result, _) = self.ph.ping(scs_id);
        match result {
            COMM::Success => Ok(model),
            _ => Err(result),
        }
    }

    pub fn read_voltage(&mut self, scs_id: u32) -> Result<u8, COMM> {
        let (voltage, result, _) = self
            .ph
            .read_1byte_value(scs_id, SMS_STS_PRESENT_VOLTAGE as u32);
        match result {
            COMM::Success => Ok(voltage),
            _ => Err(result),
        }
    }

    pub fn read_temperature(&mut self, scs_id: u32) -> Result<u8, COMM> {
        let (temperature, result, _) = self
            .ph
            .read_1byte_value(scs_id, SMS_STS_PRESENT_TEMPERATURE as u32);
        match result {
            COMM::Success => Ok(temperature),
            _ => Err(result),
        }
    }

    pub fn read_load(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (load, result, _) = self
            .ph
            .read_2byte_value(scs_id, SMS_STS_PRESENT_LOAD_L as u32);
        match result {
            COMM::Success => Ok(self.ph.scs_tohost(load as i32, 10)),
            _ => Err(result),
        }
    }

    pub fn read_current(&mut self, scs_id: u32) -> Result<i32, COMM> {
        let (current, result, _) = self
            .ph
            .read_2byte_value(scs_id, SMS_STS_PRESENT_CURRENT_L as u32);
        match result {
            COMM::Success => Ok(self.ph.scs_tohost(current as i32, 15)),
            _ => Err(result),
        }
    }

    pub fn read_model(&mut self, scs_id: u32) -> Result<u16, COMM> {
        let (model, result, _) = self.ph.read_2byte_value(scs_id, SMS_STS_MODEL_L as u32);
        match result {
            COMM::Success => Ok(model),
            _ => Err(result),
        }
    }

    pub fn set_id(&mut self, old_id: u32, new_id: u32) -> (COMM, u8) {
        self.ph
            .write_1byte_tx_rx(old_id, SMS_STS_ID as u32, new_id as u8)
    }

    pub fn set_baudrate(&mut self, scs_id: u32, baudrate: u8) -> (COMM, u8) {
        self.ph
            .write_1byte_tx_rx(scs_id, SMS_STS_BAUD_RATE as u32, baudrate)
    }
}
