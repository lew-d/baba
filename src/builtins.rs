use std::{env, path::Path, process::Child};

pub fn builtins(command: &str, previous_command: &mut Option<Child>) -> Result<bool, ()> {
    // everything after the first whitespace character is interpreted as args to the command
    let mut parts = command.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    match command {
        "cd" => {
            // default to '/' as new directory if one was not provided
            let new_dir = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }

            *previous_command = None;

            Ok(true)
        }
        "exit" => Err(()),
        _ => Ok(false),
    }
}
