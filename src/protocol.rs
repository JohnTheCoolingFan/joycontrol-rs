use crate::{
    controller::Controller,
    controller_state::ControllerState,
    memory::{FlashMemory, SizeMismatch},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Standard,
    GripMenu,
    AwaitingMaxSlots,
}

#[derive(Debug)]
pub struct ControllerProtocol {
    controller: Controller,
    controller_state: ControllerState,
    spi_flash: Option<FlashMemory>,
    is_pairing: bool,
}

impl ControllerProtocol {
    pub async fn send_controller_state(&mut self) {
        todo!()
    }

    pub fn new(
        controller: Controller,
        spi_flash: Option<FlashMemory>,
        reconnect: Option<bool>,
    ) -> Result<Self, SizeMismatch> {
        Ok(Self {
            controller,
            spi_flash: spi_flash.clone(),
            is_pairing: !reconnect.unwrap_or(false),
            controller_state: ControllerState::new(controller, spi_flash),
        })
        // TODO
    }
}
