use crate::{group_sync_write::GroupSyncWrite, protocol_packet_handler::ProtocolPacketHandler};

/*
    python use inheritance with PortHandler
    I use group_sync_write for the whole thing
*/

#[derive(Debug)]
pub struct Scsci {
    group_sync_write: GroupSyncWrite,
}
