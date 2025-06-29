pub const BROADCAST_ID: u8 = 0xFE;
pub const MAX_ID: u8 = 0xFC;
pub const SCS_END: u8 = 0;

// Instruction for SCS Protocol
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum INST {
    Ping = 1,
    Read = 2,
    Write = 3,
    RegWrite = 4,
    Action = 5,
    SyncWrite = 131, // 0x83
    SyncRead = 130,  // 0x82
}

// Communication Result
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum COMM {
    Success = 0,
    PortBusy = -1,
    TxFail = -2,
    RxFail = -3,
    TxError = -4,
    RxWaiting = -5,
    RxTimeout = -6,
    RxCorrupt = -7,
    NotAvailable = -9,
}

// 错误代码定义
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    Success = 0,
    VoltageError = 1,
    AngleError = 2,
    OverheatError = 4,
    OverElementError = 8,
    OverloadError = 32,
}

// 舵机模式定义
pub const SERVO_MODE: u8 = 0;
pub const MOTOR_MODE: u8 = 1;

// 常用波特率定义
pub const BAUD_1M: u8 = 0;
pub const BAUD_500K: u8 = 1;
pub const BAUD_250K: u8 = 2;
pub const BAUD_128K: u8 = 3;
pub const BAUD_115200: u8 = 4;
pub const BAUD_76800: u8 = 5;
pub const BAUD_57600: u8 = 6;
pub const BAUD_38400: u8 = 7;
