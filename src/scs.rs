use crate::inst::INST;

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
    /// 创建一个新的SCS实例
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
    /// 普通写指令
    /// id:舵机id, memaddr:内存表地址, n_dat:写入数据, n_len:数据长度
    pub fn gen_write(&self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, n_dat, n_len, INST::INST_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 异步写指令
    /// id:舵机id, memaddr:内存表地址, n_dat:写入数据, n_len:数据长度
    pub fn reg_write(&self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, n_dat, n_len, INST::INST_REG_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 异步写执行指令
    pub fn reg_write_action(&self, id: u8) -> i32 {
        let mut id = id;
        if id == 0x0 {
            id = 0xfe;
        }
        self.r_flush_scs();
        self.write_buf(id, 0, &[], 0, INST::INST_REG_ACTION);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 同步写指令
    pub fn sync_write(&self, id: &[u8], idn: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) {
        self.r_flush_scs();
        let mes_len = (n_len + 1) * idn + 4;
        let b_buf = [
            0xff,
            0xff,
            0xfe,
            mes_len,
            INST::INST_SYNC_WRITE,
            mem_addr,
            n_len,
        ];

        self.write_scs(&b_buf, 7);

        let mut sum = 0xfe + mes_len + INST::INST_SYNC_WRITE + mem_addr + n_len;
        for i in 0..idn as usize {
            self.write_scs1(id[i]);
            let start = i * n_len as usize;
            self.write_scs(&n_dat[start..], n_len);
            sum += id[i];
            for j in 0..n_len as usize {
                sum += n_dat[j + i * n_len as usize];
            }
        }
        self.write_scs1(!sum);
        self.w_flush_scs();
    }
    /// 写1个字节
    pub fn write_byte(&self, id: u8, mem_addr: u8, b_dat: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, &[b_dat], 1, INST::INST_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 写2个字节
    pub fn write_word(&self, id: u8, mem_addr: u8, n_dat: u16) -> i32 {
        todo!()
    }
    /// 读指令
    pub fn read(&self, mem_addr: u8, n_data: &[u8], n_len: u8) -> i32 {
        todo!()
    }
    /// 读1个字节
    pub fn read_byte(&self, id: u8, mem_addr: u8) -> i32 {
        todo!()
    }
    /// 读2个字节
    pub fn read_word(&self, id: u8, mem_addr: u8) -> i32 {
        todo!()
    }
    /// ping指令
    pub fn ping(&self, id: u8) -> i32 {
        todo!()
    }
    /// 同步读指令包发送
    pub fn sync_read_packet_tx(&self, id: &[u8], idn: u8, mem_addr: u8, n_len: u8) -> i32 {
        todo!()
    }
    /// 同步读指令包接收解码，成功返回内存字节数，失败返回0
    pub fn sync_read_packet_rx(&self, id: u8, n_dat: &[u8]) -> i32 {
        todo!()
    }
    /// 解码一个字节
    pub fn sync_read_packet_to_byte(&self) -> i32 {
        todo!()
    }
    /// 解码2个字节，neg_bit: 方向，neg_bit:0 为无方向
    pub fn sync_read_packet_to_word(&self, neg_bit: u8) -> i32 {
        todo!()
    }
    /// 同步读指令包开始
    pub fn sync_read_begin(&self, idn: u8, rx_len: u8) {
        todo!()
    }
    /// 同步读指令包结束
    pub fn sync_read_end(&self) {
        todo!()
    }
    /// 写入buffer
    pub fn write_buf(&self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8, fun: u8) {
        let mut msg_len = 2;
        let mut b_buf = [0u8; 6];
        b_buf[0] = 0xff;
        b_buf[1] = 0xff;
        b_buf[2] = id;
        b_buf[4] = fun;
        if !n_dat.is_empty() {
            msg_len += n_len + 1;
            b_buf[3] = msg_len;
            b_buf[5] = mem_addr;
            self.write_scs(&b_buf, 6);
        } else {
            b_buf[3] = msg_len;
            self.write_scs(&b_buf, 5);
        }
        let mut check_sum = id + msg_len + fun + mem_addr;
        if !n_dat.is_empty() {
            for i in 0..n_len {
                check_sum += n_dat[i as usize];
            }
            self.write_scs(&n_dat, n_len);
        }
        self.write_scs1(!check_sum);
    }
    /// 返回应答
    pub fn ack(&self, id: u8) -> i32 {
        todo!()
    }

    // 拆分u16 -> (u8, u8)
    // 合并(u8, u8) -> u16

    pub fn write_scs(&self, n_dat: &[u8], n_len: u8) -> i32 {
        todo!()
    }
    pub fn read_scs(&self, n_dat: &[u8], n_len: u8) -> i32 {
        todo!()
    }
    pub fn write_scs1(&self, b_dat: u8) -> i32 {
        todo!()
    }
    pub fn r_flush_scs(&self) {
        todo!()
    }
    pub fn w_flush_scs(&self) {
        todo!()
    }
}
