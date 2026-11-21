////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus Term
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::slice::Iter;

use color_eyre::eyre::{Result, bail};

#[derive(Debug, PartialEq)]
pub enum Term {
    Var(char),
    Abs(char, Box<Term>),
    Appl(Box<Term>, Box<Term>),
    // Name(String), // TODO start adding name
}

// impl FromStr for Term {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match parse(&String::from(s)) {
//             Ok(t) => Ok(t),
//             Err(e) => Err(e.to_string()),
//         }

//         // Err(format!("{}::{} Not Implemented", "FromStr", "from_str"))
//     }
// }

#[derive(PartialEq, Debug, Clone)]
enum Token {
    Dot,
    Lambda,
    OParen,
    CParen,
    LChar(char),
}

impl Term {
    pub fn parse(input: &str) -> Result<Box<Term>> {
        let mut tokens: Vec<Token> = Vec::new();

        for c in input.chars() {
            if c == '\\' {
                tokens.push(Token::Lambda);
            } else if c == '.' {
                tokens.push(Token::Dot);
            } else if c == '(' {
                tokens.push(Token::OParen);
            } else if c == ')' {
                tokens.push(Token::CParen);
            } else if c.is_ascii_lowercase() {
                tokens.push(Token::LChar(c));
            } else {
                bail!("Invalid input character: {c:?}");
            }
        }

        parse_term(&mut tokens.iter())
    }
}

fn parse_term(mut tstream: &mut Iter<'_, Token>) -> Result<Box<Term>> {
    let mut term: Option<Box<Term>> = Option::None;

    loop {
        let next_token = tstream.next();

        match next_token {
            Option::None => break,
            Option::Some(t) => match t {
                Token::Dot => {
                    bail!("parse error, unexpected '.'");
                }
                Token::Lambda => {
                    let abs = parse_abs(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, abs)));
                    } else {
                        term = Some(abs);
                    }
                }
                Token::OParen => {
                    let sub_term = parse_sub_term(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, sub_term)));
                    } else {
                        term = Some(sub_term);
                    }
                }
                Token::CParen => {
                    bail!("parse error, unexpected ')'");
                }
                Token::LChar(c) => {
                    let var = Box::new(Term::Var(c.clone()));

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, var)));
                    } else {
                        term = Some(var);
                    }
                }
            },
        }
    }

    match term {
        Option::None => bail!("no term read"),
        Option::Some(t) => Ok(t),
    }
}

fn parse_abs(_tstream: &mut Iter<'_, Token>) -> Result<Box<Term>> {
    // TODO
    bail!("parse_abs not implemented")
}

fn parse_sub_term(mut tstream: &mut Iter<'_, Token>) -> Result<Box<Term>> {
    let mut term: Option<Box<Term>> = Option::None;

    loop {
        let next_token = tstream.next();

        match next_token {
            Option::None => bail!("Unexpected end of input"),
            Option::Some(t) => match t {
                Token::Dot => {
                    bail!("parse error, unexpected '.'");
                }
                Token::Lambda => {
                    let abs = parse_abs(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, abs)));
                    } else {
                        term = Some(abs);
                    }
                }
                Token::OParen => {
                    let sub_term = parse_sub_term(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, sub_term)));
                    } else {
                        term = Some(sub_term);
                    }
                }
                Token::CParen => break,
                Token::LChar(c) => {
                    let var = Box::new(Term::Var(c.clone()));

                    if let Option::Some(old_term) = term {
                        term = Some(Box::new(Term::Appl(old_term, var)));
                    } else {
                        term = Some(var);
                    }
                }
            },
        }
    }

    match term {
        Option::None => bail!("no sub-term read"),
        Option::Some(t) => Ok(t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_appl_test() {
        let input = "xy";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            Term::Appl(Box::new(Term::Var('x')), Box::new(Term::Var('y'))),
            *term
        );

        let input = "xyz";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            Term::Appl(
                Box::new(Term::Appl(
                    Box::new(Term::Var('x')),
                    Box::new(Term::Var('y'))
                )),
                Box::new(Term::Var('z'))
            ),
            *term
        );
    }

    #[test]
    fn parse_sub_term_test() {
        let input = "(xy)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            Term::Appl(Box::new(Term::Var('x')), Box::new(Term::Var('y'))),
            *term
        );

        let input = "(xy)z";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            Term::Appl(
                Box::new(Term::Appl(
                    Box::new(Term::Var('x')),
                    Box::new(Term::Var('y'))
                )),
                Box::new(Term::Var('z'))
            ),
            *term
        );

        let input = "x(yz)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            Term::Appl(
                Box::new(Term::Var('x')),
                Box::new(Term::Appl(
                    Box::new(Term::Var('y')),
                    Box::new(Term::Var('z'))
                ))
            ),
            *term
        );
    }
}
