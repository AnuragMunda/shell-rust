use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), std::env::VarError> {
    // Builtin command list
    const BUILTINS: &[&str] = &[
        "echo",
        "exit",
        "type",
        "pwd",
    ];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        let path = env::var("PATH").unwrap_or_default();

        // Take user input
        io::stdin().read_line(&mut command).unwrap();
        let trimmed_command = command.trim();

        match trimmed_command {
            "exit 0" | "exit" => break, // exit command,

            cmd if trimmed_command.starts_with("echo ") => { // print command
                let text = &cmd[5..];
                println!("{text}");
            },

            cmd if trimmed_command.starts_with("type ") => { // returns the "type" of the command
                let target = &cmd[5..];
                if BUILTINS.contains(&target) {
                    println!("{target} is a shell builtin");
                    continue;
                }

                let mut found = false;

                for dir in path.split(':') { // Check for executable file
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

            cmd if trimmed_command.starts_with("cd ") => {
                let path_arg = &cmd[3..];
                let home = env::var("HOME").unwrap_or_default();

                if path_arg == "~" {
                    env::set_current_dir(home).unwrap_or_default();
                } else {
                    let new_dir = Path::new(path_arg);
                    match env::set_current_dir(&new_dir) {
                        Ok(_) => {},
                        Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
                    }
                }
            },

            "pwd" => { // returns the current path of the shell
                let cur_dir = env::current_dir().unwrap_or_default();
                println!("{}", cur_dir.display());
            }

            "" => {}, // Do nothing if command is empty

            _ => {
                // Execute the program if program name is passed
                let mut found = false;

                let cmd_parts: Vec<&str> = trimmed_command.split(' ').collect();
                let cmd = cmd_parts[0];
                let args = &cmd_parts[1..];
                
                for dir in path.split(':') {
                    let full_path = format!("{}/{}", dir, cmd);

                    match fs::metadata(&full_path) {
                        Ok(meta) => {
                            let mode = meta.permissions().mode();

                            if mode & 0o111 != 0 {
                                Command::new(cmd).args(args).status().expect("Failed to execute");
                                found = true;
                                break;
                            }
                        }
                        Err(_) => {}
                    }
                }

                // If program is not found
                if !found {
                    println!("{cmd}: command not found");
                }
            },
        }
    }
    Ok(())
}
     