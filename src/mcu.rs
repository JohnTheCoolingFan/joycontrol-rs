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
