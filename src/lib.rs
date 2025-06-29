pub mod group_sync_read;
pub mod group_sync_write;
pub mod port_handler;
pub mod protocol_packet_handler;
pub mod scscl;
pub mod scservo_def;
pub mod sms_sts;

// 重新导出主要接口
pub use sms_sts::SmsSts;
pub use scscl::Scscl;
pub use port_handler::PortHandler;
pub use protocol_packet_handler::{ProtocolPacketHandler, Endian};
pub use group_sync_write::GroupSyncWrite;
pub use group_sync_read::GroupSyncRead;
pub use scservo_def::{COMM, INST, BROADCAST_ID, MAX_ID};

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum FtServoError {
    #[error("Serial port error: {0}")]
    SerialPort(#[from] serialport::Error),
    #[error("Communication error: {0:?}")]
    Communication(COMM),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("Timeout occurred")]
    Timeout,
    #[error("Checksum mismatch")]
    ChecksumError,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FtServoError>;

// 便利函数
pub fn create_port_handler(port_name: &str) -> PortHandler {
    PortHandler::new(port_name)
}

pub fn create_sms_sts(port_handler: PortHandler) -> SmsSts {
    SmsSts::new(port_handler)
}

pub fn create_scscl(port_handler: PortHandler) -> Scscl {
    Scscl::new(port_handler)
}
