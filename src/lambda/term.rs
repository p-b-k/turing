////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus Term
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::str::FromStr;

pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    Appl(Box<Term>, Box<Term>),
}

impl FromStr for Term {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pstate = PState::App;
        let mut _state: Option<Term> = None;

        for c in s.chars() {
            match pstate {
                PState::App => match c {
                    '\\' => {
                        pstate = PState::Var;
                    }
                    _ => {
                        return Err(format!("Invalid Character: {c:?}"));
                    }
                },
                PState::Var => match c {
                    '.' => {
                        pstate = PState::Abs;
                    }
                    _ => {
                        return Err(format!("Invalid Character: {c:?}"));
                    }
                },
                PState::Abs => match c {
                    ' ' => {
                        pstate = PState::App;
                    }
                    _ => {
                        return Err(format!("Invalid Character: {c:?}"));
                    }
                },
            }
        }

        Err(format!("{}::{} Not Implemented", "FromStr", "from_str"))
    }
}

enum PState {
    App,
    Var,
    Abs,
}
