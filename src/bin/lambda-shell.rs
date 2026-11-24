////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus REPL Shell main function
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use color_eyre::eyre::{Result, bail};
use turing::lambda::{
    dictionary::Dictionary,
    shell::{AppState, start_repl_loop},
};

pub struct AppConfig {
    pub files: Vec<String>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig { files: Vec::new() }
    }
}

fn state_from_cfg(_cfg: &AppConfig) -> AppState {
    AppState {
        running: false,
        dict: Dictionary::new(),
    }
}

fn main() -> Result<()> {
    let mut cfg = AppConfig::new();

    // Populate the config from the command line
    process_args(&mut cfg)?;

    let mut state = state_from_cfg(&cfg);

    // Set Running to TRUE
    state.running = true;

    eprintln!("Starting REPL");
    start_repl_loop(&mut state)
}

fn process_args(cfg: &mut AppConfig) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "--dict" {
            i = i + 1;
            cfg.files.push(args[i].clone());
        } else {
            bail!("Unknown argument {}", args[i]);
        }

        i = i + 1;
    }

    Ok(())
}
