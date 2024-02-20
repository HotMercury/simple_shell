/*
simple shell
basic command  echo, cat, ls, find, grep
bash command
*/

use std::io;
use std::io::Write;
use std::process::Command;

fn main(){
    loop {
        print!("#$%^&haha : ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let mut part = command.trim().split_whitespace();
        let command = part.next().unwrap();
        let args = part;
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .expect("Failed to execute command");
        child.wait().expect("Command failed to run");
    }
}