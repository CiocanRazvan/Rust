use std::fs::File;
use std::io::{self,BufRead};

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong");
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("Counted {} args",args.len());
    }
}

struct TimesCommand {
    count:u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("Called {} times",self.count);
    }
}

struct EchoCommand;

impl Command for EchoCommand {
    fn get_name(&self) -> &str {
        "echo"
    }
    fn exec(&mut self, args: &[&str]) {
        println!("Called {} times",args.join(" "));
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Self {commands: Vec::new()}
    }

    fn register(&mut self,cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    fn find_exact_mut(&mut self,name: &str) -> Option<&mut Box<dyn Command>> {
        self.commands.iter_mut().find(|c| c.get_name() == name)
    }

    fn suggest_case_insensitive(&self,wrong: &str) -> Option<String> {
        let wrong_lower=wrong.to_lowercase();
        for cmd in &self.commands {
            if cmd.get_name().to_lowercase() ==wrong_lower {
                return Some(cmd.get_name().to_string());
            }
        }
        None
    }

    fn run(&mut self) {
        let file = match File::open("file.txt") {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error open file");
                return;
            }
        };

        for line in io::BufReader::new(file).lines() {
            let Ok(line) = line else { continue };
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let command_name = parts[0];
            let command_lower = command_name.to_lowercase();
            let args = &parts[1..];

            if command_lower == "stop" {
                println!("Stopping terminal");
                return;
            }

            if let Some(cmd)=self.find_exact_mut(&command_lower) {
                cmd.exec(args);
            } else if let Some(suggest) = self.suggest_case_insensitive(command_name) {
                    eprintln!("Unknown command '{}'. Did you mean '{}' ?",command_name,suggest);
            } else {
                    eprintln!("Unknown command '{} .'",command_name);
            }
            
        }
    }

}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand));
    terminal.register(Box::new(CountCommand));
    terminal.register(Box::new(TimesCommand{count:0}));
    terminal.register(Box::new(EchoCommand));

    terminal.run();
}