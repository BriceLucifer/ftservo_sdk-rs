use serialport::SerialPortBuilder;

use std::io::Error;

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

    pub fn close_port(&mut self) {
        self.is_open = false;
    }

    pub fn clear_port(&mut self) {
        if let Some(mut serport) = self.ser.clone() {
            match serport.open() {
                Ok(mut port) => port.flush().unwrap(),
                Err(_) => eprintln!("cannot open"),
        }
    }

    pub fn set_baudrate(&mut self, baudrate: u32) -> bool{
        let bauld = self.get_c_flag_baud(baudrate);

        if let Some(baud) = bauld {
            self.baudrate = baud;
            return self.setup_port(baud)
        } else {
            false
        }
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }

    pub fn set_port_name(&mut self, port_name: String) {
        self.port_name = port_name;
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
