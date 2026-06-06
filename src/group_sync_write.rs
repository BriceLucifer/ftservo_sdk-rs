use std::{
    collections::BTreeMap,
    io::{Error, ErrorKind},
};

use crate::{protocol_packet_handler::ProtocolPacketHandler, scservo_def::COMM};

#[derive(Debug)]
pub struct GroupSyncWrite {
    pub start_address: u32,
    data_length: u32,

    is_param_changed: bool,
    param: Vec<u32>,
    data_dict: BTreeMap<u32, Vec<u32>>,
}

impl GroupSyncWrite {
    pub fn new(start_address: u32, data_length: u32) -> Self {
        Self {
            start_address,
            data_length,
            is_param_changed: false,
            param: Vec::new(),
            data_dict: BTreeMap::new(),
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
            self.param
                .resize(self.param.len() + self.data_length as usize - data.len(), 0);
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

    pub fn tx_packet(&mut self, ph: &mut ProtocolPacketHandler) -> COMM {
        if self.data_dict.is_empty() {
            return COMM::NotAvailable;
        }

        if self.is_param_changed {
            self.make_param();
            self.is_param_changed = false;
        }

        ph.sync_write_tx_only(
            self.start_address,
            self.data_length,
            self.param.clone(),
            (self.data_dict.len() * (1 + self.data_length as usize)) as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_param_orders_ids_and_pads_short_data() {
        let mut group = GroupSyncWrite::new(42, 3);

        group.add_param(3, vec![30, 31]).unwrap();
        group.add_param(1, vec![10, 11, 12]).unwrap();
        group.add_param(2, vec![20]).unwrap();

        group.make_param();

        assert_eq!(group.param, vec![1, 10, 11, 12, 2, 20, 0, 0, 3, 30, 31, 0]);
    }

    #[test]
    fn clear_param_resets_cached_payload() {
        let mut group = GroupSyncWrite::new(42, 2);

        group.add_param(1, vec![10, 11]).unwrap();
        group.make_param();
        assert!(!group.param.is_empty());

        group.clear_param();

        assert!(group.param.is_empty());
        assert!(group.data_dict.is_empty());
    }
}
