#[cfg(test)]
mod tests {
    use ftservo_sdk::port_handler::PortHandler;
    use ftservo_sdk::protocol_packet_handler::{Endian, ProtocolPacketHandler};

    #[test]
    fn test_endian_conversion() {
        let port = PortHandler::new("/dev/null");
        let ph_le = ProtocolPacketHandler::new(port, Endian::SmallEndian);

        assert_eq!(ph_le.scs_lobyte(0x1234), 0x34);
        assert_eq!(ph_le.scs_hibyte(0x1234), 0x12);
        assert_eq!(ph_le.scs_makeword(0x34, 0x12), 0x1234);

        let port2 = PortHandler::new("/dev/null");
        let ph_be = ProtocolPacketHandler::new(port2, Endian::BigEndian);

        assert_eq!(ph_be.scs_lobyte(0x1234), 0x12);
        assert_eq!(ph_be.scs_hibyte(0x1234), 0x34);
        assert_eq!(ph_be.scs_makeword(0x34, 0x12), 0x3412);
    }

    #[test]
    fn test_scs_toscs() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);

        assert_eq!(ph.scs_toscs(100, 15), 100);
        assert_eq!(ph.scs_toscs(-100, 15), (100 | (1 << 15)));
        assert_eq!(ph.scs_toscs(0, 15), 0);
    }

    #[test]
    fn test_scs_tohost() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);

        assert_eq!(ph.scs_tohost(100, 15), 100);
        assert_eq!(ph.scs_tohost(100 | 1 << 15, 15), -100);
        assert_eq!(ph.scs_tohost(0, 15), 0);
    }

    #[test]
    fn test_word_operations() {
        let port = PortHandler::new("/dev/null");
        let ph = ProtocolPacketHandler::new(port, Endian::SmallEndian);

        assert_eq!(ph.scs_loword(0x12345678), 0x5678);
        assert_eq!(ph.scs_hiword(0x12345678), 0x1234);
        assert_eq!(ph.scs_makedword(0x5678, 0x1234), 0x12345678);
    }
}
