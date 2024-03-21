use std::env;
use std::io::*;
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn change_directory(args: Vec<&str>) {
    let dir = args.get(0).map_or("/", |x| *x);
    let root = Path::new(dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprint!("{}", e);
    }
}

fn exit() {
    std::process::exit(0);
}

fn execute_command(
    command: &str,
    prev_command: Option<Child>,
    args: Vec<&str>,
    has_next: bool,
) -> Result<Child> {
    // get prev_command's output as the input
    let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.unwrap())
    });

    let stdout = if has_next {
        // more commands piped after this, send the output to the next
        Stdio::piped()
    } else {
        // no more commands piped after this, so display the output
        Stdio::inherit()
    };

    Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
}

fn execute_commands(input: &str) -> Result<()> {
    let mut commands = input.trim().split(" | ").peekable();
    let mut prev_command: Option<Child> = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts.collect();

        match command {
            // [cd has to be a built-in to the shell](https://unix.stackexchange.com/questions/38808/why-is-cd-not-a-program/38809#38809)
            "cd" => {
                change_directory(args);
                prev_command = None;
            }
            // exit from the shell
            "exit" => exit(),
            // the rest
            command => {
                match execute_command(command, prev_command, args, commands.peek().is_some()) {
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

    Ok(())
}

fn main() {
    loop {
        let pwd = env::current_dir().unwrap();
        println!("> {}", pwd.display());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match execute_commands(&input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
