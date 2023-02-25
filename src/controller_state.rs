use crate::{
    button_state::ButtonState, controller::Controller, memory::FlashMemory, nfc_tag::NFCTag,
    stick_state::StickState,
};

// Protocol is ignored for now because that causes cyclic referencing or self-referencing, which is
// hard  and there is probably a better way to do what the original code does
#[derive(Debug)]
pub struct ControllerState {
    controller: Controller,
    nfc_content: Option<NFCTag>,
    spi_flash: Option<FlashMemory>,
    button_state: ButtonState,
    l_stick_state: Option<StickState>,
    r_stick_state: Option<StickState>,
}

impl ControllerState {
    pub fn new(controller: Controller, spi_flash: Option<FlashMemory>) -> Self {
        todo!()
    }
}
