use std::sync::Arc;

use rsevents::{EventState, ManualResetEvent};

use crate::{
    button_state::ButtonState, controller::Controller, memory::FlashMemory, nfc_tag::NFCTag,
    stick_calibration::StickCalibration, stick_state::StickState,
};

// Protocol is ignored for now because that causes cyclic referencing or self-referencing, which is
// hard and there is probably a better way to do what the original code does
pub struct ControllerState {
    controller: Controller,
    nfc_content: Option<NFCTag>,
    spi_flash: Option<FlashMemory>,
    button_state: ButtonState,
    l_stick_state: Option<StickState>,
    r_stick_state: Option<StickState>,
    sig_is_send: Arc<ManualResetEvent>,
}

impl ControllerState {
    pub fn new(controller: Controller, spi_flash: Option<FlashMemory>) -> Self {
        let button_state = ButtonState::new(controller);

        let l_stick_state: Option<StickState> =
            (matches!(controller, Controller::ProController | Controller::JoyconL)).then(|| {
                let calibration = if let Some(flash) = &spi_flash {
                    let calibration_data = flash
                        .get_user_l_stick_calibration()
                        .unwrap_or_else(|| flash.get_factory_l_stick_calibration());
                    Some(StickCalibration::l_from_bytes(
                        &calibration_data.try_into().unwrap(),
                    ))
                } else {
                    None
                };
                let mut l_stick_state = StickState::new(None, None, calibration).unwrap();
                let _ = l_stick_state.set_center(); // Ignoring the error that would happen if
                                                    // calibration is None
                l_stick_state
            });

        let r_stick_state: Option<StickState> =
            (matches!(controller, Controller::ProController | Controller::JoyconR)).then(|| {
                let calibration = if let Some(flash) = &spi_flash {
                    let calibration_data = flash
                        .get_user_r_stick_calibration()
                        .unwrap_or_else(|| flash.get_factory_r_stick_calibration());
                    Some(StickCalibration::r_from_bytes(
                        &calibration_data.try_into().unwrap(),
                    ))
                } else {
                    None
                };
                let mut r_stick_state = StickState::new(None, None, calibration).unwrap();
                let _ = r_stick_state.set_center(); // Ignoring the error that would happen if
                                                    // calibration is None
                r_stick_state
            });

        Self {
            controller,
            nfc_content: None,
            spi_flash,
            button_state,
            l_stick_state,
            r_stick_state,
            sig_is_send: Arc::new(ManualResetEvent::new(EventState::Unset)),
        }
    }

    pub fn get_controller(&self) -> Controller {
        self.controller
    }

    pub fn get_flash_memory(&self) -> Option<&FlashMemory> {
        self.spi_flash.as_ref()
    }

    pub fn set_nfc(&mut self, data: NFCTag) {
        self.nfc_content = Some(data)
    }

    pub fn get_nfc(&self) -> Option<&NFCTag> {
        self.nfc_content.as_ref()
    }
}
