#[derive(Debug, Clone)]
pub struct SCS {
    /// 舵机返回等级
    pub level: u8,
    /// 处理器大小端结构
    pub end: u8,
    /// 舵机状态
    pub error: u8,
    pub sync_read_rx_packet_index: u8,
    pub sync_read_rx_packet_len: u8,
    pub sync_read_rx_packet_packet: Vec<u8>,
    pub sync_read_rx_packet_buffer: Vec<u8>,
    pub sync_read_rx_packet_buff_len: u16,
    pub sync_read_rx_packet_buff_max: u16,
}

impl SCS {
    pub fn new(end: u8, level: u8) -> Self {
        Self {
            level: level,
            end: end,
            error: 0,
            sync_read_rx_packet_index: 0,
            sync_read_rx_packet_len: 0,
            sync_read_rx_packet_packet: Vec::new(),
            sync_read_rx_packet_buffer: Vec::new(),
            sync_read_rx_packet_buff_len: 0,
            sync_read_rx_packet_buff_max: 0,
        }
    }

    pub fn gen_write(&self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) -> i32 {
        todo!()
    }
}
