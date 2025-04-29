use crate::{group_sync_write::GroupSyncWrite, protocol_packet_handler::ProtocolPacketHandler};

#[derive(Debug)]
pub struct Scscl {
    protocol_packet_handler: ProtocolPacketHandler,
    group_sync_write: GroupSyncWrite,
}
