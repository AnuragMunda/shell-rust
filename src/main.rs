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

    // Iterate over the commands until exited
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        let path = env::var("PATH").unwrap_or_default(); // Get the value of PATH environment variable

        // Take user input
        io::stdin().read_line(&mut command).unwrap();
        let trimmed_command = command.trim();

        // Match the user command with expected functionality
        match trimmed_command {
            // `exit` command
            // Exits the shell
            "exit 0" | "exit" => break,
            
            // `echo` command
            // Prints the user input text on the screen
            cmd if trimmed_command.starts_with("echo ") => {
                let text = &cmd[5..];

                if text.starts_with("'") && text.ends_with("'") {
                    let unquoted_text = text.replace("'", "");
                    println!("{unquoted_text}");
                } else {
                    let filtered_text = text.split_whitespace().collect::<Vec<&str>>().join(" ");
                    let new_text = filtered_text.replace("''", "");
                    println!("{new_text}");
                }
            },

            // `type` command
            // Returns whether the passed argument is a bulitin command or an executable
            cmd if trimmed_command.starts_with("type ") => {
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
                if !found { // if passed arguement is not recognised 
                    println!("{target}: not found");
                }
            },

            // `cd` command
            // Changes the current directory to the input path
            cmd if trimmed_command.starts_with("cd ") => {
                let path_arg = &cmd[3..];
                let home = env::var("HOME").unwrap_or_default();

                if path_arg == "~" { // `~` - HOME directory
                    env::set_current_dir(home).unwrap_or_default();
                } else { // Handles absolute and relative paths
                    let new_dir = Path::new(path_arg);
                    match env::set_current_dir(&new_dir) {
                        Ok(_) => {},
                        Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
                    }
                }
            },

            // `pwd` command
            // Returns the current working directory
            "pwd" => {
                let cur_dir = env::current_dir().unwrap_or_default();
                println!("{}", cur_dir.display());
            }

            "" => {}, // Do nothing if command is empty

            // Handle other cases
            // Handle running a program
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

                // If command is not found
                if !found {
                    println!("{cmd}: command not found");
                }
            },
        }
    }
    Ok(())
}
     