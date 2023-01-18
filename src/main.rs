pub mod builtins;
pub mod env;
pub mod preprocess;

use crate::env::load_env;
use builtins::builtins;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use std::process::{Child, Command, Stdio};

fn handle_err(e: ReadlineError) {
    match e {
        ReadlineError::Interrupted => {
            println!("CTRL-C");
        }
        ReadlineError::Eof => {
            println!("CTRL-D");
        }
        err => {
            println!("Error: {:?}", err);
        }
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let env = load_env();

    loop {
        //current dir
        let readline =
            rl.readline(format!("({}) ", std::env::current_dir().unwrap().display()).as_str());

        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            }
            Err(e) => {
                handle_err(e);
                break;
            }
        };

        // read_line leaves a trailing newline, which trim removes
        // this needs to be peekable so we can determine when we are on the last command
        let commands = preprocess::split_pipes(input);
        let mut peekable_command = commands.iter().peekable();

        let mut previous_command = None;

        while let Some(command) = peekable_command.next() {
            // break if we get a hit
            if builtins(command, &mut previous_command) {
                break;
            };

            let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                Stdio::from(output.stdout.unwrap())
            });

            let stdout = if peekable_command.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let processed_command = preprocess::split_command(command.to_string());
            let executable = &processed_command[0].clone();
            let args = &processed_command[1..];

            let mut cmd = Command::new(executable);

            cmd.args(args);

            env::set_env(&mut cmd, env.clone());

            let output = cmd.stdin(stdin).stdout(stdout).spawn();

            match output {
                Ok(output) => {
                    previous_command = Some(output);
                }
                Err(e) => {
                    previous_command = None;
                    eprintln!("{}", e);
                }
            };
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
    rl.save_history("history.txt")
}
