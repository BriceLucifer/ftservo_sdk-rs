pub struct PortHandler<T> {
    is_open: bool,
    baudrate: u32,
    packet_start_time: f32,
    packet_timeout: f32,
    tx_time_per_byte: f32,

    is_using: bool,
    port_name: String,
    ser: Option<T>,
}
