use std::env;
use std::io::*;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        let pwd = env::current_dir().unwrap();
        println!("> {}", pwd.display());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprint!("{}", e);
                }
            }
            "exit" => return,
            command => {
                if let Ok(mut child) = Command::new(command).args(args).spawn() {
                    // don't accept another command until the current one completes
                    let _ = child.wait();
                }
            }
        }
    }
}
