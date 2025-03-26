//! 舵机控制寄存器定义（Rust 版本）

// 波特率枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsclBaudRate {
    /// 1 Mbps
    SCScl1M = 0,
    /// 0.5 Mbps
    SCScl0_5M = 1,
    /// 250 Kbps
    SCScl250K = 2,
    /// 128 Kbps
    SCScl128K = 3,
    /// 115200 bps
    SCScl115200 = 4,
    /// 76800 bps
    SCScl76800 = 5,
    /// 57600 bps
    SCScl57600 = 6,
    /// 38400 bps
    SCScl38400 = 7,
}

// 寄存器地址定义
pub mod registers {
    // EPROM (只读)
    /// 固件版本号低字节
    pub const SCSCL_VERSION_L: u8 = 3;
    /// 固件版本号高字节
    pub const SCSCL_VERSION_H: u8 = 4;

    // EPROM (读写)
    /// 舵机ID
    pub const SCSCL_ID: u8 = 5;
    /// 波特率设置
    pub const SCSCL_BAUD_RATE: u8 = 6;
    /// 最小角度限制低字节
    pub const SCSCL_MIN_ANGLE_LIMIT_L: u8 = 9;
    /// 最小角度限制高字节
    pub const SCSCL_MIN_ANGLE_LIMIT_H: u8 = 10;
    /// 最大角度限制低字节
    pub const SCSCL_MAX_ANGLE_LIMIT_L: u8 = 11;
    /// 最大角度限制高字节
    pub const SCSCL_MAX_ANGLE_LIMIT_H: u8 = 12;
    /// 顺时针死区
    pub const SCSCL_CW_DEAD: u8 = 26;
    /// 逆时针死区
    pub const SCSCL_CCW_DEAD: u8 = 27;

    // SRAM (读写)
    /// 扭矩使能
    pub const SCSCL_TORQUE_ENABLE: u8 = 40;
    /// 目标位置低字节
    pub const SCSCL_GOAL_POSITION_L: u8 = 42;
    /// 目标位置高字节
    pub const SCSCL_GOAL_POSITION_H: u8 = 43;
    /// 运动时间低字节
    pub const SCSCL_GOAL_TIME_L: u8 = 44;
    /// 运动时间高字节
    pub const SCSCL_GOAL_TIME_H: u8 = 45;
    /// 目标速度低字节
    pub const SCSCL_GOAL_SPEED_L: u8 = 46;
    /// 目标速度高字节
    pub const SCSCL_GOAL_SPEED_H: u8 = 47;
    /// 参数锁定
    pub const SCSCL_LOCK: u8 = 48;

    // SRAM (只读)
    /// 当前位置低字节
    pub const SCSCL_PRESENT_POSITION_L: u8 = 56;
    /// 当前位置高字节
    pub const SCSCL_PRESENT_POSITION_H: u8 = 57;
    /// 当前速度低字节
    pub const SCSCL_PRESENT_SPEED_L: u8 = 58;
    /// 当前速度高字节
    pub const SCSCL_PRESENT_SPEED_H: u8 = 59;
    /// 当前负载低字节
    pub const SCSCL_PRESENT_LOAD_L: u8 = 60;
    /// 当前负载高字节
    pub const SCSCL_PRESENT_LOAD_H: u8 = 61;
    /// 当前电压
    pub const SCSCL_PRESENT_VOLTAGE: u8 = 62;
    /// 当前温度
    pub const SCSCL_PRESENT_TEMPERATURE: u8 = 63;
    /// 运动状态
    pub const SCSCL_MOVING: u8 = 66;
    /// 当前电流低字节
    pub const SCSCL_PRESENT_CURRENT_L: u8 = 69;
    /// 当前电流高字节
    pub const SCSCL_PRESENT_CURRENT_H: u8 = 70;
}

// 辅助方法
impl ScsclBaudRate {
    /// 获取波特率数值 (bps)
    pub fn baud_value(&self) -> u32 {
        match self {
            Self::SCScl1M => 1_000_000,
            Self::SCScl0_5M => 500_000,
            Self::SCScl250K => 250_000,
            Self::SCScl128K => 128_000,
            Self::SCScl115200 => 115_200,
            Self::SCScl76800 => 76_800,
            Self::SCScl57600 => 57_600,
            Self::SCScl38400 => 38_400,
        }
    }
}

const MEM_LENGTH: usize =
    (registers::SCSCL_PRESENT_CURRENT_H - registers::SCSCL_PRESENT_POSITION_L + 1) as usize;

pub struct SCSCL {
    pub mem: [u8; MEM_LENGTH],
}

impl SCSCL {
    /// 初始化函数
    pub fn new(end: u8, level: u8) -> Self {
        Self {
            mem: [0; MEM_LENGTH],
        }
    }
    /// 普通写单个舵机位置指令
    pub fn write_pos(&self, id: u8, position: u16, time: u16, speed: u16) -> i32 {
        todo!()
    }
    /// 异步写单个舵机位置指令(RegWriteAction生效)
    pub fn reg_write_pos(&self, id: u8, position: u16, time: u16, speed: u16) -> i32 {
        todo!()
    }
    pub fn sync_write_pos(&self, id: &[u8], idn: u8, positon: &[u16], time: &[u16], speed: &[u16]) {
        todo!()
    }
    pub fn pwm_mode(&self, id: u8) -> i32 {
        todo!()
    }
    pub fn write_pwm(&self, id: u8, pwmout: i16) -> i32 {
        todo!()
    }
    pub fn enable_torque(&self, id: u8, enable: u8) -> i32 {
        todo!()
    }
    pub fn unlock_eprom(&self, id: u8) -> i32 {
        todo!()
    }
    pub fn lock_eprom(&self, id: u8) -> i32 {
        todo!()
    }
}
