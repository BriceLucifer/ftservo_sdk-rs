use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::{protocol_packet_handler::ProtocolPacketHandler, scservo_def::COMM};

#[derive(Debug)]
pub struct GroupSyncWrite {
    pub ph: ProtocolPacketHandler,
    pub start_address: u32,
    data_length: u32,

    is_param_changed: bool,
    param: Vec<u32>,
    data_dict: HashMap<u32, Vec<u32>>,
}

impl GroupSyncWrite {
    pub fn new(ph: ProtocolPacketHandler, start_address: u32, data_length: u32) -> Self {
        Self {
            ph,
            start_address,
            data_length,
            is_param_changed: false,
            param: Vec::new(),
            data_dict: HashMap::new(),
        }
    }

    pub fn make_param(&mut self) {
        self.param.clear();
        
        if self.data_dict.is_empty() {
            return;
        }

        for (&scs_id, data) in &self.data_dict {
            if data.is_empty() {
                return;
            }

            self.param.push(scs_id);
            self.param.extend(data.iter());
        }
    }

    pub fn add_param(&mut self, scs_id: u32, data: Vec<u32>) -> Result<(), Error> {
        if self.data_dict.contains_key(&scs_id) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "scs_id already exists",
            ));
        }

        if data.len() > self.data_length as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "data length is too long",
            ));
        }

        self.data_dict.insert(scs_id, data);
        self.is_param_changed = true;
        Ok(())
    }

    pub fn remove_param(&mut self, scs_id: u32) -> Result<(), Error> {
        if self.data_dict.remove(&scs_id).is_none() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "can not find in the data dict",
            ));
        }
        self.is_param_changed = true;
        Ok(())
    }

    pub fn change_param(&mut self, scs_id: u32, data: Vec<u32>) -> Result<(), Error> {
        if !self.data_dict.contains_key(&scs_id) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "can not find scs_id in the data dict",
            ));
        }

        if data.len() > self.data_length as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "data length is too long",
            ));
        }

        self.data_dict.insert(scs_id, data);
        self.is_param_changed = true;
        Ok(())
    }

    pub fn clear_param(&mut self) {
        self.data_dict.clear();
        self.param.clear();
        self.is_param_changed = true;
    }

    pub fn tx_packet(&mut self) -> COMM {
        if self.data_dict.is_empty() {
            return COMM::NotAvailable;
        }

        if self.is_param_changed {
            self.make_param();
            self.is_param_changed = false;
        }
        
        self.ph.sync_write_tx_only(
            self.start_address,
            self.data_length,
            self.param.clone(),
            (self.data_dict.len() * (1 + self.data_length as usize)) as u32,
        )
    }
}
