use std::collections::HashMap;

use crate::port_handler::PortHandler;

#[derive(Debug)]
pub struct GroupSyncWrite {
    ph: PortHandler,
    start_address: u32,
    data_length: u32,

    is_param_changed: bool,
    param: Vec<u32>,
    data_dict: HashMap<u32, Vec<u32>>,
}

impl GroupSyncWrite {
    pub fn new() {}
    pub fn make_param(&self) {}
    pub fn add_param(&self) {}
    pub fn remove_param(&self) {}
    pub fn change_param(&self) {}
    pub fn clear_param(&self) {}
    pub fn tx_packet(&self) {}
}
