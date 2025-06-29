use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::{protocol_packet_handler::ProtocolPacketHandler, scservo_def::COMM};

#[derive(Debug)]
pub struct GroupSyncRead {
    pub ph: ProtocolPacketHandler,
    start_address: u32,
    data_length: u32,

    last_result: bool,
    is_param_changed: bool,
    param: Vec<u32>,
    data_dict: HashMap<u32, Vec<u32>>,
}

impl GroupSyncRead {
    pub fn new(ph: ProtocolPacketHandler, start_address: u32, data_length: u32) -> Self {
        Self {
            ph,
            start_address,
            data_length,
            last_result: false,
            is_param_changed: false,
            param: Vec::new(),
            data_dict: HashMap::new(),
        }
    }

    pub fn make_param(&mut self) {
        if !self.data_dict.is_empty() {
            self.param = self.data_dict.keys().cloned().collect();
        }
    }

    pub fn add_param(&mut self, scs_id: u32) -> Result<(), Error> {
        if self.data_dict.contains_key(&scs_id) {
            Err(Error::new(
                ErrorKind::AlreadyExists,
                "SCS ID already exists",
            ))
        } else {
            self.data_dict
                .insert(scs_id, vec![0; self.data_length as usize]);
            self.param.push(scs_id);
            self.is_param_changed = true;
            Ok(())
        }
    }

    pub fn remove_param(&mut self, scs_id: u32) -> Result<(), Error> {
        if self.data_dict.contains_key(&scs_id) {
            self.data_dict.remove(&scs_id);
            self.param.retain(|&x| x != scs_id);
            self.is_param_changed = true;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "param is not found"))
        }
    }

    pub fn clear_param(&mut self) {
        self.data_dict.clear();
        self.param.clear();
    }

    pub fn tx_packet(&mut self) -> COMM {
        if self.data_dict.is_empty() {
            return COMM::NotAvailable;
        }

        if self.is_param_changed || self.param.is_empty() {
            self.make_param();
            self.is_param_changed = false;
        }

        // 修复：传递正确的参数给sync_read_tx
        self.ph.sync_read_tx(self.start_address, self.data_length, self.param.clone())
    }
    
    pub fn rx_packet(&mut self) -> COMM {
        self.last_result = true;

        if self.data_dict.is_empty() {
            return COMM::NotAvailable;
        }

        // 修复：传递正确的参数给sync_read_rx
        let expected_ids: Vec<u32> = self.data_dict.keys().cloned().collect();
        let (mut result, rxpacket) = self.ph.sync_read_rx(&expected_ids, self.data_length);

        if rxpacket.len() >= self.data_length as usize + 6 {
            let data_dict_keys: Vec<u32> = self.data_dict.keys().cloned().collect();
            for scs_id in data_dict_keys {
                let (data, comm) = self.read_rx(&rxpacket, scs_id, self.data_length);
                self.data_dict.insert(scs_id, data);
                result = comm;
                match result {
                    COMM::Success => {}
                    _ => self.last_result = false,
                }
            }
        } else {
            self.last_result = false
        }
        result
    }

    pub fn tx_rx_packet(&mut self) -> COMM {
        let tx_result = self.tx_packet();
        match tx_result {
            COMM::Success => self.rx_packet(),
            _ => tx_result,
        }
    }

    pub fn read_rx(&self, rxpacket: &[u32], scs_id: u32, data_length: u32) -> (Vec<u32>, COMM) {
        let mut data: Vec<u32> = Vec::new();
        let rx_length = rxpacket.len();
        let mut rx_index = 0;

        while (rx_index + 6 + data_length) as usize <= rx_length {
            let mut headpacket = vec![0x00, 0x00, 0x00];
            while rx_index < rx_length as u32 {
                headpacket[2] = headpacket[1];
                headpacket[1] = headpacket[0];
                headpacket[0] = rxpacket[rx_index as usize];
                rx_index += 1;
                if (headpacket[2] == 0xFF) && (headpacket[1] == 0xFF) && (headpacket[0] == scs_id) {
                    break;
                }
            }
            if (rx_index + 3 + data_length) as usize > rx_length {
                break;
            }
            if rxpacket[rx_index as usize] != data_length + 2 {
                rx_index += 1;
                continue;
            }
            rx_index += 1;
            let error = rxpacket[rx_index as usize];
            let mut cal_sum = scs_id + data_length + 2 + error;
            data.push(error);
            data.extend_from_slice(&rxpacket[rx_index as usize..(rx_index + data_length) as usize]);
            for _ in 0..data_length {
                cal_sum += rxpacket[rx_index as usize];
                rx_index += 1;
            }
            cal_sum = !cal_sum & 0xFF;
            if cal_sum != rxpacket[rx_index as usize] {
                return (Vec::new(), COMM::RxCorrupt);
            }
            return (data, COMM::Success);
        }
        (Vec::new(), COMM::RxCorrupt)
    }
    
    pub fn is_available(&self, scs_id: u32, address: u32, data_length: u32) -> (bool, u32) {
        if !self.data_dict.contains_key(&scs_id) {
            return (false, 0);
        }

        if (address < self.start_address)
            || (self.start_address + self.data_length - data_length < address)
        {
            return (false, 0);
        }

        let data = self.data_dict.get(&scs_id);

        match data {
            Some(data) => {
                if data.len() < data_length as usize + 1 {
                    (false, 0)
                } else {
                    (true, data[0])
                }
            }
            None => (false, 0),
        }
    }

    pub fn get_data(&self, scs_id: u32, address: u32, data_length: u32) -> Option<u32> {
        let index = (address - self.start_address + 1) as usize;

        if data_length == 1 {
            self.data_dict.get(&scs_id).and_then(|data| data.get(index).copied())
        } else {
            None
        }
    }
}
