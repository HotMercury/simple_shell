/*
simple shell
basic command  echo, cat, ls, find, grep
bash command
*/
mod builtin;

use std::io;
use std::io::Write;
use std::process::Command;
use builtin::echo;

fn main(){
    
    loop {
        print!("#$%^&haha : ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        
        let command = command.trim();
        if !command.is_empty() {
            let mut part = command.splitn(2, ' ');
            let command = part.next().unwrap();
            let args = part.next().unwrap_or("");
            match command {
                "echo" => echo(args),
                _ => {
                    let mut child = Command::new(command)
                        .args(args.split_whitespace())
                        .spawn()
                        .expect("Failed to execute command");
                    child.wait().expect("Command failed to run");
                }
            }
        }



        // let mut part = command.trim().split_whitespace();
        // let command = part.next().unwrap();
        // let args = part;
        // let mut child = Command::new(command)
        //     .args(args)
        //     .spawn()
        //     .expect("Failed to execute command");
        // child.wait().expect("Command failed to run");
    }
}