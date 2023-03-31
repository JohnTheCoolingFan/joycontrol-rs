use std::{future::Future, pin::Pin};

use hashbrown::HashMap;
use tokio::sync::mpsc::{channel, Receiver};

use crate::{
    controller_state::ControllerState,
    stick_state::{InvalidStickValue, StickDirection, StickState},
};

pub struct Command {
    function: Box<dyn Fn(&[&str]) -> Pin<Box<dyn Future<Output = String>>>>,
    doc: Option<String>,
}

// TODO: help command
pub struct ControllerCli {
    commands: HashMap<String, Command>,
    rx: Receiver<String>,
    available_buttons: String,
}

impl ControllerCli {
    pub fn new(controller_state: &mut ControllerState) -> Self {
        let (tx, rx) = channel(32); // I don't know what number will work the best
        let stdin = std::io::stdin();
        tokio::spawn(async move {
            loop {
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                tx.send(buf).await.unwrap();
            }
        });
        let mut result = Self {
            commands: HashMap::default(),
            rx,
            available_buttons: itertools::join(
                controller_state.button_state.get_available_buttons().iter(),
                ", ",
            ),
        };
        result.add_command(
            "stick",
            Box::new(|args| Box::pin(Self::cmd_stick(controller_state, args))),
            Some("stick - command to set stick positions\n\
            :param side: 'l', 'left' for left control stick; 'r', 'right' for right control stick\n\
            :param direction: 'center', 'up', 'down', 'left', 'right';\n\
                              'h', 'horizontal' or 'v', 'vertical' to set the value directly to the \"value\" argument\n\
            :param value: horizontal or vertical value"),
        );
        result
    }

    fn add_command(
        &mut self,
        name: &str,
        command: Box<dyn Fn(&[&str]) -> Pin<Box<dyn Future<Output = String>>>>,
        doc: Option<&str>,
    ) {
        if !self.commands.contains_key(name) {
            self.commands.insert(
                name.into(),
                Command {
                    function: command,
                    doc: doc.map(Into::into),
                },
            );
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
        for (name, Command { function: _, doc }) in &self.commands {
            print!("{}", name);
            if let Some(docstr) = doc {
                println!(": {}", docstr);
            } else {
                println!("");
            }
        }
        println!("Commands can be chained using \"&&\"");
        println!("Type \"exit\" to close.");
    }

    async fn help(&self) {
        println!("Button commands:");
        println!("{}", self.available_buttons);
        println!("");
        self.regular_help().await;
    }

    /// Reimplement if other behavior is needed
    /// For example, custom help command
    pub async fn run(&mut self) {
        'inputloop: loop {
            let user_input = self.read_input_line().await;

            for command in user_input.split("&&") {
                let mut args = shlex::split(command).unwrap();
                let cmd = args.remove(0);
                if cmd == "exit" {
                    break 'inputloop;
                }
                if cmd == "help" {
                    self.regular_help().await;
                } else {
                    if let Some(command) = self.commands.get(&cmd) {
                        println!(
                            "{}",
                            (command.function)(
                                &args.iter().map(String::as_str).collect::<Vec<&str>>()
                            )
                            .await
                        );
                    } else {
                        println!("command {} not found, call help for help.", cmd);
                    }
                }
            }
        }
    }

    async fn cmd_stick(controller_state: &mut ControllerState, args: &[&str]) -> String {
        let mut args_iter = args.iter();
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
