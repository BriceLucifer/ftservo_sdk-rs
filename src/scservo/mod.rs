pub mod scscl;
pub mod smsbl;
pub mod smscl;
pub mod smssts;

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
