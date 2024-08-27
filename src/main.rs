#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    
    loop {
      print!("$ ");

      let mut input = String::new();

      io::stdout().flush().unwrap();
      
      stdin.read_line(&mut input).unwrap();

      let command = input.trim();

      if command.is_empty() {
        continue;
      } else if command == "exit 0" {
        break;
      } else if command.starts_with("echo ") {
        let echo_content = &command[5..];
        println!("{}", echo_content);
      } else {
        println!("{}: command not found", input.trim());
      }  
    }
}
