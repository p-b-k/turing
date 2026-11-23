////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus REPL Shell
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use color_eyre::eyre::Result;
use turing::lambda::dictionary::Dictionary;

pub struct AppConfig {
    pub files: Vec<String>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig { files: Vec::new() }
    }
}

pub struct AppState {
    pub dict: Dictionary,
}

impl AppState {
    pub fn from_cfg(_cfg: &AppConfig) -> AppState {
        AppState {
            dict: Dictionary::new(),
        }
    }
}

fn main() -> Result<()> {
    let mut cfg = AppConfig::new();

    // Populate the config from the command line
    process_args(&mut cfg)?;

    let mut state = AppState::from_cfg(&cfg);

    start_repl_loop(&mut state)
}

fn process_args(_cfg: &AppConfig) -> Result<()> {
    Ok(())
}

fn start_repl_loop(_state: &mut AppState) -> Result<()> {
    Ok(())
}
