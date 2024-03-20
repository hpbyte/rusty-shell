use std::io::*;
use std::process::Command;

fn main() {
    loop {
        println!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command).args(args).spawn().unwrap();

        // don't accept another command until the current one completes
        let _ = child.wait();
    }
}
