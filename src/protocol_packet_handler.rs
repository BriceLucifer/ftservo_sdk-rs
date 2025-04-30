use crate::port_handler::PortHandler;

const TXPACKET_MAX_LEN: usize = 250;
const RXPACKET_MAX_LEN: usize = 250;

// for Protocol Packet
const HEADER0: u8 = 0;
const HEADER1: u8 = 1;
const ID: u8 = 2;
const LENGTH: u8 = 3;
const INSTRUCTION: u8 = 4;
const ERROR: u8 = 4;
const PARAMETER0: u8 = 5;

// Protocal Error bit
const ERRBIT_VOLTAGE: u8 = 1;
const ERRBIT_ANGLE: u8 = 2;
const ERRBIT_OVERHEAT: u8 = 4;
const ERRBIT_OVERELE: u8 = 8;
const ERRBIT_OVERLOAD: u8 = 32;

#[derive(Debug)]
pub struct ProtocolPacketHandler {
    port_handler: PortHandler,
    scs_end: i32,
}
