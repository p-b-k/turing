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

pub fn _var(c: char) -> Box<Term> {
    Box::new(Term::Var(c))
}

pub fn _abs(c: char, t: Box<Term>) -> Box<Term> {
    Box::new(Term::Abs(c, t))
}

pub fn _appl(t1: Box<Term>, t2: Box<Term>) -> Box<Term> {
    Box::new(Term::Appl(t1, t2))
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
    // debug!("{:20}: Calling", "parse_term");
    let mut term: Option<Box<Term>> = Option::None;

    loop {
        let next_token = tstream.next();
        // debug!("{:20}: next = {next_token:?}", "parse_term");

        match next_token {
            Option::None => break,
            Option::Some(t) => match t {
                Token::Dot => {
                    bail!("parse error, unexpected '.'");
                }
                Token::Lambda => {
                    let abs = parse_abs(&mut tstream, true)?;
                    // debug!( "{:20}: parse_sub_term, parse_abs returned {:?}", "parse_term", abs );

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, abs));
                    } else {
                        term = Some(abs);
                    }
                }
                Token::OParen => {
                    let sub_term = parse_sub_term(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, sub_term));
                    } else {
                        term = Some(sub_term);
                    }
                }
                Token::CParen => {
                    bail!("parse error, unexpected ')'");
                }
                Token::LChar(c) => {
                    let var = _var(c.clone());

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, var));
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

fn parse_abs(tstream: &mut Iter<'_, Token>, top: bool) -> Result<Box<Term>> {
    // debug!("{:20}: Calling, top = {top}", "parse_abs");

    let next = tstream.next();

    match next {
        Option::None => bail!("Abstraction missing variable"),
        Option::Some(c) => {
            if Option::Some(&Token::Dot) == tstream.next() {
                let term = if top {
                    // debug!( "{:20}: calling parse_term because top is {top}", "parse_abs" );
                    parse_term(tstream)
                } else {
                    // debug!( "{:20}: calling parse_sub_term because top is {top}", "parse_abs" );
                    parse_sub_term(tstream)
                }?;

                match c {
                    Token::LChar(ch) => {
                        // debug!( "{:20}: Returning, top = {top}, c = {ch:?}, term = {term:?}", "parse_abs" );
                        Ok(_abs(*ch, term))
                    }
                    _ => bail!("Expected lower case character"),
                }
            } else {
                bail!("Expected dot")
            }
        }
    }
}

fn parse_sub_term(mut tstream: &mut Iter<'_, Token>) -> Result<Box<Term>> {
    // debug!("{:20}: Calling", "parse_sub_term");
    let mut term: Option<Box<Term>> = Option::None;

    loop {
        // debug!("{:20}: Top of loop", "parse_sub_term");
        let next_token = tstream.next();

        // debug!("{:20}: next = {next_token:?}", "parse_sub_term");
        match next_token {
            Option::None => bail!("Unexpected end of input"),
            Option::Some(t) => match t {
                Token::Dot => {
                    bail!("parse error, unexpected '.'");
                }
                Token::Lambda => {
                    let abs = parse_abs(&mut tstream, false)?;
                    // debug!("{:20}: parse_abs returned {:?}", "parse_sub_term", abs);

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, abs));
                    } else {
                        term = Some(abs);
                    }

                    break;
                }
                Token::OParen => {
                    let sub_term = parse_sub_term(&mut tstream)?;

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, sub_term));
                    } else {
                        term = Some(sub_term);
                    }
                }
                Token::CParen => {
                    // debug!( "{:20}: Close Parenthesis, expecting to close sub expression", "parse_sub_term" );
                    break;
                }
                Token::LChar(c) => {
                    let var = _var(c.clone());

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, var));
                    } else {
                        term = Some(var);
                    }
                }
            },
        }

        // debug!("{:20}: End of loop", "parse_sub_term");
    }

    match term {
        Option::None => bail!("no sub-term read"),
        Option::Some(t) => {
            // debug!("{:20}: Returning {:?}", "parse_sub_term", t);
            Ok(t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_test() {
        // Simple variable
        let input = "x";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_var('x'), term);

        // Parenthesis get parsed out
        let input = "(x)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_var('x'), term);

        // Extra parenthesis do nothing
        let input = "((x))";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_var('x'), term);
    }

    #[test]
    fn parse_appl_test() {
        let input = "xy";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_var('x'), _var('y')), term);

        let input = "xyz";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_appl(_var('x'), _var('y')), _var('z')), term);
    }

    #[test]
    fn parse_sub_term_test() {
        let input = "(xy)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_var('x'), _var('y')), term);

        let input = "(xy)z";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_appl(_var('x'), _var('y')), _var('z')), term);

        let input = "((xy)z)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_appl(_var('x'), _var('y')), _var('z')), term);

        let input = "((xy)z)y";
        let term = Term::parse(&input).unwrap();
        assert_eq!(
            _appl(_appl(_appl(_var('x'), _var('y')), _var('z')), _var('y')),
            term
        );

        let input = "x(yz)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_var('x'), _appl(_var('y'), _var('z'))), term);
    }

    #[test]
    fn parse_abs_test() {
        let input = "\\x.y";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _var('y')), term);

        let input = "\\x.yz";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _appl(_var('y'), _var('z'))), term);

        let input = "\\x.\\y.z";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _abs('y', _var('z'))), term);

        let input = "\\x.(\\y.z)";
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _abs('y', _var('z'))), term);
    }

    #[test]
    fn parse_complex_test() {
        let input = "\\x.(\\y.y(x(zz))\\z.xz)x";

        #[rustfmt::skip]
        let expected =
            _abs(
                'x',
                _appl(
                    _abs(
                        'y',
                        _appl(
                            _appl(
                                _var('y'),
                                _appl(
                                    _var('x'),
                                    _appl(
                                        _var('z'),
                                        _var('z')))),
                            _abs(
                                'z',
                                 _appl(
                                     _var('x'),
                                      _var('z'))),
                        ),
                    ),
                    _var('x'),
                ),
            );

        let term = Term::parse(&input).unwrap();

        assert_eq!(expected, term);
    }

    #[test]
    fn parse_lecture_test() {
        let input = "xyz";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_appl(_var('x'), _var('y')), _var('z')), term);

        let input = "\\x.\\y.z";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _abs('y', _var('z'))), term);

        let input = "\\x.yz";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        assert_eq!(_abs('x', _appl(_var('y'), _var('z'))), term);

        let input = "y\\x.z";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        assert_eq!(_appl(_var('y'), _abs('x', _var('z'))), term);
    }
}
