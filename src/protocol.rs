#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Standard,
    GripMenu,
    AwaitingMaxSlots,
}
