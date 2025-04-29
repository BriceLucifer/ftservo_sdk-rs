pub const BROADCAST_ID: u8 = 0xFE;
pub const MAX_ID: u8 = 0xFC;
pub const SCS_END: u8 = 0;

// Instruction for SCS Protocol
#[repr(u8)]
pub enum INST {
    Ping = 1,
    Read = 2,
    Write = 3,
    RegWrite = 4,
    Aciton = 5,
    SyncWrite = 131, // 0x83
    SyncRead = 130,  // 0x82
}

// Commuication Result
#[repr(i8)]
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
