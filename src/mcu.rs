use crc::{Crc, CRC_8_SMBUS};

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
