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
    pub sync_read_rx_buffer: Vec<u8>,
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
            sync_read_rx_buffer: Vec::new(),
            sync_read_rx_packet_buff_len: 0,
            sync_read_rx_packet_buff_max: 0,
        }
    }
    /// 普通写指令
    /// id:舵机id, memaddr:内存表地址, n_dat:写入数据, n_len:数据长度
    pub fn gen_write(&mut self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, n_dat, n_len, INST::INST_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 异步写指令
    /// id:舵机id, memaddr:内存表地址, n_dat:写入数据, n_len:数据长度
    pub fn reg_write(&mut self, id: u8, mem_addr: u8, n_dat: &[u8], n_len: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, n_dat, n_len, INST::INST_REG_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 异步写执行指令
    pub fn reg_write_action(&mut self, id: u8) -> i32 {
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
    pub fn write_byte(&mut self, id: u8, mem_addr: u8, b_dat: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, &[b_dat], 1, INST::INST_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 写2个字节
    pub fn write_word(&mut self, id: u8, mem_addr: u8, w_dat: u16) -> i32 {
        let mut buf_item = (0, 0);
        self.host_2_scs(&mut buf_item.0, &mut buf_item.1, w_dat);
        let b_buf = [buf_item.0, buf_item.1];
        self.r_flush_scs();
        self.write_buf(id, mem_addr, &b_buf, 2, INST::INST_WRITE);
        self.w_flush_scs();
        return self.ack(id);
    }
    /// 读指令
    pub fn read(&mut self, id: u8, mem_addr: u8, n_data: &mut [u8], n_len: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, mem_addr, &[n_len], n_len, INST::INST_READ);
        self.w_flush_scs();

        let mut b_buf = [0u8; 255];
        let size = self.read_scs(&mut b_buf, n_len + 6);

        if size != n_len as i32 + 6 {
            return 0;
        }

        if b_buf[0] != 0xff || b_buf[1] != 0xff {
            return 0;
        }

        let mut cal_sum = 0;
        for i in 2..size as usize - 1 {
            cal_sum += b_buf[i];
        }
        cal_sum = !cal_sum;
        if cal_sum != b_buf[size as usize - 1] {
            return 0;
        }
        n_data.copy_from_slice(&b_buf[5..]);
        self.error = b_buf[4];
        return n_len as i32;
    }
    /// 读1个字节
    pub fn read_byte(&mut self, id: u8, mem_addr: u8) -> i32 {
        let mut buf = [0u8]; // 栈上数组
        let size = self.read(id, mem_addr, &mut buf, 1);
        if size != 1 { -1 } else { buf[0] as i32 }
    }
    /// 读2个字节
    pub fn read_word(&mut self, id: u8, mem_addr: u8) -> i32 {
        let mut n_dat = [0u8; 2];
        let size = self.read(id, mem_addr, &mut n_dat, 2);
        if size != 2 {
            return -1;
        }
        let w_dat = self.scs_2_host(n_dat[0], n_dat[1]);
        return w_dat as i32;
    }
    /// ping指令
    pub fn ping(&mut self, id: u8) -> i32 {
        self.r_flush_scs();
        self.write_buf(id, 0, &[], 0, INST::INST_PING);
        self.w_flush_scs();

        let mut cal_sum = 0;
        let mut b_buf = [0u8; 6];
        let size = self.read_scs(&mut b_buf, 6);
        if size != 6 {
            return -1;
        }
        if b_buf[0] != 0xff || b_buf[1] != 0xff {
            return -1;
        }
        if b_buf[2] != id || id != 0xfe {
            return -1;
        }
        if b_buf[3] != 2 {
            return -1;
        }
        for i in 2..size as usize - 1 {
            cal_sum += b_buf[i];
        }
        cal_sum = !cal_sum;
        if cal_sum != b_buf[size as usize - 1] {
            return -1;
        }

        self.error = b_buf[2];
        return self.error as i32;
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
    pub fn sync_read_begin(&mut self, idn: u8, rx_len: u8) {
        self.sync_read_rx_packet_buff_max = (idn * (rx_len + 6)) as u16;
        self.sync_read_rx_buffer = vec![0u8; self.sync_read_rx_packet_buff_max as usize];
    }
    /// 同步读指令包结束
    pub fn sync_read_end(&self) {
        // 其实这个函数貌似没啥必要。。。 因为会自动释放内存
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
    pub fn ack(&mut self, id: u8) -> i32 {
        let mut b_buf = [0u8; 6];
        self.error = 0;
        let mut cal_sum = 0;
        if id != 0xfe && self.level != 0 {
            let size = self.read_scs(&mut b_buf, 6);
            if size != 6 {
                return 0;
            }
            if b_buf[0] != 0xff || b_buf[1] != 0xff || b_buf[2] != id {
                return 0;
            }
            if b_buf[3] != 2 {
                return 0;
            }
            for i in 2..size as usize - 1 {
                cal_sum += b_buf[i];
            }
            cal_sum = !cal_sum;
            if (cal_sum != b_buf[size as usize - 1]) {
                return 0;
            }
            self.error = b_buf[4];
        }
        return 1;
    }

    // 拆分u16
    pub fn host_2_scs(&self, data_l: &mut u8, data_h: &mut u8, data: u16) {
        if self.end != 0 {
            *data_l = (data >> 8) as u8;
            *data_h = (data & 0xff) as u8;
        } else {
            *data_h = (data >> 8) as u8;
            *data_l = (data & 0xff) as u8;
        }
    }
    // 合并(u8, u8) -> u16
    pub fn scs_2_host(&self, data_l: u8, data_h: u8) -> u16 {
        todo!()
    }

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
