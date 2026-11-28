////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus REPL Shell logic
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use color_eyre::eyre::{Result, bail};

use std::io::{self, Write};

use crate::lambda::{
    actions::{ShellCmd, action_list_free},
    dictionary::Dictionary,
};

pub struct AppState {
    pub running: bool,
    pub dict: Dictionary,
}

impl AppState {}

pub fn start_repl_loop(state: &mut AppState) -> Result<()> {
    while state.running {
        print_prompt(state)?;
        let cmd = read_cmd(state)?;

        match cmd.execute(state) {
            Err(e) => eprintln!("Failed: {e}"),
            _ => {
                // Do Nothing
            }
        }
    }

    println!("{}", '\u{1f44d}');

    Ok(())
}

pub fn print_prompt(_state: &AppState) -> Result<()> {
    print!("{} {} ", '\u{039b}', '\u{27a1}');
    io::stdout().flush()?;
    Ok(())
}

pub fn read_cmd(state: &AppState) -> Result<ShellCmd> {
    let mut input = String::new();

    let len = io::stdin().read_line(&mut input)?;

    if len > 0 {
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
    } else {
        Ok(ShellCmd::Exit)
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

fn parse_command_line(input: &str, _state: &AppState) -> Result<ShellCmd> {
    let tokens = split_command(input)?;

    if tokens.len() == 0 {
        Ok(ShellCmd::NoOp)
    } else {
        // Get Arguments
        let mut args: Vec<String> = Vec::new();

        for x in tokens[1..].iter() {
            args.push(x.clone());
        }

        match tokens[0].as_str() {
            // "def" => Ok(ShellCmd::Action(action_define_term), CmdForm::from([ArgType::Value, ArgType::LTerm]))
            "free" => Ok(ShellCmd::Action(action_list_free, args)),
            _ => Ok(ShellCmd::Error(format!("Unknown Command [{input}]"))),
        }
    }
}
