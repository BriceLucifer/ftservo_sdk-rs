use std::collections::HashMap;

#[derive(Debug)]
pub struct GroupSyncRead {
    ph: u32,
    start_address: u32,
    data_length: u32,

    last_result: bool,
    is_param_changed: bool,
    param: Vec<u32>,
    data_dict: HashMap<u32, Vec<u32>>,
}

impl GroupSyncRead {
    pub fn new() {}
    pub fn make_param(&self) {}
    pub fn add_param(&self, scs_id: u32) {}
    pub fn remove_param(&self, scs_id: u32) {}
    pub fn clear_param(&self) {}
    pub fn tx_packet(&self) {}
    pub fn rx_packet(&self) {}
    pub fn tx_rx_packet(&self) {}
    pub fn read_rx(&self) {}
    pub fn is_available(&self) {}
    pub fn get_data(&self) {}
}
