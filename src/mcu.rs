use crc::{Crc, CRC_8_SMBUS};
use lazy_static::lazy_static;
use log::warn;
use uuid::Uuid;

use crate::{controller_state::ControllerState, nfc_tag::NFCTag};

lazy_static! {
    pub static ref REMOVE_AMIIBO: NFCTag = NFCTag::new(&[0; 540], None, None);
    pub static ref NO_RESPONSE_MESSAGE: Vec<u8> = pack_message(&[0xFF], None, None, None);
}

const MAX_RESPONSE_QUEUE_LEN: usize = 4;

const SET_POWER_VALUES: [MCUPowerState; 2] = [MCUPowerState::Suspended, MCUPowerState::Ready];
const SET_CONFIG_VALUES: [MCUPowerState; 2] = [MCUPowerState::Ready, MCUPowerState::ConfiguredNFC];
const GET_STATUS_VALUES: [MCUPowerState; 2] = [MCUPowerState::Ready, MCUPowerState::ConfiguredNFC];

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

pub struct MicroControllerUnit {
    pub power_state: MCUPowerState,
    pub nfc_state: NFCState,
    pub nfc_counter: i32,
    last_poll_uid: Option<[u8; 6]>,
    pending_active_remove: u32,
    pub remove_nfc_after_write: bool,
    controller: ControllerState,
    pub seq_no: u32,
    pub ack_seq_no: u32,
    pub received_data: Vec<u8>,  // FIXME: this is probably some other type
    pub response_queue: Vec<u8>, // FIXME: this is probably some other type
}

impl MicroControllerUnit {
    pub fn new(controller: ControllerState) -> Self {
        Self {
            controller,
            power_state: MCUPowerState::Suspended,
            nfc_state: NFCState::None,
            nfc_counter: 0,
            last_poll_uid: None,
            pending_active_remove: 0,
            remove_nfc_after_write: true,
            seq_no: 0,
            ack_seq_no: 0,
            received_data: vec![],
            response_queue: vec![],
        }
    }

    fn flush_response_queue(&mut self) {
        self.response_queue = vec![]
    }

    fn queue_response(&mut self, resp: &[u8]) {
        if self.response_queue.len() < MAX_RESPONSE_QUEUE_LEN {
            self.response_queue.extend_from_slice(resp)
        } else {
            warn!("Full queue, dropped outgoing MCU packet")
        }
    }

    fn force_queue_response(&mut self, resp: &[u8]) {
        self.response_queue.extend_from_slice(resp);
        if self.response_queue.len() > MAX_RESPONSE_QUEUE_LEN {
            warn!("Forced response queue")
        }
    }

    pub fn set_remove_nfc_after_read(&mut self, value: bool) {
        // self.remove_nfc_after_write = value
    }

    fn get_status_data(&self) -> Option<Vec<u8>> {
        if matches!(self.power_state, MCUPowerState::Suspended) {
            warn!("MCU: status request when disabled");
            Some(NO_RESPONSE_MESSAGE.to_vec())
        } else if GET_STATUS_VALUES.contains(&self.power_state) {
            Some(pack_message(
                &[
                    hex::decode("0100000008001b").unwrap().as_slice(),
                    &[self.power_state as u8],
                ]
                .concat(),
                None,
                None,
                None,
            ))
        } else {
            None
        }
    }

    fn get_nfc_status_data(&mut self) -> Vec<u8> {
        self.nfc_counter -= 1;
        let mut nfc_tag = self.controller.get_nfc();

        if [NFCState::Poll, NFCState::PollAgain].contains(&self.nfc_state)
            && (self.remove_nfc_after_write || !nfc_tag.is_some())
            && (self.pending_active_remove > 0)
        {
            nfc_tag = Some(&REMOVE_AMIIBO);
            self.pending_active_remove -= 1;
        }

        if matches!(self.nfc_state, NFCState::ProcessingWrite) && self.nfc_counter <= 0 {
            self.nfc_state = NFCState::None
        } else if matches!(self.nfc_state, NFCState::Poll) {
            if let Some(nfc_tag) = nfc_tag {
                if Some(nfc_tag.get_uid()) == self.last_poll_uid {
                    self.nfc_state = NFCState::PollAgain
                } else {
                    self.last_poll_uid = Some(nfc_tag.get_uid())
                }
            } else {
                self.last_poll_uid = None
            }
        } else if matches!(self.nfc_state, NFCState::PollAgain) {
            if let Some(nfc_tag) = &nfc_tag {
                if Some(nfc_tag.get_uid()) != self.last_poll_uid {
                    self.nfc_state = NFCState::Poll;
                    self.last_poll_uid = Some(nfc_tag.get_uid());
                }
            } else {
                self.nfc_state = NFCState::Poll;
                self.last_poll_uid = None;
            }
        }

        if let Some(nfc_tag) = &nfc_tag {
            if !matches!(self.nfc_state, NFCState::None) {
                pack_message(
                    &[
                        hex::decode("2a0005").unwrap().as_slice(),
                        &[self.seq_no as u8],
                        hex::decode("0931").unwrap().as_slice(),
                        &[self.nfc_state as u8],
                        hex::decode("0000000101020007").unwrap().as_slice(),
                        &nfc_tag.get_uid(),
                    ]
                    .concat(),
                    None,
                    None,
                    None,
                )
            } else {
                pack_message(
                    &[
                        hex::decode("2a000500000931").unwrap().as_slice(),
                        &[self.nfc_state as u8],
                    ]
                    .concat(),
                    None,
                    None,
                    None,
                )
            }
        } else {
            pack_message(
                &[
                    hex::decode("2a000500000931").unwrap().as_slice(),
                    &[self.nfc_state as u8],
                ]
                .concat(),
                None,
                None,
                None,
            )
        }
    }
}
