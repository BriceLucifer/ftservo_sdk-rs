//! 舵机控制寄存器定义（Rust 版本）

// ====================== 波特率定义 ======================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmsBlBaudRate {
    /// 1 Mbps
    SmsBl1M = 0,
    /// 0.5 Mbps
    SmsBl0_5M = 1,
    /// 250 Kbps
    SmsBl250K = 2,
    /// 128 Kbps
    SmsBl128K = 3,
    /// 115200 bps
    SmsBl115200 = 4,
    /// 76800 bps
    SmsBl76800 = 5,
    /// 57600 bps
    SmsBl57600 = 6,
    /// 38400 bps
    SmsBl38400 = 7,
}

impl SmsBlBaudRate {
    /// 获取实际的波特率数值（单位：bps）
    pub fn baud_value(&self) -> u32 {
        match self {
            Self::SmsBl1M => 1_000_000,
            Self::SmsBl0_5M => 500_000,
            Self::SmsBl250K => 250_000,
            Self::SmsBl128K => 128_000,
            Self::SmsBl115200 => 115_200,
            Self::SmsBl76800 => 76_800,
            Self::SmsBl57600 => 57_600,
            Self::SmsBl38400 => 38_400,
        }
    }
}

// ====================== 寄存器地址定义 ======================
pub mod registers {
    // ======= EPROM (只读) =======
    pub const SMSBL_MODEL_L: u8 = 3;
    pub const SMSBL_MODEL_H: u8 = 4;

    // ======= EPROM (读写) =======
    pub const SMSBL_ID: u8 = 5;
    pub const SMSBL_BAUD_RATE: u8 = 6;
    pub const SMSBL_MIN_ANGLE_LIMIT_L: u8 = 9;
    pub const SMSBL_MIN_ANGLE_LIMIT_H: u8 = 10;
    pub const SMSBL_MAX_ANGLE_LIMIT_L: u8 = 11;
    pub const SMSBL_MAX_ANGLE_LIMIT_H: u8 = 12;
    pub const SMSBL_CW_DEAD: u8 = 26;
    pub const SMSBL_CCW_DEAD: u8 = 27;
    pub const SMSBL_OFS_L: u8 = 31;
    pub const SMSBL_OFS_H: u8 = 32;
    pub const SMSBL_MODE: u8 = 33;

    // ======= SRAM (读写) =======
    pub const SMSBL_TORQUE_ENABLE: u8 = 40;
    pub const SMSBL_ACC: u8 = 41;
    pub const SMSBL_GOAL_POSITION_L: u8 = 42;
    pub const SMSBL_GOAL_POSITION_H: u8 = 43;
    pub const SMSBL_GOAL_TIME_L: u8 = 44;
    pub const SMSBL_GOAL_TIME_H: u8 = 45;
    pub const SMSBL_GOAL_SPEED_L: u8 = 46;
    pub const SMSBL_GOAL_SPEED_H: u8 = 47;
    pub const SMSBL_LOCK: u8 = 55;

    // ======= SRAM (只读) =======
    pub const SMSBL_PRESENT_POSITION_L: u8 = 56;
    pub const SMSBL_PRESENT_POSITION_H: u8 = 57;
    pub const SMSBL_PRESENT_SPEED_L: u8 = 58;
    pub const SMSBL_PRESENT_SPEED_H: u8 = 59;
    pub const SMSBL_PRESENT_LOAD_L: u8 = 60;
    pub const SMSBL_PRESENT_LOAD_H: u8 = 61;
    pub const SMSBL_PRESENT_VOLTAGE: u8 = 62;
    pub const SMSBL_PRESENT_TEMPERATURE: u8 = 63;
    pub const SMSBL_MOVING: u8 = 66;
    pub const SMSBL_PRESENT_CURRENT_L: u8 = 69;
    pub const SMSBL_PRESENT_CURRENT_H: u8 = 70;
}

// ====================== 辅助方法 ======================
/// 用于处理16位寄存器的辅助方法
pub mod utils {
    /// 将16位值拆分为高低字节
    pub fn split_u16(value: u16) -> (u8, u8) {
        ((value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8)
    }

    /// 将高低字节合并为16位值
    pub fn join_u16(low: u8, high: u8) -> u16 {
        ((high as u16) << 8) | (low as u16)
    }
}

/// 计算mem长度
const MEM_LENGTH: usize =
    (registers::SMSBL_PRESENT_CURRENT_H - registers::SMSBL_PRESENT_POSITION_L + 1) as usize;

pub struct SMSBL {
    pub mem: [u8; MEM_LENGTH],
}

impl SMSBL {
    /// 初始化函数
    pub fn new(end: u8, level: u8) -> Self {
        Self {
            mem: [0; MEM_LENGTH],
        }
    }
    /// 普通写单个舵机位置指令
    pub fn write_pos_ex(&self, id: u8, position: u16, speed: u16, acc: u8) -> i32 {
        todo!()
    }
    /// 异步写单个舵机位置指令(RegWriteAction生效)
    pub fn reg_write_pos_ex(&self, id: u8, position: u16, speed: u16, acc: u8) -> i32 {
        todo!()
    }
    /// 同步写多个舵机位置指令
    pub fn sync_write_pos_ex(
        &self,
        id: &[u8],
        idn: u8,
        positon: &[u16],
        speed: &[u16],
        acc: &[u8],
    ) {
        todo!()
    }
    /// 恒速模式
    pub fn wheel_mode(&self, id: u8) -> i32 {
        todo!()
    }
    /// PWM输出指令模式
    pub fn write_pwm(&self, id: u8, pwmout: i16) -> i32 {
        todo!()
    }
    /// 扭矩控制指令
    pub fn enable_torque(&self, id: u8, enable: u8) -> i32 {
        todo!()
    }
    /// 解锁EPROM
    pub fn unlock_eprom(&self, id: u8) -> i32 {
        todo!()
    }
    /// 锁定EPROM
    pub fn lock_eprom(&self, id: u8) -> i32 {
        todo!()
    }
    /// 中位校准
    pub fn calibration_ofs(&self, id: u8) -> i32 {
        todo!()
    }
    /// 反馈舵机信息
    pub fn feed_back(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机位置
    pub fn read_pos(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机速度
    pub fn read_speed(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取电压百分比(0~1000)
    pub fn read_load(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机电压
    pub fn read_voltage(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机温度
    pub fn read_temper(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机移动状态
    pub fn read_move(&self, id: u8) -> i32 {
        todo!()
    }
    /// 读取舵机电流
    pub fn read_current(&self, id: u8) -> i32 {
        todo!()
    }
}
