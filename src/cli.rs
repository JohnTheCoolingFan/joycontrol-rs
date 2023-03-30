use std::{future::Future, pin::Pin};

use hashbrown::HashMap;
use tokio::sync::mpsc::{channel, Receiver};

use crate::{
    controller_state::ControllerState,
    stick_state::{InvalidStickValue, StickDirection, StickState},
};

macro_rules! wrap_command {
    ($func_expr:path) => {
        Box::new(|args| Box::pin($func_expr(args)))
    };
}

pub struct Command {
    function: Box<dyn Fn(&[&str]) -> Pin<Box<dyn Future<Output = String>>>>,
    doc: Option<String>,
}

// TODO: help command
pub struct CliBase {
    commands: HashMap<String, Command>,
    help_command: Box<dyn Fn(&Self) -> Pin<Box<dyn Future<Output = ()> + '_>>>,
    rx: Receiver<String>,
}

impl CliBase {
    pub fn add_command(
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

    /// Reimplement if other behavior is needed
    /// Implementing this as a regular command isn't possible because it requires access to &self
    /// for iterating over the commands.
    async fn default_help(&self) {
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

    /// Reimplement if other behavior is needed
    /// For example, custom help command
    async fn run(&mut self) {
        'inputloop: loop {
            let user_input = self.read_input_line().await;

            for command in user_input.split("&&") {
                let mut args = shlex::split(command).unwrap();
                let cmd = args.remove(0);
                if cmd == "exit" {
                    break 'inputloop;
                }
                if cmd == "help" {
                    self.default_help().await;
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
}

pub struct CliBaseBuilder {
    commands: HashMap<String, Command>,
    help_command: Box<dyn Fn(&CliBase) -> Pin<Box<dyn Future<Output = ()> + '_>>>,
}

impl CliBaseBuilder {
    pub fn new() -> Self {
        Self {
            commands: HashMap::default(),
            help_command: Box::new(|cli| Box::pin(cli.default_help())),
        }
    }

    pub fn add_command(
        mut self,
        name: String,
        doc: Option<String>,
        command: Box<dyn Fn(&[&str]) -> Pin<Box<dyn Future<Output = String>>>>,
    ) -> Self {
        if !self.commands.contains_key(&name) {
            self.commands.insert(
                name,
                Command {
                    function: command,
                    doc,
                },
            );
        }
        self
    }

    pub fn change_help(
        mut self,
        command: Box<dyn Fn(&CliBase) -> Pin<Box<dyn Future<Output = ()> + '_>>>,
    ) -> Self {
        self.help_command = command;
        self
    }

    pub fn build(self) -> CliBase {
        let (tx, rx) = channel(32); // I don't know what number will work the best
        let stdin = std::io::stdin();
        tokio::spawn(async move {
            loop {
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                tx.send(buf).await.unwrap();
            }
        });
        CliBase {
            commands: self.commands,
            help_command: self.help_command,
            rx,
        }
    }
}

pub struct ControllerCli<'a> {
    cli: CliBase,
    pub controller_state: &'a ControllerState,
}

impl<'a> ControllerCli<'a> {
    pub fn new(controller_state: &'a ControllerState) -> Self {
        let cli = CliBaseBuilder::new().build();
        Self {
            cli,
            controller_state,
        }
    }

    /// `value` is only used for StickDirection::{Horizontal, Vertical}, so you can set it to any
    /// value or just default to 0
    fn set_stick(
        stick: &mut StickState,
        direction: StickDirection,
        value: u32,
    ) -> Result<String, InvalidStickValue> {
        match direction {
            // Not sure if just unwrapping these is a good idea... But I don't want to make yet
            // another error type that just combines those
            StickDirection::Center => stick.set_center().unwrap(),
            StickDirection::Up => stick.set_up().unwrap(),
            StickDirection::Down => stick.set_down().unwrap(),
            StickDirection::Left => stick.set_left().unwrap(),
            StickDirection::Right => stick.set_right().unwrap(),
            StickDirection::Horizontal => stick.set_h(value)?,
            StickDirection::Vertical => stick.set_v(value)?,
        }
        Ok(format!(
            "stick was set to ({}, {})",
            stick.get_h(),
            stick.get_v()
        ))
    }
}
