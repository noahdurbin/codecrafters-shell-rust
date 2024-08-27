use std::io::{self, Write};
use std::collections::HashSet;
use std::env;
use std::path::Path;

fn main() {
    let stdin = io::stdin();
    let builtins: HashSet<&str> = HashSet::from(["echo", "exit", "type"]);
    
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
                match find_command(cmd_to_check) {
                    Some(path) => println!("{} is {}", cmd_to_check, path),
                    None => println!("{}: not found", cmd_to_check),
                }
            }
        } else {
            println!("{}: command not found", command);
        }  
    }
}

fn find_command(cmd: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(cmd);
            if full_path.is_file() {
                return full_path.to_str().map(|s| s.to_string());
            }
        }
    }
    None
}
