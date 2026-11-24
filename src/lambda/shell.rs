////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus REPL Shell logic
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use color_eyre::eyre::Result;

use clearscreen;
use log::warn;

use std::{
    io::{self, Write},
    thread, time,
};

use crate::lambda::dictionary::Dictionary;

pub struct AppState {
    pub running: bool,
    pub dict: Dictionary,
}

impl AppState {}

pub enum ShellCmd {
    Exit,
    NoOp,
    Clear,
    Error(String),
    // TODO -- Add More Commands
}

impl ShellCmd {
    pub fn execute(&self, state: &mut AppState) -> Result<()> {
        match self {
            Self::Exit => {
                state.running = false;
                Ok(())
            }
            Self::NoOp => {
                // Do Nothing
                Ok(())
            }
            Self::Clear => match clearscreen::clear() {
                Err(e) => {
                    warn!("{e}");
                    Ok(())
                }
                Ok(_) => {
                    // Do Nothing
                    Ok(())
                }
            },
            Self::Error(e) => {
                eprintln!("Error: {e}");
                Ok(())
            }
        }
    }
}

pub fn start_repl_loop(state: &mut AppState) -> Result<()> {
    while state.running {
        print_prompt(state)?;
        thread::sleep(time::Duration::from_secs(1));
        let cmd = read_cmd(state)?;

        cmd.execute(state)?;
    }

    println!("Exiting Cleanlhy {}", '\u{1f44d}');

    Ok(())
}

pub fn print_prompt(_state: &AppState) -> Result<()> {
    print!("{} {} ", '\u{039b}', '\u{27a1}');
    io::stdout().flush()?;
    Ok(())
}

pub fn read_cmd(state: &AppState) -> Result<ShellCmd> {
    // TODO
    // bail!("Not Implmented");
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    if input.len() == 0 {
        Ok(ShellCmd::NoOp)
    } else {
        match input.as_str() {
            "clear" => Ok(ShellCmd::Clear),
            "exit" => Ok(ShellCmd::Exit),
            _ => parse_command_line(input.as_str(), state),
        }
    }
}

fn parse_command_line(input: &str, _state: &AppState) -> Result<ShellCmd> {
    Ok(ShellCmd::Error(format!("Unknown Command [{input}]")))
}
