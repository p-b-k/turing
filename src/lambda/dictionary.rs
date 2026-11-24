////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Dictionary for names
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{collections::HashMap, fs::read_to_string};

use color_eyre::eyre::{Result, bail};

use crate::lambda::term::Term;

pub struct Dictionary {
    map: HashMap<char, Box<Term>>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: char, term: Box<Term>) -> Result<()> {
        if self.map.contains_key(&name) {
            bail!("Dictionary::insert: failed because {name} is already defined")
        } else {
            self.map.insert(name, term);
            Ok(())
        }
    }

    pub fn lookup(&self, name: char) -> Result<&Box<Term>> {
        match self.map.get(&name) {
            Option::None => bail!("Dictionary::lookup: {name} not found"),
            Option::Some(t) => Ok(t),
        }
    }

    pub fn source(&mut self, file: &str) -> Result<()> {
        let file_data_result = read_to_string(file);
        match file_data_result {
            Ok(file_data) => {
                // TODO
                for l in file_data.lines() {
                    // Read Def
                    let comment_start = l.find('#');
                    let line = match comment_start {
                        Option::None => l.to_string(),
                        Option::Some(i) => l[0..i].to_string(),
                    }
                    .trim()
                    .to_string();

                    if line.len() > 0 {
                        // TODO Parse Line and insert def into dict
                        println!("Read Line: {line}");
                    }
                    // let line = if l.
                }
                Ok(())
            }
            Err(e) => bail!("{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lambda::term::_var;

    use super::*;

    #[test]
    fn insert_term_test() {
        let mut dict = Dictionary::new();

        match dict.lookup('N') {
            Ok(_) => assert_eq!(true, false),
            _ => {}
        }

        match dict.insert('N', _var('x')) {
            Err(_) => assert_eq!(true, false),
            _ => {}
        }

        match dict.lookup('N') {
            Ok(t) => assert_eq!(_var('x'), *t),
            _ => assert_eq!(true, false),
        }
    }
}
