use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::{protocol_packet_handler::ProtocolPacketHandler, scservo_def::COMM};

#[derive(Debug)]
pub struct GroupSyncRead {
    ph: ProtocolPacketHandler,
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
            ph: ph,
            start_address: start_address,
            data_length: data_length,

            last_result: false,
            is_param_changed: false,
            param: Vec::new(),
            data_dict: HashMap::new(),
        }
    }
    pub fn make_param(&mut self) {
        if self.data_dict.is_empty() {
            self.param = self.data_dict.keys().map(|x| x.clone()).collect();
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
            let _ = self.data_dict.remove(&scs_id);
            self.is_param_changed = true;
            return Ok(());
        }
        Err(Error::new(ErrorKind::NotFound, "param is not found"))
    }

    pub fn clear_param(&mut self) {
        self.data_dict.clear();
    }
    pub fn tx_packet(&mut self) {}
    pub fn rx_packet(&self) {}
    pub fn tx_rx_packet(&self) {}
    pub fn read_rx(&self) {}
    pub fn is_available(&self, scs_id: u32, address: u32, data_length: u32) -> (bool, u32) {
        if self.data_dict.contains_key(&scs_id) {
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
                    return (false, 0);
                } else {
                    return (true, data[0]);
                }
            }
            None => {
                return (false, 0);
            }
        }
    }
    pub fn get_data(&self, scs_id: u32, address: u32, data_length: u32) -> Option<u32> {
        let index = (address - self.start_address + 1) as usize;
        if data_length == 1 {
            return Some(self.data_dict[&scs_id][index]);
        }
        None
    }
}
