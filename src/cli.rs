use std::collections::VecDeque;

use hashbrown::HashMap;
use tokio::sync::mpsc::{channel, Receiver};

pub struct Command {
    function: fn(&[&str]) -> String,
    doc: Option<String>,
}

pub struct CliBase {
    commands: HashMap<String, Command>,
    rx: Receiver<String>,
}

impl CliBase {
    pub fn new() -> Self {
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
            commands: HashMap::default(),
            rx,
        }
    }

    pub fn add_command(&mut self, name: &str, command: fn(&[&str]) -> String, doc: Option<&str>) {
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

    async fn run(&mut self) {
        'inputloop: loop {
            let user_input = self.read_input_line().await;

            for command in user_input.split("&&") {
                let mut args = shlex::split(command).unwrap();
                let cmd = args.remove(0);
                if cmd == "exit" {
                    break 'inputloop;
                }
                if let Some(command) = self.commands.get(&cmd) {
                    println!(
                        "{}",
                        (command.function)(&args.iter().map(String::as_str).collect::<Vec<&str>>())
                    );
                } else {
                    println!("command {} not found, call help for help.", cmd);
                }
            }
        }
    }
}
