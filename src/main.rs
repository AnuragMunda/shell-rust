#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        // Take user input
        io::stdin().read_line(&mut command).unwrap();
        let trimmed_command = command.trim();

        match trimmed_command {
            "exit 0" => break, // exit command,

            cmd if trimmed_command.starts_with("echo ") => { // print command
                let text = &cmd[5..];
                println!("{text}");
            },

            cmd if trimmed_command.starts_with("type ") => { // returns the "type" of the command
                let target = &cmd[5..];
                match target {
                    "echo" | "exit" | "type" => println!("{target} is a shell builtin"),
                _ => println!("{target}: not found"),
                }
            },

            "" => {},

            _ => println!("{trimmed_command}: not found"),
        }
    }
}
     