#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        // exit command
        let exit0_command = String::from("exit 0");

        // Take user input
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == exit0_command { // exit if encounter exit command
            break;
        } else if command.starts_with("echo ") { // print if encounter echo command
            let substring = &command.trim()[5..];
            println!("{substring}");
        } else if command.starts_with("type ") {
            let sub_command = &command.trim()[5..];
            match sub_command {
                "echo" | "exit" | "type" => println!("{sub_command} is a shell builtin"),
                _ => println!("{sub_command}: not found"),
            }
        } else {
            println!("{}: command not found", command.trim()); 
        }
    }
}
     