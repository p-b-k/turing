////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda calculus command line tool
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use color_eyre::eyre::{Result, bail};
use turing::lambda::{dictionary::Dictionary, term::Term};

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Alphas,
    Alpha(char, char),
    Betas,
    Beta(char, char),
    Parse,
}

struct AppConfig {
    pub files: Vec<String>,
    pub op: Operation,
    pub exp: Option<Box<Term>>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig {
            op: Operation::Parse,
            files: Vec::new(),
            exp: Option::None,
        }
    }
}

struct AppState {
    pub dict: Dictionary,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            dict: Dictionary::new(),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    // Create the application config data structure
    let mut cfg = AppConfig::new();

    // Populate the config from the command line
    process_args(&mut cfg)?;

    let mut state = AppState::new();

    // Load dictionary from files
    for file in cfg.files {
        state.dict.source(&file)?;
    }

    // Do whatever the operation is
    match cfg.op {
        Operation::Parse => {
            match cfg.exp {
                Option::Some(e) => println!("Expression: {e}"),
                _ => {
                    // Do Nothing
                }
            }
        }
        _ => {
            // TODO
            bail!("Operation {:?} defined yet", cfg.op)
        }
    }

    Ok(())
}

fn process_args(cfg: &mut AppConfig) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "--dict" {
            i = i + 1;
            cfg.files.push(args[i].clone());
        } else if args[i] == "--op" {
            i = i + read_operation(cfg, i + 1, &args)?;
        } else {
            // println!("Parsing input: {}", args[i]);
            if cfg.exp == Option::None {
                cfg.exp = Some(Term::parse(args[i].as_str())?);
            } else {
                bail!("Unknown argument {}", args[i]);
            }
        }

        i = i + 1;
    }

    Ok(())
}

fn set_config_op(cfg: &mut AppConfig, op: Operation) -> Result<()> {
    if cfg.op == Operation::Parse {
        cfg.op = op;
        Ok(())
    } else {
        bail!("operation already set to {:?}", cfg.op)
    }
}

fn read_operation(cfg: &mut AppConfig, index: usize, args: &Vec<String>) -> Result<usize> {
    match args[index].as_str() {
        "alphas" => {
            set_config_op(cfg, Operation::Alphas)?;
            Ok(1)
        }
        "alpha" => {
            let id = args[index + 1].clone();
            let new_val = args[index + 2].clone();
            set_config_op(
                cfg,
                Operation::Alpha(id.chars().nth(0).unwrap(), new_val.chars().nth(0).unwrap()),
            )?;

            Ok(3)
        }
        "betas" => {
            set_config_op(cfg, Operation::Betas)?;
            Ok(1)
        }
        "beta" => {
            let id = args[index + 1].clone();
            let new_val = args[index + 2].clone();
            set_config_op(
                cfg,
                Operation::Beta(id.chars().nth(0).unwrap(), new_val.chars().nth(0).unwrap()),
            )?;

            Ok(3)
        }
        _ => bail!("Unknown operation defintion: {}", args[index]),
    }
}
