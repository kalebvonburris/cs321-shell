use cs321_shell::*;
use std::io::Write;

fn main() {
    unsafe {
        loop {
            // Print prompt and wait for input
            print!("cs321% ");
            std::io::stdout().flush().unwrap();
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut input: String = input.trim().to_string();

            // "exit" case - quit operations.
            if input == "exit" {
                return;
            }

            // Check character length constraint (100 characters max)
            if input.len() > 100 {
                println!("Error: Input exceeds 100 character limit.");
                continue;
            }

            // Process commands
            let commands = Commands::from_input(&mut input);

            // Execute commands
            for command in commands.commands {
                let delimiter = command.delimiter.clone();
                let id = execute(command);

                // Child process id is 0, so we end the execution
                if id == 0 {
                    return;
                }

                if delimiter == CommandDelimiter::SemiColon {
                    // Wait for command to complete
                    libc::wait(id as *mut i32);
                }
            }
        }
    }
}
