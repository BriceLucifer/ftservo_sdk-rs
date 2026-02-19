#[cfg(test)]
mod tests {
    use ftservo_sdk::group_sync_read::GroupSyncRead;
    use ftservo_sdk::port_handler::PortHandler;
    use ftservo_sdk::protocol_packet_handler::{Endian, ProtocolPacketHandler};

    #[test]
    fn test_group_sync_read_new() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);
        let _gsr = GroupSyncRead::new(ph, 56, 2);
    }

    #[test]
    fn test_add_param() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);
        let mut gsr = GroupSyncRead::new(ph, 56, 2);

        assert!(gsr.add_param(1).is_ok());
        assert!(gsr.add_param(1).is_err());
    }

    #[test]
    fn test_remove_param() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);
        let mut gsr = GroupSyncRead::new(ph, 56, 2);

        gsr.add_param(1).unwrap();
        assert!(gsr.remove_param(1).is_ok());
        assert!(gsr.remove_param(1).is_err());
    }

    #[test]
    fn test_clear_param() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);
        let mut gsr = GroupSyncRead::new(ph, 56, 2);

        gsr.add_param(1).unwrap();
        gsr.add_param(2).unwrap();
        gsr.clear_param();

        assert!(gsr.remove_param(1).is_err());
    }

    #[test]
    fn test_make_param() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);
        let mut gsr = GroupSyncRead::new(ph, 56, 2);

        gsr.add_param(1).unwrap();
        gsr.add_param(2).unwrap();
        gsr.add_param(3).unwrap();
        gsr.make_param();
    }
}
