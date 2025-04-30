use serialport::{SerialPort, SerialPortBuilder};

use std::time::Duration;

const DEFAULT_BAUDRATE: u32 = 1000000;
const LATENCY_TIMER: u32 = 50;

#[derive(Debug)]
pub struct PortHandler {
    port_name: String,
    is_open: bool,
    baudrate: u32,
    // time line
    packet_start_time: Duration,
    packet_timeout: Duration,
    tx_time_per_byte: Duration,

    is_using: bool,
    // use SerialPortBuilder
    ser: Option<Box<dyn SerialPort>>,
}

impl PortHandler {
    pub fn new(port_name: &str) -> Self {
        Self {
            port_name: port_name.to_string(),
            is_open: false,
            baudrate: DEFAULT_BAUDRATE,
            packet_start_time: Duration::default(),
            packet_timeout: Duration::default(),
            tx_time_per_byte: Duration::default(),

            is_using: false,
            ser: None,
        }
    }

    pub fn open_port(&mut self) -> bool {
        return self.set_baudrate(self.baudrate);
    }

    pub fn close_port(&mut self) {
        self.is_open = false;
    }

    pub fn clear_port(&mut self) -> Result<(), serialport::Error> {
        if let Some(serport) = &mut self.ser {
            serport.clear(serialport::ClearBuffer::All)?
        }
        Ok(())
    }

    pub fn set_port_name(&mut self, port_name: String) {
        self.port_name = port_name;
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }

    pub fn get_baudrate(&self) -> u32 {
        return self.baudrate;
    }

    // need to check the serial library
    pub fn get_bytes_available(&self) -> bool {
        return true;
    }

    // need to check the serial library
    pub fn read_port(&self, length: u32) {}

    pub fn write_port(&self, packet: SerialPortBuilder) {
        return self.ser.unwrap().open().unwrap().write();
    }

    pub fn set_packet_timeout(&self, packet_length: u32) {}

    pub fn set_packet_timeout_millis(&self, msec: u32) {}

    pub fn is_packet_timeout(&self) -> bool {
        return true;
    }

    pub fn get_time_since_start(&self) -> u32 {
        return 0;
    }

    pub fn setup_port(&mut self, cflag_baud: u32) -> bool {
        return true;
    }

    pub fn set_baudrate(&mut self, baudrate: u32) -> bool {
        let bauld = self.get_c_flag_baud(baudrate);

        if let Some(baud) = bauld {
            self.baudrate = baud;
            return self.setup_port(baud);
        } else {
            false
        }
    }

    pub fn get_c_flag_baud(&self, baudrate: u32) -> Option<u32> {
        let baudrate_list: Vec<u32> = vec![
            4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000, 250000, 500000, 1000000,
        ];
        if baudrate_list.contains(&baudrate) {
            Some(baudrate)
        } else {
            None
        }
    }
}

impl Drop for PortHandler {
    fn drop(&mut self) {
        let _ = self.close_port();
    }
}
