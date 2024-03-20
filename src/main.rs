use std::env;
use std::io::*;
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        let pwd = env::current_dir().unwrap();
        println!("> {}", pwd.display());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // piped commands
        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                // [cd has to be a built-in to the shell](https://unix.stackexchange.com/questions/38808/why-is-cd-not-a-program/38809#38809)
                "cd" => {
                    let dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprint!("{}", e);
                    }

                    prev_command = None;
                }
                // exit from the shell
                "exit" => return,
                command => {
                    let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // more commands piped after this, send the output to the next
                        Stdio::piped()
                    } else {
                        // no more commands piped after this, so display the output
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(child) => {
                            prev_command = Some(child);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            prev_command = None;
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = prev_command {
            // don't accept another command until the current one completes
            let _ = final_command.wait();
        }
    }
}
