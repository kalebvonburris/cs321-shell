// lib.rs
pub mod execute;
pub use execute::execute;

pub static DELIMITERS: &[char] = &[';', '|', '&'];

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CommandDelimiter {
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

    /// Returns the delimiter this char matches, or `None`.
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "|" => Some(CommandDelimiter::Pipette),
            "&" => Some(CommandDelimiter::Ampersand),
            ";" => Some(CommandDelimiter::SemiColon),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub delimiter: CommandDelimiter,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct Commands {
    /// The commands to parse
    pub commands: Vec<Command>,
}

impl Commands {
    /// Takes an input String and parses out the individual commands
    pub fn from_input(input: &mut str) -> Self {
        let mut commands: Vec<Command> = vec![];

        let mut args = Vec::new();

        for arg in input.split(' ') {
            if let Some(delimiter) = CommandDelimiter::from_str(arg) {
                commands.push(Command {
                    delimiter,
                    command: args.remove(0),
                    args: std::mem::take(&mut args),
                });
                continue;
            }
            args.push(String::from(arg));
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
