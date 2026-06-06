#[cfg(test)]
mod tests {
    use ftservo_sdk::port_handler::PortHandler;

    #[test]
    fn test_port_handler_new() {
        let port = PortHandler::new("/dev/null");
        assert_eq!(port.get_port_name(), "/dev/null");
        assert_eq!(port.get_baudrate(), 1000000);
    }

    #[test]
    fn test_set_port_name() {
        let mut port = PortHandler::new("/dev/null");
        port.set_port_name("/dev/ttyUSB0".to_string());
        assert_eq!(port.get_port_name(), "/dev/ttyUSB0");
    }

    #[test]
    fn test_valid_baudrates() {
        let port = PortHandler::new("/dev/null");

        let valid_bauds = vec![
            4800, 9600, 14400, 19200, 38400, 57600, 76800, 115200, 128000, 250000, 500000, 1000000,
        ];

        for &baud in &valid_bauds {
            assert_eq!(port.get_c_flag_baud(baud), Some(baud));
        }
    }

    #[test]
    fn test_invalid_baudrate() {
        let port = PortHandler::new("/dev/null");
        assert_eq!(port.get_c_flag_baud(12345), None);
    }

    #[test]
    fn test_get_current_time() {
        let port = PortHandler::new("/dev/null");
        assert!(port.get_current_time().is_some());
    }
}
