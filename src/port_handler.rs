use serialport::SerialPort;

use std::time::{Duration, Instant};

const DEFAULT_BAUDRATE: u32 = 1000000;
const LATENCY_TIMER: u32 = 50;

#[derive(Debug)]
pub struct PortHandler {
    port_name: String,
    is_open: bool,
    baudrate: u32,
    // time line
    packet_start_time: Option<Instant>,
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
            packet_start_time: None,
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
    pub fn read_port(&mut self, length: usize) -> Result<usize, std::io::Error> {
        let mut temp = String::with_capacity(length);
        if let Some(port) = &mut self.ser {
            return port.read_to_string(&mut temp);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "can not open",
            ));
        }
    }
    pub fn write_port(&mut self, packet: &[u8]) -> Result<usize, std::io::Error> {
        if let Some(port) = &mut self.ser {
            return port.write(packet);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "can not find",
            ));
        }
    }

    pub fn set_packet_timeout(&mut self, packet_length: u32) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = self.tx_time_per_byte * packet_length
            + self.tx_time_per_byte * 3
            + Duration::new(LATENCY_TIMER as u64, 0);
    }

    pub fn set_packet_timeout_millis(&mut self, msec: u64) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = Duration::from_millis(msec);
    }

    pub fn get_current_time(&self) -> Option<Instant> {
        return Some(Instant::now());
    }

    pub fn is_packet_timeout(&mut self) -> bool {
        if self.get_time_since_start() > self.packet_timeout {
            self.packet_timeout = Duration::new(0, 0);
            return true;
        }
        return false;
    }

    pub fn get_time_since_start(&mut self) -> Duration {
        let time_since = self.get_current_time().unwrap() - self.packet_start_time.unwrap();
        if time_since < Duration::new(0, 0) {
            self.packet_start_time = self.get_current_time();
        }
        return time_since;
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
