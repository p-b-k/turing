////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus REPL Shell logic
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use color_eyre::eyre::{Result, bail};

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
    let tokens = split_command(input)?;

    if tokens.len() == 0 {
        Ok(ShellCmd::NoOp)
    } else {
        match tokens[0].as_str() {
            _ => Ok(ShellCmd::Error(format!("Unknown Command [{input}]"))),
        }
    }
}

enum ParseState {
    Token(String),
    // TEsc(String),
    Whitespace,
    Quote(String),
    // QEsc(String),
}

fn split_command(input: &str) -> Result<Vec<String>> {
    let mut state = ParseState::Whitespace;
    let mut tokens: Vec<String> = Vec::new();

    for c in input.chars() {
        match &mut state {
            ParseState::Whitespace => {
                if c.is_whitespace() {
                    // Do Nothing
                } else if c == '\'' {
                    state = ParseState::Quote(String::new())
                } else if c.is_ascii_graphic() {
                    state = ParseState::Token(format!("{c}"));
                } else {
                    bail!("Invalid input character [{c:?}]");
                }
            }
            ParseState::Token(s) => {
                if c.is_whitespace() {
                    tokens.push(s.clone());
                    state = ParseState::Whitespace;
                } else if c == '\'' {
                    tokens.push(s.clone());
                    state = ParseState::Quote(String::new())
                } else if c.is_ascii_graphic() {
                    s.push(c);
                } else {
                    bail!("Invalid input character [{c:?}]");
                }
            }
            ParseState::Quote(s) => {
                if c == '\'' {
                    tokens.push(s.clone());
                    state = ParseState::Whitespace;
                } else if c.is_ascii_graphic() {
                    s.push(c);
                } else if c.is_whitespace() {
                    s.push(c);
                } else {
                    bail!("Invalid input character [{c:?}]");
                }
            }
        }
    }

    match &state {
        ParseState::Whitespace => Ok(tokens),
        ParseState::Token(s) => {
            tokens.push(s.clone());
            Ok(tokens)
        }
        ParseState::Quote(s) => bail!("Unclosed quoted string: [{s}]"),
    }
}
