use std::os::unix::fs::PermissionsExt;
use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), std::env::VarError> {
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
                    "echo" | "exit" | "type" => {
                        println!("{target} is a shell builtin");
                        continue;
                    },
                _ => {},
                }

                let path = env::var("PATH").unwrap_or_default();
                let mut found = false;

                for dir in path.split(':') {
                    let full_path = format!("{}/{}", dir, target);

                    match fs::metadata(&full_path) {
                        Ok(meta) => {
                            let mode = meta.permissions().mode();

                            if mode & 0o111 != 0 {
                                println!("{target} is {full_path}");
                                found = true;
                                break;
                            }
                        }
                        Err(_) => {}
                    }
                }
                if !found {
                    println!("{target}: not found");
                }
            },

            "" => {}, // Do nothing if command is empty

            _ => println!("{trimmed_command}: command not found"),
        }
    }
    Ok(())
}
     