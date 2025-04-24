use std::{fmt::Debug, io::Write};

fn main() {
    loop {
        // Print prompt and wait for input
        print!("cs321% ");
        std::io::stdout().flush().unwrap();
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().to_string();

        // "exit" case - quit operations.
        if input == "exit" {
            break;
        }

        let commands = Commands::from_input(&mut input);

        println!("{commands:#?}")
    }
}

#[derive(Debug, Default)]
enum CommandDelimiter {
    #[default]
    SemiColon,
    Ampersand,
    Pipette,
}

impl CommandDelimiter {
    /// Returns the delimiter this char matches, or `None`.
    pub fn from_char(input: char) -> Option<Self> {
        match input {
            '|' => Some(CommandDelimiter::Pipette),
            '&' => Some(CommandDelimiter::Ampersand),
            ';' => Some(CommandDelimiter::SemiColon),
            _ => None,
        }
    }
}

pub static DELIMITERS: &[char] = &[';', '|', '&'];

#[derive(Debug)]
struct Command {
    delimiter: CommandDelimiter,
    command: String,
    args: Vec<String>,
}

#[derive(Debug)]
struct Commands {
    /// The commands to parse
    commands: Vec<Command>,
}

impl Commands {
    /// Takes an input string and parses out the individual commands
    pub fn from_input(input: &mut str) -> Self {
        let mut commands: Vec<Command> = vec![];

        let mut args = Vec::new();

        for mut input in input.split(' ') {
            for (pos, c) in input.chars().enumerate() {
                if let Some(delimiter) = CommandDelimiter::from_char(c) {
                    let split = input.split_at(pos);
                    args.push(split.0.to_string());
                    commands.push(Command {
                        delimiter,
                        command: args.remove(0),
                        args: std::mem::take(&mut args),
                    });

                    input = split.1.split_at(1).1;
                }
                args.push(input.to_string());
            }
        }

        // Parse the final command
        if !args.is_empty() {
            commands.push(Command {
                delimiter: CommandDelimiter::SemiColon,
                command: args.remove(0),
                args,
            })
        }

        Self { commands }
    }
}
