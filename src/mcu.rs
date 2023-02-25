use crc::{Crc, CRC_8_SMBUS};
use lazy_static::lazy_static;
use log::warn;
use uuid::Uuid;

use crate::{controller::Controller, nfc_tag::NFCTag};

lazy_static! {
    pub static ref REMOVE_AMIIBO: NFCTag = NFCTag::new(&[0; 540], None, None);
    pub static ref NO_RESPONSE_MESSAGE: Vec<u8> = pack_message(&[0xFF], None, None, None);
}

const MAX_RESPONSE_QUEUE_LEN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MCUPowerState {
    Suspended = 0x00,
    Ready = 0x01,
    ReadyUpdate = 0x02,
    ConfiguredNFC = 0x04,
    // ConfiguredIR = 0x05,
    // COnfiguredUpdate = 0x06
}

pub fn mcu_crc(data: &[u8]) -> u8 {
    let crc_calculator = Crc::<u8>::new(&CRC_8_SMBUS);
    crc_calculator.checksum(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NFCState {
    None = 0x00,
    Poll = 0x01,
    PendingRead = 0x02,
    Writing = 0x03,
    AwaitingWrite = 0x04,
    ProcessingWrite = 0x05,
    PollAgain = 0x09,
}

pub fn pack_message(
    data: &[u8],
    background: Option<u8>,
    checksum: Option<fn(&[u8]) -> u8>,
    length: Option<usize>,
) -> Vec<u8> {
    let mut buf: Vec<u8> = data.into();
    buf.resize(length.unwrap_or(313), background.unwrap_or(0));
    if buf.len() > length.unwrap_or(313) {
        warn!("MCU: too long message packed");
    }
    let buf_len_without_checksum: usize = buf.len() - 1;
    buf[buf_len_without_checksum] = checksum.unwrap_or(mcu_crc)(&buf[..(buf_len_without_checksum)]);
    buf
}

#[derive(Debug, Clone)]
pub struct MicroControllerUnit {
    pub power_state: MCUPowerState,
    pub nfc_state: NFCState,
    pub nfc_counter: u32,
    last_poll_uid: Uuid,
    pending_active_remove: u32,
    pub remove_nfc_after_write: bool,
    controller: Controller,
    pub seq_no: u32,
    pub ack_seq_no: u32,
    pub received_data: Vec<u8>,
    pub response_queue: Vec<u8>,
}
