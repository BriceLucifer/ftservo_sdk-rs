use nix::libc::termios;

#[derive(Debug, Clone)]
pub struct SCSerial {
    /// 输入输出超时
    pub io_time_out: usize,
    /// 错误码
    pub err: i32,
    /// serial port handle
    pub fd: i32,
    /// fd ort opt
    pub org_opt: termios,
    /// fd cur opt
    pub cur_opt: termios,
    /// tx buffer
    pub tx_buf: Vec<u8>,
    /// tx buffer length
    pub tx_buf_len: i32,
}

impl SCSerial {
    /// 初始化函数
    pub fn new(end: u8, level: u8) -> Self {
        todo!()
    }
    /// 输出nlen字节
    pub fn write_scs(&self, n_dat: &[u8], n_len: i32) -> i32 {
        todo!()
    }
    /// 输入nlen字节
    pub fn read_scs(&self, n_dat: &mut [u8], n_len: i32) -> i32 {
        todo!()
    }
    /// 清空接收缓冲区
    pub fn r_flush_scs(&self) {
        todo!()
    }
    /// 清空发送缓冲区
    pub fn w_flush_scs(&self) {
        todo!()
    }
    /// 获取错误码
    pub fn get_err(&self) -> i32 {
        self.err
    }
    /// 设置波特率
    pub fn set_baud_rate(&self, baud_rate: i32) -> i32 {
        todo!()
    }
    /// 开始串口通信
    pub fn begin(&self, baud_rate: i32, serial_port: &str) -> bool {
        todo!()
    }
    /// 结束串口通信
    pub fn end(&self) {
        todo!()
    }
}
