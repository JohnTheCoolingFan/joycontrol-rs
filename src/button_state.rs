use std::{fmt::Display, iter::repeat};

use hashbrown::HashMap;
use thiserror::Error;

use crate::controller::Controller;

#[derive(Debug, Clone)]
pub struct ButtonState {
    pub controller: Controller,
    button_states: HashMap<String, bool>,
}

impl ButtonState {
    const PRO_CONTROLLER_AVAILABLE_BUTTONS: [&'static str; 18] = [
        "y", "x", "b", "a", "r", "zr", "minus", "plus", "r_stick", "l_stick", "home", "capture",
        "down", "up", "right", "left", "l", "zl",
    ];
    const JOYCON_R_AVAILABLE_BUTTONS: [&'static str; 11] = [
        "y", "x", "b", "a", "sr", "sl", "r", "zr", "plus", "r_stick", "home",
    ];
    const JOYCON_L_AVAILABLE_BUTTONS: [&'static str; 11] = [
        "minus", "l_stick", "capture", "down", "up", "right", "left", "sr", "sl", "l", "zl",
    ];

    pub fn new(controller: Controller) -> Self {
        let button_states = match controller {
            Controller::JoyconL => Self::JOYCON_L_AVAILABLE_BUTTONS
                .into_iter()
                .map(Into::into)
                .zip(repeat(false))
                .collect(),
            Controller::JoyconR => Self::JOYCON_R_AVAILABLE_BUTTONS
                .into_iter()
                .map(Into::into)
                .zip(repeat(false))
                .collect(),
            Controller::ProController => Self::PRO_CONTROLLER_AVAILABLE_BUTTONS
                .into_iter()
                .map(Into::into)
                .zip(repeat(false))
                .collect(),
        };
        Self {
            controller,
            button_states,
        }
    }

    pub fn set_button(&mut self, button: &str, pushed: bool) -> Result<(), ButtonNotAvailable> {
        let button = button.to_lowercase();
        if !self.button_available(&button) {
            return Err(ButtonNotAvailable(button, self.controller));
        }
        self.button_states.insert(button, pushed);
        Ok(())
    }

    pub fn get_button(&self, button: &str) -> Result<bool, ButtonNotAvailable> {
        let button = button.to_lowercase();
        if !self.button_available(&button) {
            return Err(ButtonNotAvailable(button, self.controller));
        }
        Ok(*self.button_states.get(&button).unwrap())
    }

    fn button_available(&self, button: &str) -> bool {
        match self.controller {
            Controller::JoyconL => Self::JOYCON_L_AVAILABLE_BUTTONS.contains(&button),
            Controller::JoyconR => Self::JOYCON_R_AVAILABLE_BUTTONS.contains(&button),
            Controller::ProController => Self::PRO_CONTROLLER_AVAILABLE_BUTTONS.contains(&button),
        }
    }

    pub fn get_available_buttons(&self) -> &[&'static str] {
        match self.controller {
            Controller::JoyconL => &Self::JOYCON_L_AVAILABLE_BUTTONS,
            Controller::JoyconR => &Self::JOYCON_R_AVAILABLE_BUTTONS,
            Controller::ProController => &Self::PRO_CONTROLLER_AVAILABLE_BUTTONS,
        }
    }

    pub fn clear(&mut self) {
        self.button_states.iter_mut().for_each(|(_, v)| *v = false)
    }

    fn bitmask_button_state(&self, button: &str, byte: &mut u8, mask: u8) {
        if *self.button_states.get(button).unwrap() {
            *byte |= mask
        }
    }


    /* Utility func to set buttons in the input report
     * https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering/blob/master/bluetooth_hid_notes.md
    ┌─────┬──────┬─────┬────────┬────────┬─────┬────────┬───┬────┐
    │Byte │ 0    │ 1   │ 2      │ 3      │ 4   │ 5      │ 6 │ 7  │
    ├─────┼──────┼─────┼────────┼────────┼─────┼────────┼───┼────┤
    │     │      │     │        │        │     │        │   │    │
    │   1 │ Y    │ X   │ B      │ A      │ SR  │ SL     │ R │ ZR │
    ├─────┼──────┼─────┼────────┼────────┼─────┼────────┼───┼────┤
    │     │      │     │        │        │     │        │   │    │
    │   2 │ Minus│ Plus│ R_Stick│ L_Stick│ Home│ Capture│   │    │
    ├─────┼──────┼─────┼────────┼────────┼─────┼────────┼───┼────┤
    │     │      │     │        │        │     │        │   │    │
    │   3 │ Down │ Up  │ Right  │ Left   │ SR  │ SL     │ L │ ZL │
    └─────┴──────┴─────┴────────┴────────┴─────┴────────┴───┴────┘
     */

    pub fn as_bytes(&self) -> [u8; 3] {
        let mut result = [0, 0, 0];
        match self.controller {
            Controller::ProController => {
                // byte 0
                self.bitmask_button_state("y", &mut result[0], 0b10000000);
                self.bitmask_button_state("x", &mut result[0], 0b01000000);
                self.bitmask_button_state("b", &mut result[0], 0b00100000);
                self.bitmask_button_state("a", &mut result[0], 0b00010000);

                self.bitmask_button_state("r", &mut result[0], 0b00000010);
                self.bitmask_button_state("zr", &mut result[0], 0b00000001);

                // byte 1
                self.bitmask_button_state("minus", &mut result[1], 0b10000000);
                self.bitmask_button_state("plus", &mut result[1], 0b01000000);
                self.bitmask_button_state("r_stick", &mut result[1], 0b00100000);
                self.bitmask_button_state("l_stick", &mut result[1], 0b00010000);

                self.bitmask_button_state("home", &mut result[1], 0b00001000);
                self.bitmask_button_state("capture", &mut result[1], 0b00000100);

                // byte 2
                self.bitmask_button_state("down", &mut result[2], 0b10000000);
                self.bitmask_button_state("up", &mut result[2], 0b01000000);
                self.bitmask_button_state("right", &mut result[2], 0b00100000);
                self.bitmask_button_state("left", &mut result[2], 0b00010000);

                self.bitmask_button_state("l", &mut result[2], 0b00000010);
                self.bitmask_button_state("zl", &mut result[2], 0b00000001);
            }
            Controller::JoyconR => {
                // byte 0
                self.bitmask_button_state("y", &mut result[0], 0b10000000);
                self.bitmask_button_state("x", &mut result[0], 0b01000000);
                self.bitmask_button_state("b", &mut result[0], 0b00100000);
                self.bitmask_button_state("a", &mut result[0], 0b00010000);

                self.bitmask_button_state("sr", &mut result[0], 0b00001000);
                self.bitmask_button_state("sl", &mut result[0], 0b00000100);

                self.bitmask_button_state("r", &mut result[0], 0b00000010);
                self.bitmask_button_state("zr", &mut result[0], 0b00000001);

                // byte 1
                self.bitmask_button_state("minus", &mut result[1], 0b10000000);
                self.bitmask_button_state("plus", &mut result[1], 0b01000000);
                self.bitmask_button_state("r_stick", &mut result[1], 0b00100000);
                self.bitmask_button_state("l_stick", &mut result[1], 0b00010000);

                self.bitmask_button_state("home", &mut result[1], 0b00001000);

                // byte 2
                // Nothing for Joycon R
            }
            Controller::JoyconL => {
                // byte 0
                // Nothing for Joycon L

                // byte 1
                self.bitmask_button_state("minus", &mut result[1], 0b10000000);
                self.bitmask_button_state("plus", &mut result[1], 0b01000000);
                self.bitmask_button_state("r_stick", &mut result[1], 0b00100000);
                self.bitmask_button_state("l_stick", &mut result[1], 0b00010000);

                self.bitmask_button_state("capture", &mut result[1], 0b00000100);

                // byte 2
                self.bitmask_button_state("down", &mut result[2], 0b10000000);
                self.bitmask_button_state("up", &mut result[2], 0b01000000);
                self.bitmask_button_state("right", &mut result[2], 0b00100000);
                self.bitmask_button_state("left", &mut result[2], 0b00010000);

                self.bitmask_button_state("sr", &mut result[2], 0b00001000);
                self.bitmask_button_state("sl", &mut result[2], 0b00000100);

                self.bitmask_button_state("l", &mut result[2], 0b00000010);
                self.bitmask_button_state("zl", &mut result[2], 0b00000001);
            }
        };
        result
    }
}

#[derive(Debug, Clone, Error)]
pub struct ButtonNotAvailable(String, Controller);

impl Display for ButtonNotAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Given button \"{}\" is not available to {}.",
            self.0,
            self.1.to_string()
        )
    }
}