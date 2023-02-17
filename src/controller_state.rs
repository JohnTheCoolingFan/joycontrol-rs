use crate::{button_state::ButtonState, controller::Controller, mcu::NFCTag, memory::FlashMemory};

#[derive(Debug)]
pub struct ControllerState {
    protocol: ControllerProtocol,
    controller: Controller,
    nfc_content: Option<NFCTag>,
    spi_flash: Option<FlashMemory>,
    button_state: ButtonState,
}
