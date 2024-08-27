#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut builtins = HashSet::new();
    builtins.insert("echo");
    builtins.insert("exit");
    builtins.insert("type");
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();

        if command.is_empty() {
            continue;
        } else if command == "exit 0" {
            break;
        } else if command.starts_with("echo ") {
            let echo_content = &command[5..];
            println!("{}", echo_content);
        } else if command.starts_with("type ") {
            let cmd_to_check = &command[5..];
            if builtins.contains(cmd_to_check) {
                println!("{} is a shell builtin", cmd_to_check);
            } else {
                println!("{}: not found", cmd_to_check);
            }
        } else {
            println!("{}: command not found", command);
        }  
    }
}