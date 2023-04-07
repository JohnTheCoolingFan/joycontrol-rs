use std::{future::Future, pin::Pin};

use hashbrown::HashMap;
use tokio::sync::mpsc::{channel, Receiver};

use crate::{
    controller_state::ControllerState,
    stick_state::{InvalidStickValue, StickDirection, StickState}, button_state::button_push,
};

const STICK_CMD_DOC: &'static str = 
            "stick - command to set stick positions\n\
            :param side: 'l', 'left' for left control stick; 'r', 'right' for right control stick\n\
            :param direction: 'center', 'up', 'down', 'left', 'right';\n\
                              'h', 'horizontal' or 'v', 'vertical' to set the value directly to the \"value\" argument\n\
            :param value: horizontal or vertical value";

pub struct ControllerCli<'a> {
    rx: Receiver<String>,
    controller_state: &'a mut ControllerState,
}

// TODO: other commands, mostly test, from run_controller_cli.py @
// _register_commands_with_controller_state, line 168
impl<'a> ControllerCli<'a> {
    pub fn new(controller_state: &'a mut ControllerState) -> Self {
        let (tx, rx) = channel(32); // I don't know what number will work the best
        let stdin = std::io::stdin();
        tokio::spawn(async move {
            loop {
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                tx.send(buf).await.unwrap();
            }
        });
        Self {
            rx,
            controller_state,
        }
    }

    async fn read_input_line(&mut self) -> String {
        print!("cmd >> ");
        self.rx.recv().await.unwrap()
    }

    /// Implementing this as a regular command isn't possible because it requires access to &self
    /// for iterating over the commands.
    async fn regular_help(&self) {
        println!("Commands:");
        println!("{}", STICK_CMD_DOC);
        println!("Commands can be chained using \"&&\"");
        println!("Type \"exit\" to close.");
    }

    async fn help(&self) {
        let available_buttons = itertools::join(
            self.controller_state
                .button_state
                .get_available_buttons()
                .iter(),
            ", ",
        );
        println!("Button commands:");
        println!("{}", available_buttons);
        println!("");
        self.regular_help().await;
    }

    /// Reimplement if other behavior is needed
    /// For example, custom help command
    pub async fn run(&mut self) {
        let mut buttons_to_push = Vec::new();
        'inputloop: loop {
            let user_input = self.read_input_line().await;

            for command in user_input.split("&&") {
                let mut args = shlex::split(command).unwrap();
                let cmd = args.remove(0);
                if cmd == "exit" {
                    break 'inputloop;
                } else if cmd == "help" {
                    self.regular_help().await;
                } else if cmd == "stick" {
                    Self::cmd_stick(self.controller_state, &args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>()).await;
                } else if self.controller_state.button_state.get_available_buttons().contains(&cmd.as_ref()) {
                    buttons_to_push.push(cmd.clone())
                } else {
                    println!("command {} not found, call help for help.", cmd);
                }
            }

            if !buttons_to_push.is_empty() {
                button_push(self.controller_state, &buttons_to_push, None).await.unwrap();
            } else {
                self.controller_state.send().await;
            }
            buttons_to_push.clear(); // avoids re-allocation of the vec
        }
    }

    async fn cmd_stick(
        controller_state: &mut ControllerState,
        args: &[&str],
    ) -> String {
        let mut args_iter = args.into_iter();
        let side = args_iter.next().unwrap();
        let direction = args_iter.next().unwrap();
        let value = args_iter.next();
        if *side == "l" || *side == "left" {
            let stick = controller_state.l_stick_state.as_mut().unwrap();
            Self::set_stick(stick, direction.parse().unwrap(), value.copied()).unwrap()
        } else if *side == "r" || *side == "right" {
            let stick = controller_state.r_stick_state.as_mut().unwrap();
            Self::set_stick(stick, direction.parse().unwrap(), value.copied()).unwrap()
        } else {
            panic!("Unexpected argument {}", direction)
        }
    }

    /// `value` is only used for StickDirection::{Horizontal, Vertical}, so you can set it to any
    /// value or just default to 0
    fn set_stick(
        stick: &mut StickState,
        direction: StickDirection,
        value: Option<&str>,
    ) -> Result<String, InvalidStickValue> {
        match direction {
            // Not sure if just unwrapping these is a good idea... But I don't want to make yet
            // another error type that just combines those
            StickDirection::Center => stick.set_center().unwrap(),
            StickDirection::Up => stick.set_up().unwrap(),
            StickDirection::Down => stick.set_down().unwrap(),
            StickDirection::Left => stick.set_left().unwrap(),
            StickDirection::Right => stick.set_right().unwrap(),
            StickDirection::Horizontal => {
                let value = value.expect("Missing value").parse().unwrap();
                stick.set_h(value)?
            }
            StickDirection::Vertical => {
                let value = value.expect("Missing value").parse().unwrap();
                stick.set_v(value)?
            }
        }
        Ok(format!(
            "stick was set to ({}, {})",
            stick.get_h(),
            stick.get_v()
        ))
    }
}
