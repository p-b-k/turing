////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus Action definitions
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::lambda::{shell::AppState, term::Term};

use color_eyre::eyre::Result;
use log::warn;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum ArgValue {
    LTerm(Term),
    Value(String),
    Number(usize),
}

impl Display for ArgValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ArgValue::LTerm(t) => write!(f, "{t}"),
            ArgValue::Value(t) => write!(f, "{t}"),
            ArgValue::Number(t) => write!(f, "{t}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ArgType {
    LTerm,
    Value,
    Number,
}

#[derive(Debug)]
pub struct CmdForm {
    pub args: Vec<ArgType>,
}

impl CmdForm {
    pub fn new() -> CmdForm {
        CmdForm { args: Vec::new() }
    }

    pub fn from(spec: &[ArgType]) -> Result<CmdForm> {
        Ok(CmdForm {
            args: Vec::from(spec),
        })
    }
}

#[derive(Debug)]
pub struct CmdArgs {
    pub vals: Vec<ArgValue>,
}

#[derive(Debug)]
pub enum ShellCmd {
    Exit,
    NoOp,
    Clear,
    Error(String),
    Action(fn(&mut AppState, &Vec<String>) -> Result<()>, Vec<String>),
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
            Self::Action(f, a) => {
                // eprintln!("Error: {f:?} on {a:?}");
                f(state, &a)?;
                Ok(())
            }
        }
    }
}

pub fn action_define_term(_state: &mut AppState, args: &Vec<String>) -> Result<()> {
    println!("Def Called on {:?}", args);
    Ok(())
}

pub fn action_list_free(_state: &mut AppState, args: &Vec<String>) -> Result<()> {
    let term = Term::parse(args[0].as_str())?;
    // println!("free Called on {}", term);
    let mut free = term.free();
    free.sort();
    let mut sep = "";
    for x in free {
        print!("{sep}{x}");
        sep = " ";
    }
    println!();

    Ok(())
}
