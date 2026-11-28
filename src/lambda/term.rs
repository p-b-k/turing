////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Lambda Calculus Term
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    slice::Iter,
};

use color_eyre::eyre::{Result, bail};

const LAMBDA_CHAR: char = '\u{03bb}';

#[derive(Debug, PartialEq)]
pub enum Term {
    Var(char),
    Abs(char, Box<Term>),
    Appl(Box<Term>, Box<Term>),
    Name(char),
}

pub fn _var(c: char) -> Box<Term> {
    Box::new(Term::Var(c))
}

pub fn _name(c: char) -> Box<Term> {
    Box::new(Term::Name(c))
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
    UChar(char),
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
            } else if c.is_whitespace() {
                // Do Nothing
            } else if c.is_ascii_lowercase() {
                tokens.push(Token::LChar(c));
            } else if c.is_ascii_uppercase() {
                tokens.push(Token::UChar(c));
            } else {
                bail!("Invalid input character: {c:?}");
            }
        }

        parse_term(&mut tokens.iter())
    }

    pub fn pretty_print(&self) -> String {
        let mut output = String::new();

        self.pretty_print_to_string(&mut output);

        output
    }

    fn pretty_print_to_string(&self, output: &mut String) {
        match self {
            Term::Var(x) => {
                output.push(*x);
            }
            Term::Name(x) => {
                output.push(*x);
            }
            Term::Appl(left, right) => {
                match left.as_ref() {
                    Term::Var(c) => {
                        output.push(*c);
                    }
                    Term::Name(c) => {
                        output.push(*c);
                    }
                    Term::Appl(_, _) => {
                        left.pretty_print_to_string(output);
                    }
                    Term::Abs(_, _) => {
                        output.push('(');
                        left.pretty_print_to_string(output);
                        output.push(')');
                    }
                }
                match right.as_ref() {
                    Term::Var(c) => {
                        output.push(*c);
                    }
                    Term::Name(c) => {
                        output.push(*c);
                    }
                    Term::Appl(_, _) => {
                        output.push('(');
                        right.pretty_print_to_string(output);
                        output.push(')');
                    }
                    Term::Abs(_, _) => {
                        right.pretty_print_to_string(output);
                    }
                }
            }
            Term::Abs(var, term) => {
                output.push(LAMBDA_CHAR);
                output.push(*var);
                output.push('.');
                term.pretty_print_to_string(output);
            }
        }
    }

    pub fn explicit_print(&self) -> String {
        let mut output = String::new();

        self.explicit_print_to_string(&mut output);

        output
    }

    fn explicit_print_to_string(&self, output: &mut String) {
        match self {
            Term::Var(x) => {
                output.push(*x);
            }
            Term::Name(x) => {
                output.push(*x);
            }
            Term::Appl(left, right) => {
                output.push('(');
                left.explicit_print_to_string(output);
                output.push(' ');
                right.explicit_print_to_string(output);
                output.push(')');
            }
            Term::Abs(var, term) => {
                output.push(LAMBDA_CHAR);
                output.push(*var);
                output.push('.');
                match term.as_ref() {
                    Term::Var(_) => term.explicit_print_to_string(output),
                    _ => {
                        output.push('(');
                        term.explicit_print_to_string(output);
                        output.push(')');
                    }
                }
            }
        }
    }

    pub fn free(&self) -> Vec<char> {
        match self {
            Term::Var(c) => Vec::from([*c]),
            Term::Abs(v, t) => remove(*v, &t.free()),
            Term::Appl(l, r) => merge(&l.free(), &r.free()),
            Term::Name(_) => Vec::new(),
        }
    }
}

pub fn remove(ch: char, vars: &Vec<char>) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    for c in vars {
        if *c != ch {
            v.push(*c);
        }
    }

    v
}

pub fn merge(target: &Vec<char>, source: &Vec<char>) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();

    for c in target {
        v.push(*c);
    }

    for c in source {
        if !target.contains(c) {
            v.push(*c);
        }
    }

    v
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
                    let var = _var(*c);

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, var));
                    } else {
                        term = Some(var);
                    }
                }
                Token::UChar(c) => {
                    let name = _name(*c);

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, name));
                    } else {
                        term = Some(name);
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
            let next = tstream.next();
            if Option::Some(&Token::Dot) == next {
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
                bail!("Expected dot, got {next:?}")
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
                    let var = _var(*c);

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, var));
                    } else {
                        term = Some(var);
                    }
                }
                Token::UChar(c) => {
                    let name = _name(*c);

                    if let Option::Some(old_term) = term {
                        term = Some(_appl(old_term, name));
                    } else {
                        term = Some(name);
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

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.pretty_print())
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
    fn parse_expr_test() {
        let input = "\\x.x\\y.y";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_appl(_appl(_var('x'), _name('N')), _var('y')), term);
        println!();

        // Force failure to see output
        // assert_eq!(0, 1);
    }

    #[test]
    fn parse_name_test() {
        let input = "xNy";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_appl(_appl(_var('x'), _name('N')), _var('y')), term);
        println!();

        let input = "\\x.Ny";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_abs('x', _appl(_name('N'), _var('y'))), term);
        println!();

        let input = "NN";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_appl(_name('N'), _name('N')), term);
        println!();
    }

    #[test]
    fn parse_lecture_test() {
        let input = "xyz";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_appl(_appl(_var('x'), _var('y')), _var('z')), term);
        println!();

        let input = "\\x.\\y.z";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_abs('x', _abs('y', _var('z'))), term);
        println!();

        let input = "\\x.yz";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_abs('x', _appl(_var('y'), _var('z'))), term);
        println!();

        let input = "y\\x.z";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        assert_eq!(_appl(_var('y'), _abs('x', _var('z'))), term);
        println!();

        let input = "(\\x.\\y.xyxz)(abc)";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        #[rustfmt::skip]
        assert_eq!(
            _appl(
                _abs(
                    'x',
                    _abs(
                        'y',
                        _appl(
                            _appl(
                                _appl(
                                    _var('x'),
                                    _var('y')),
                                _var('x')),
                            _var('z')))),
                _appl(
                    _appl(
                       _var('a'),
                       _var('b'),
                    ),
                    _var('c'))),
            term);
        println!();

        let input = "(\\x.(\\y.xyxz)(ab))c";
        println!("input = {input}");
        let term = Term::parse(&input).unwrap();
        println!("term  = {term}");
        println!("also  = {}", term.explicit_print());
        #[rustfmt::skip]
        assert_eq!(
            _appl(
                _abs(
                    'x',
                    _appl(
                        _abs(
                            'y',
                            _appl(
                                _appl(
                                    _appl(
                                        _var('x'),
                                        _var('y')
                                    ),
                                    _var('x')
                                ),
                                _var('z')
                            )
                        ),
                        _appl(
                            _var('a'),
                            _var('b')
                        )
                    )
                ),
                _var('c')),
            term);
        println!();

        // Force failure to see output
        // assert_eq!(0, 1);
    }
}
