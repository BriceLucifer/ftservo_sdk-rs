//! 舵机控制寄存器定义（Rust 版本）

// ====================== 波特率定义 ======================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmsClBaudRate {
    /// 1 Mbps
    SmsCl1M = 0,
    /// 0.5 Mbps
    SmsCl0_5M = 1,
    /// 250 Kbps
    SmsCl250K = 2,
    /// 128 Kbps
    SmsCl128K = 3,
    /// 115200 bps
    SmsCl115200 = 4,
    /// 76800 bps
    SmsCl76800 = 5,
    /// 57600 bps
    SmsCl57600 = 6,
    /// 38400 bps
    SmsCl38400 = 7,
}

impl SmsClBaudRate {
    /// 获取实际的波特率数值（单位：bps）
    pub fn baud_value(&self) -> u32 {
        match self {
            Self::SmsCl1M => 1_000_000,
            Self::SmsCl0_5M => 500_000,
            Self::SmsCl250K => 250_000,
            Self::SmsCl128K => 128_000,
            Self::SmsCl115200 => 115_200,
            Self::SmsCl76800 => 76_800,
            Self::SmsCl57600 => 57_600,
            Self::SmsCl38400 => 38_400,
        }
    }
}

// ====================== 寄存器地址定义 ======================
pub mod registers {
    // ======= EPROM (只读) =======
    pub const SMSCL_VERSION_L: u8 = 3;
    pub const SMSCL_VERSION_H: u8 = 4;

    // ======= EPROM (读写) =======
    pub const SMSCL_ID: u8 = 5;
    pub const SMSCL_BAUD_RATE: u8 = 6;
    pub const SMSCL_RETURN_DELAY_TIME: u8 = 7;
    pub const SMSCL_RETURN_LEVEL: u8 = 8;
    pub const SMSCL_MIN_ANGLE_LIMIT_L: u8 = 9;
    pub const SMSCL_MIN_ANGLE_LIMIT_H: u8 = 10;
    pub const SMSCL_MAX_ANGLE_LIMIT_L: u8 = 11;
    pub const SMSCL_MAX_ANGLE_LIMIT_H: u8 = 12;
    pub const SMSCL_LIMIT_TEMPERATURE: u8 = 13;
    pub const SMSCL_MAX_LIMIT_VOLTAGE: u8 = 14;
    pub const SMSCL_MIN_LIMIT_VOLTAGE: u8 = 15;
    pub const SMSCL_MAX_TORQUE_L: u8 = 16;
    pub const SMSCL_MAX_TORQUE_H: u8 = 17;
    pub const SMSCL_ALARM_LED: u8 = 19;
    pub const SMSCL_ALARM_SHUTDOWN: u8 = 20;
    pub const SMSCL_COMPLIANCE_P: u8 = 21;
    pub const SMSCL_COMPLIANCE_D: u8 = 22;
    pub const SMSCL_COMPLIANCE_I: u8 = 23;
    pub const SMSCL_PUNCH_L: u8 = 24;
    pub const SMSCL_PUNCH_H: u8 = 25;
    pub const SMSCL_CW_DEAD: u8 = 26;
    pub const SMSCL_CCW_DEAD: u8 = 27;
    pub const SMSCL_OFS_L: u8 = 33;
    pub const SMSCL_OFS_H: u8 = 34;
    pub const SMSCL_MODE: u8 = 35;
    pub const SMSCL_MAX_CURRENT_L: u8 = 36;
    pub const SMSCL_MAX_CURRENT_H: u8 = 37;

    // ======= SRAM (读写) =======
    pub const SMSCL_TORQUE_ENABLE: u8 = 40;
    pub const SMSCL_ACC: u8 = 41;
    pub const SMSCL_GOAL_POSITION_L: u8 = 42;
    pub const SMSCL_GOAL_POSITION_H: u8 = 43;
    pub const SMSCL_GOAL_TIME_L: u8 = 44;
    pub const SMSCL_GOAL_TIME_H: u8 = 45;
    pub const SMSCL_GOAL_SPEED_L: u8 = 46;
    pub const SMSCL_GOAL_SPEED_H: u8 = 47;
    pub const SMSCL_LOCK: u8 = 48;

    // ======= SRAM (只读) =======
    pub const SMSCL_PRESENT_POSITION_L: u8 = 56;
    pub const SMSCL_PRESENT_POSITION_H: u8 = 57;
    pub const SMSCL_PRESENT_SPEED_L: u8 = 58;
    pub const SMSCL_PRESENT_SPEED_H: u8 = 59;
    pub const SMSCL_PRESENT_LOAD_L: u8 = 60;
    pub const SMSCL_PRESENT_LOAD_H: u8 = 61;
    pub const SMSCL_PRESENT_VOLTAGE: u8 = 62;
    pub const SMSCL_PRESENT_TEMPERATURE: u8 = 63;
    pub const SMSCL_REGISTERED_INSTRUCTION: u8 = 64;
    pub const SMSCL_MOVING: u8 = 66;
    pub const SMSCL_PRESENT_CURRENT_L: u8 = 69;
    pub const SMSCL_PRESENT_CURRENT_H: u8 = 70;
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

    /// 将32位值拆分为4个字节
    pub fn split_u32(value: u32) -> (u8, u8, u8, u8) {
        (
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
        )
    }

    /// 将4个字节合并为32位值
    pub fn join_u32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
        ((b3 as u32) << 24) | ((b2 as u32) << 16) | ((b1 as u32) << 8) | (b0 as u32)
    }
}

/// 计算MEM的长度
const MEM_LENGTH: usize =
    (registers::SMSCL_PRESENT_CURRENT_H - registers::SMSCL_PRESENT_POSITION_L + 1) as usize;

pub struct SMSCL {
    pub mem: [u8; MEM_LENGTH],
}

impl SMSCL {
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
