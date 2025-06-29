use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::{
    io::ErrorKind,
    time::{Duration, Instant},
};

// 默认设置
const DEFAULT_BAUDRATE: u32 = 1000000;
const LATENCY_TIMER: u32 = 50;

// PortHandler 结构体
#[derive(Debug)]
pub struct PortHandler {
    port_name: String,
    is_open: bool,
    baudrate: u32,
    // 时间线
    packet_start_time: Option<Instant>,
    packet_timeout: Duration,
    tx_time_per_byte: Duration,

    pub is_using: bool,
    // 使用 SerialPortBuilder
    ser: Option<Box<dyn SerialPort>>,
}

impl PortHandler {
    /// 创建新的 PortHandler
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

    // 打开端口
    pub fn open_port(&mut self) -> Result<(), serialport::Error> {
        self.setup_port()
    }

    // 关闭端口
    pub fn close_port(&mut self) -> Result<(), serialport::Error> {
        if let Some(port) = &mut self.ser {
            port.flush()?;
        }
        self.ser = None;
        self.is_open = false;
        Ok(())
    }

    // 清除传输
    pub fn clear_port(&mut self) -> Result<(), serialport::Error> {
        if let Some(serport) = &mut self.ser {
            serport.clear(ClearBuffer::All)?
        }
        Ok(())
    }

    // 设置新的端口名
    pub fn set_port_name(&mut self, port_name: String) {
        self.port_name = port_name;
    }

    // 获取端口名
    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }

    // 获取波特率
    pub fn get_baudrate(&self) -> u32 {
        self.baudrate
    }

    // 检查端口是否可用
    pub fn get_bytes_available(&self) -> Result<u32, serialport::Error> {
        match &self.ser {
            Some(port) => port.bytes_to_read(),
            None => Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::NotConnected),
                "Port not open",
            )),
        }
    }

    // 读取端口
    pub fn read_port(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        if let Some(port) = &mut self.ser {
            port.read(buf).map_err(|e| e.into())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "can not open",
            ))
        }
    }

    // 通过端口写入
    pub fn write_port(&mut self, packet: &[u8]) -> Result<usize, std::io::Error> {
        if let Some(port) = &mut self.ser {
            port.write(packet)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "can not find",
            ))
        }
    }

    // 设置超时
    pub fn set_packet_timeout(&mut self, packet_length: u32) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = self.tx_time_per_byte * packet_length
            + self.tx_time_per_byte * 3
            + Duration::from_millis(LATENCY_TIMER as u64);
    }

    // 以毫秒为单位设置超时
    pub fn set_packet_timeout_millis(&mut self, msec: u64) {
        self.packet_start_time = self.get_current_time();
        self.packet_timeout = Duration::from_millis(msec);
    }

    // 获取当前时间
    pub fn get_current_time(&self) -> Option<Instant> {
        Some(Instant::now())
    }

    // 是否仍然超时
    pub fn is_packet_timeout(&mut self) -> bool {
        if self.get_time_since_start() > self.packet_timeout {
            self.packet_timeout = Duration::new(0, 0);
            return true;
        }
        false
    }

    // 获取从端口启动以来的时间
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

    // 设置端口
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

    // 设置波特率
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), serialport::Error> {
        let valid_baud = self
            .get_c_flag_baud(baudrate)
            .ok_or(serialport::Error::new(
                serialport::ErrorKind::InvalidInput,
                "Invalid baudrate",
            ))?;

        self.baudrate = valid_baud;
        if self.is_open {
            self.setup_port()?; // 重新配置端口
        }
        Ok(())
    }

    // 获取标志波特率
    pub fn get_c_flag_baud(&self, baudrate: u32) -> Option<u32> {
        let baudrate_list: Vec<u32> = vec![
            4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000, 250000, 500000, 1000000,
        ];
        
        for &baud in &baudrate_list {
            if baud == baudrate {
                return Some(baud);
            }
        }
        None
    }
}

// destructor for PortHandler
impl Drop for PortHandler {
    fn drop(&mut self) {
        let _ = self.close_port();
    }
}
