use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};

use std::{
    io::ErrorKind,
    time::{Duration, Instant},
};

// some default setting
const DEFAULT_BAUDRATE: u32 = 1000000;
const LATENCY_TIMER: u32 = 50;

// PortHandler structure
#[derive(Debug)]
pub struct PortHandler {
    port_name: String,
    is_open: bool,
    baudrate: u32,
    // time line
    packet_start_time: Option<Instant>,
    packet_timeout: Duration,
    tx_time_per_byte: Duration,

    pub is_using: bool,
    // use SerialPortBuilder
    ser: Option<Box<dyn SerialPort>>,
}

impl PortHandler {
    /// new a PortHandler
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

    // open a port
    pub fn open_port(&mut self) -> Result<(), serialport::Error> {
        return self.setup_port();
    }

    // close the port
    pub fn close_port(&mut self) -> Result<(), serialport::Error> {
        if let Some(port) = &mut self.ser {
            port.flush()?;
        }
        self.ser = None;
        self.is_open = false;
        Ok(())
    }

    // clear the transfermition
    pub fn clear_port(&mut self) -> Result<(), serialport::Error> {
        if let Some(serport) = &mut self.ser {
            serport.clear(ClearBuffer::All)?
        }
        Ok(())
    }

    // set a new port name
    pub fn set_port_name(&mut self, port_name: String) {
        self.port_name = port_name;
    }

    // get the port name
    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }

    // get the baurate
    pub fn get_baudrate(&self) -> u32 {
        return self.baudrate;
    }

    // check if the port is available
    pub fn get_bytes_available(&self) -> Result<u32, serialport::Error> {
        match &self.ser {
            Some(port) => port.bytes_to_read(),
            None => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::NotConnected),
                "Port not open",
            )),
        }
    }

    // read the port
    pub fn read_port(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        if let Some(port) = &mut self.ser {
            return port.read(buf).map_err(|e| e.into());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "can not open",
            ));
        }
    }

    // write through the port
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

    // set the timeout
    pub fn set_packet_timeout(&mut self, packet_length: u32) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = self.tx_time_per_byte * packet_length
            + self.tx_time_per_byte * 3
            + Duration::from_millis(LATENCY_TIMER as u64);
    }

    // set the timeout in millis
    pub fn set_packet_timeout_millis(&mut self, msec: u64) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = Duration::from_millis(msec);
    }

    // get the current time
    pub fn get_current_time(&self) -> Option<Instant> {
        return Some(Instant::now());
    }

    // is still timeout
    pub fn is_packet_timeout(&mut self) -> bool {
        if self.get_time_since_start() > self.packet_timeout {
            self.packet_timeout = Duration::new(0, 0);
            return true;
        }
        return false;
    }

    // get the time since from the start of the port
    pub fn get_time_since_start(&mut self) -> Duration {
        match (self.get_current_time(), self.packet_start_time) {
            (Some(now), Some(start)) => now - start,
            _ => {
                // 初始化时间为当前时间
                self.packet_start_time = self.get_current_time();
                Duration::new(0, 0)
            }
        }
    }

    // setup the port
    pub fn setup_port(&mut self) -> Result<(), serialport::Error> {
        if self.is_open {
            self.close_port()?
        }

        let port = serialport::new(&self.port_name, self.baudrate)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_millis(100))
            .data_bits(DataBits::Eight)
            .open()?;

        port.clear(ClearBuffer::Input)?;

        self.ser = Some(port);

        self.is_open = true;
        self.tx_time_per_byte = Duration::from_secs_f64(10.0 / self.baudrate as f64);
        Ok(())
    }

    // setup the baudrate
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), serialport::Error> {
        let valid_baud = self
            .get_c_flag_baud(baudrate)
            .ok_or(serialport::Error::new(
                serialport::ErrorKind::InvalidInput,
                "Invalid baudrate",
            ))?; // return Error if we got wrong baudrate

        self.baudrate = valid_baud;
        if self.is_open {
            self.setup_port()?; // 重新配置端口
        }
        Ok(())
    }

    // get the flag baud
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

// destructor for PortHandler
impl Drop for PortHandler {
    fn drop(&mut self) {
        let _ = self.close_port();
    }
}
