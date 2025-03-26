pub mod INST {
    pub const INST_PING: u8 = 0x01;
    pub const INST_READ: u8 = 0x02;
    pub const INST_WRITE: u8 = 0x03;
    pub const INST_REG_WRITE: u8 = 0x04;
    pub const INST_REG_ACTION: u8 = 0x05;
    pub const INST_SYNC_READ: u8 = 0x82;
    pub const INST_SYNC_WRITE: u8 = 0x83;
}
