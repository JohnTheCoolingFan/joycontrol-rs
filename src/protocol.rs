use crate::{
    controller::Controller,
    controller_state::ControllerState,
    memory::{FlashMemory, SizeMismatch},
};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use std::iter::FromIterator;

lazy_static! {
    pub static ref DELAY_MAP: HashMap<u8, f32> = HashMap::from_iter(vec![
        (0x3F, 1.0),
        (0x21, f32::INFINITY),
        (0x30, 1.0 / 60.0),
        (0x31, 1.0 / 60.0)
    ]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Standard,
    GripMenu,
    AwaitingMaxSlots,
}

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
