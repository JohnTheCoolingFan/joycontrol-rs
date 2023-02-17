use crate::{button_state::ButtonState, controller::Controller, memory::FlashMemory};

#[derive(Debug)]
pub struct ControllerState {
    protocol: ControllerProtocol, // TODO: find out type
    controller: Controller,
    nfc_content: Option<()>, // TODO: find out type
    spi_flash: Option<FlashMemory>,
    button_state: ButtonState,
}
