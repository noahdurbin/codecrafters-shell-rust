use std::io::{self, Write};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let stdin = io::stdin();
    let builtins: HashSet<&str> = HashSet::from(["echo", "exit", "type"]);
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];
        
        match command {
            "exit" if args == ["0"] => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if let Some(cmd_to_check) = args.first() {
                    if builtins.contains(cmd_to_check) {
                        println!("{} is a shell builtin", cmd_to_check);
                    } else {
                        match find_command(cmd_to_check) {
                            Some(path) => println!("{} is {}", cmd_to_check, path),
                            None => println!("{}: not found", cmd_to_check),
                        }
                    }
                } else {
                    println!("type: missing argument");
                }
            }
            _ => {
                match find_command(command) {
                    Some(path) => {
                        match execute_command(&path, args) {
                            Ok(output) => print!("{}", output),
                            Err(e) => eprintln!("Error executing {}: {}", command, e),
                        }
                    }
                    None => println!("{}: command not found", command),
                }
            }
        }
    }
}

fn find_command(cmd: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(cmd);
            if full_path.is_file() {
                return full_path.to_str().map(String::from);
            }
        }
    }
    None
}

fn execute_command(command: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()?;
    
    let mut result = String::new();
    if !output.stdout.is_empty() {
        result.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        result.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    Ok(result)
}
