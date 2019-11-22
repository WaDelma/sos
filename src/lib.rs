use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::{error::ErrorKind, IResult};

use std::cell::RefCell;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Op(Op),
    Conditional {
        condition: Box<Expr>,
        success: Box<Expr>,
        failure: Option<Box<Expr>>,
    },
    Text(String),
    Number(i64),
    WriteIO(Box<Expr>),
    ReadIO,
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

pub const ifs: &[&str] = &[
    "given that",
    "assuming that",
    "conceding that",
    "granted that",
    "in case that",
    "on the assumption that",
    "on the occasion that",
    "supposing that",
    "whenever",
    "wherever",
    "with the condition that",
];

pub const elses: &[&str] = &[
    "otherwise",
    "differently",
    "any other way",
    "contrarily",
    "diversely",
    "elseways",
    "if not",
    "in different circumstances",
    "on the other hand",
    "or else",
    "or then",
    "under other conditions",
    "variously",
];

pub struct State {
    used_conditionals: RefCell<HashSet<usize>>,
    used_elses: RefCell<HashSet<usize>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            used_conditionals: RefCell::new(HashSet::new()),
            used_elses: RefCell::new(HashSet::new()),
        }
    }
}

//🆘

pub fn ws<T>(parser: impl Fn(&str) -> IResult<&str, T>) -> impl Fn(&str) -> IResult<&str, T> {
    move |code| {
        let (code, res) = parser(code)?;
        let (code, _) = space0(code)?;
        Ok((code, res))
    }
}

pub fn cond<'a>(
    used: &'a RefCell<HashSet<usize>>,
    variants: &'a [&'a str],
) -> impl Fn(&str) -> IResult<&str, &str> + 'a {
    move |code| {
        if used.borrow().len() == variants.len() {
            used.borrow_mut().clear();
        }
        for (n, i) in variants.iter().enumerate() {
            if let ok @ Ok(_) = tag(*i)(code) {
                if used.borrow().contains(&n) {
                    // TODO: Error handling
                    return Err(nom::Err::Failure((code, ErrorKind::Verify)));
                }
                return ok;
            }
        }
        // TODO: Error handling
        Err(nom::Err::Error((code, ErrorKind::Verify)))
    }
}

pub fn otherwise<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Box<Expr>> + 'a {
    move |code| {
        let (code, _) = cond(&state.used_elses, &elses)(code)?;
        let (code, failure) = expr(state, code)?;
        Ok((code, failure))
    }
}

pub fn paren_start(code: &str) -> IResult<&str, &str> {
    tag("{")(code)
}

pub fn conditional<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, _) = ws(cond(&state.used_conditionals, ifs))(code)?;
        let (code, condition) = expr(state, code)?;
        let (code, _) = paren_start(code)?;
        let (code, success) = expr(state, code)?;
        let (code, failure) = opt(otherwise(state))(code)?;
        Ok((
            code,
            Expr::Conditional {
                condition,
                success,
                failure,
            },
        ))
    }
}

pub fn number(code: &str) -> IResult<&str, Expr> {
    let (code, s) = alt((tag("."), tag(":")))(code)?;
    let n = if s == "." { 1 } else { 2 };
    let (code, m) = many0_count(tag(":"))(code)?;
    Ok((code, Expr::Number(n + 2 * m as i64)))
}

pub fn expr<'a>(state: &State, code: &'a str) -> IResult<&'a str, Box<Expr>> {
    map(alt((conditional(state), number)), Box::new)(code)
}
