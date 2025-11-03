#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // User input
        let mut command = String::new();
        let exit0_command = String::from("exit 0");

        io::stdin().read_line(&mut command).unwrap();
        if command.trim() == exit0_command  {
            break;
        }

        println!("{}: command not found", command.trim())
    }
}
