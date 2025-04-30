use serialport::SerialPortBuilder;

use std::time::Duration;

const DEFAULT_BAUDRATE: u32 = 1000000;
const LATENCY_TIMER: u32 = 50;

#[derive(Debug)]
pub struct PortHandler {
    is_open: bool,
    baudrate: u32,
    packet_start_time: f32,
    packet_timeout: f32,
    tx_time_per_byte: f32,

    is_using: bool,
    port_name: String,
    // use SerialPortBuilder
    ser: Option<SerialPortBuilder>,
}

impl PortHandler {
    pub fn new(port_name: String) -> Self {
        Self {
            is_open: false,
            baudrate: DEFAULT_BAUDRATE,
            packet_start_time: 0.0,
            packet_timeout: 0.0,
            tx_time_per_byte: 0.0,

            is_using: false,
            port_name: port_name,
            ser: None,
        }
    }

    pub fn open_port(&mut self) -> bool {
        return self.set_baudrate(self.baudrate);
    }

    pub fn close_port(&mut self) {
        self.is_open = false;
    }

    pub fn clear_port(&mut self) {
        if let Some(serport) = self.ser.clone() {
            match serport.open() {
                Ok(mut port) => port.flush().unwrap(),
                Err(_) => eprintln!("cannot open"),
            }
        }
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

    pub fn write_port(&self, packet: SerialPortBuilder) {}

    pub fn set_packet_timeout(&self, packet_length: u32) {}

    pub fn set_packet_timeout_millis(&self, msec: u32) {}

    pub fn is_packet_timeout(&self) -> bool {
        return true;
    }

    pub fn get_time_since_start(&self) -> u32 {
        return 0;
    }

    pub fn setup_port(&mut self, cflag_baud: u32) -> bool {
        if self.is_open {
            self.close_port();
        }

        self.ser = Some(
            serialport::new(self.port_name.clone(), self.baudrate).timeout(Duration::from_secs(0)),
        );

        self.is_open = false;

        // reset input buffer

        self.tx_time_per_byte = (1000.0 / self.baudrate as f32) * 10.0;
        true
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
