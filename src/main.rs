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
            
            //print PID
            if input == "print" {
                println!("{}",std::process::id());
            }

            //help
            if input == "help" {
                println!("cd - changes working directory\nprint - gets PID\nexit - exits the program\ngetdir - prints working directory");
            }

            // "exit" case - quit operations.
            if input == "exit" {
                return;
            }

            // Validate input using ASCII validation
            if !is_valid_input_ascii(input) {
                println!("Error: Invalid characters in input.");
                continue;
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
                
                if command.command == "cd" {
                    
                    if command.args.len() == 1 {

                        let mut path = std::env::current_dir().unwrap();

                        if command.args[0] == ".." {

                            path.pop();
                            std::env::set_current_dir(&path);

                        } else {

                            path.push(command.args[0]);

                            if path.is_dir() {

                                std::env::set_current_dir(&path);

                            }

                        }

                    }

                }
                
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


// Helper function to validate if the input contains only allowed ASCII characters.
fn is_valid_input_ascii(input: &str) -> bool {
    input.chars().all(|c| {
        let ascii = c as u8;
        (ascii >= b'A' && ascii <= b'Z') || // A-Z
        (ascii >= b'a' && ascii <= b'z') || // a-z
        (ascii >= b'0' && ascii <= b'9') || // 0-9
        ascii == b'-' ||                    // dash (-)
        ascii == b'.' ||                    // dot (.)
        ascii == b'/' ||                    // forward slash (/)
        ascii == b'_' ||                    // underscore (_)
        ascii == b' ' ||                    // space (for separation)
        ascii == b';' ||                    // semicolon (for command separation)
        ascii == b'&' ||                    // ampersand (for background execution)
        ascii == b'|'                      // pipe (for piping commands)
    })
}