use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::sequence::*;
use nom::combinator::*;
use nom::multi::*;
use nom::{error::{ParseError, ErrorKind}, IResult, InputLength};

use std::cell::RefCell;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug)]
pub enum Expr {
    Scope(Box<Expr>),
    Op(Box<Expr>, Op, Box<Expr>),
    Conditional {
        condition: Box<Expr>,
        success: Box<Expr>,
        failure: Box<Option<Expr>>,
    },
    Definition(Ident, Box<Expr>),
    Call(Ident, Vec<Expr>),
    Param(Number),
    Text(String),
    Number(Number),
    WriteIO(Box<Expr>),
    ReadIO,
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Equ,
    Add,
    Mul,
    Sub,
}

type Number = u64;

pub const IFS: &[&str] = &[
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

pub const ELSES: &[&str] = &[
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
    level: u64,
}

impl Default for State {
    fn default() -> Self {
        State {
            used_conditionals: RefCell::new(HashSet::new()),
            used_elses: RefCell::new(HashSet::new()),
            level: 0,
        }
    }
}

//🆘

pub fn ws<'a, T: 'a>(parser: impl Fn(&'a str) -> IResult<&'a str, T>) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    terminated(parser, space0)
}

pub fn eof<I: Copy + InputLength, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
        Ok((input, input))
    } else {
        Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
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

pub fn otherwise<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, _) = ws(cond(&state.used_elses, &ELSES))(code)?;
        let (code, failure) = expr(state)(code)?;
        Ok((code, failure))
    }
}

pub fn paren_start(code: &str) -> IResult<&str, &str> {
    ws(tag("{"))(code)
}

pub fn paren_end(code: &str) -> IResult<&str, &str> {
    ws(tag(")"))(code)
}

pub fn conditional<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, _) = ws(cond(&state.used_conditionals, IFS))(code)?;
        let (code, condition) = expr(state)(code)?;
        let (code, success) = expr(state)(code)?;
        let (code, failure) = opt(otherwise(state))(code)?;
        Ok((
            code,
            Expr::Conditional {
                condition: Box::new(condition),
                success: Box::new(success),
                failure: Box::new(failure),
            },
        ))
    }
}

pub fn number(code: &str) -> IResult<&str, Number> {
    let (code, s) = alt((tag("."), tag(":")))(code)?;
    let n = if s == "." { 1 } else { 2 };
    let (code, m) = many0_count(tag(":"))(code)?;
    Ok((code, n + 2 * m as Number))
}

pub fn param(code: &str) -> IResult<&str, Expr> {
    let (code, _) = tag(r"\")(code)?;
    let (code, n) = number(code)?;
    Ok((code, Expr::Param(n)))
}

pub fn ident(code: &str) -> IResult<&str, Ident> {
    map(
        take_while1(|c: char| !c.is_ascii() && c.is_alphanumeric()),
        |s: &str| Ident(s.into())
    )(code)
}

pub fn fundef<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, name) = ws(ident)(code)?;
        let (code, _) = ws(tag("¤"))(code)?;
        let (code, body) = expr(state)(code)?;
        Ok((code, Expr::Definition(name, Box::new(body))))
    }
}

pub fn funcall<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, name) = ws(ident)(code)?;
        let (code, params) = many0(expr(state))(code)?;
        Ok((code, Expr::Call(name, params)))
    }
}

pub fn scope<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, _) = paren_start(code)?;
        let (code, body) = expr(state)(code)?;
        let (code, _) = alt((
            paren_end,
            peek(line_ending),
            eof
        ))(code)?;
        Ok((code, Expr::Scope(Box::new(body))))
    }
}

pub fn oper<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, (Op, Box<Expr>)> + 'a {
    move |code| {
        let (code, op) = ws(alt((
            map(tag("="), |_| Op::Equ),
            map(tag("*"), |_| Op::Mul),
            map(tag("+"), |_| Op::Add),
            map(tag("-"), |_| Op::Sub)
        )))(code)?;
        let (code, rhs) = expr(state)(code)?;
        Ok((code, (op, Box::new(rhs))))
    }
}

pub fn write_io<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, _) = ws(tag("@"))(code)?;
        let (code, _) = ws(tag("<<"))(code)?;
        let (code, rhs) = expr(state)(code)?;
        Ok((code, Expr::WriteIO(Box::new(rhs))))
    }
}

pub fn text(mut code1: &str) -> IResult<&str, Expr> {
    let mut s = String::new();
    loop {
        let (code, _) = tag("/")(code1)?;
        let (code, t) = opt(tag("/"))(code)?;
        if t.is_some() {
            s.push('/');
        }
        let (code, t) = opt(tag(" "))(code)?;
        if t.is_some() {
            s.push(' ');
        }
        let (code, part) = take_while(|c: char| c != ' ' && c != '/' && c != '\r' && c != '\n')(code)?;
        s.push_str(part);
        code1 = code;
        if code.is_empty() {
            break;
        }
        let c = code.chars().next().unwrap();
        if c == ' ' || c == '\r' || c == '\n' {
            break;
        }
    }
    Ok((code1, Expr::Text(s)))
}

pub fn expr<'a>(state: &'a State) -> impl Fn(&str) -> IResult<&str, Expr> + 'a {
    move |code| {
        let (code, expr) = ws(alt((fundef(state), funcall(state), text, write_io(state), scope(state), conditional(state), param, map(number, Expr::Number))))(code)?;
        let (code, oper) = opt(oper(state))(code)?;
        if let Some((op, rhs)) = oper {
            Ok((code, Expr::Op(Box::new(expr), op, rhs)))
        } else {
            Ok((code, expr))
        }
    }
}

pub fn parse<'a>(state: &State, code: &'a str) -> IResult<&'a str, Vec<Expr>> {
    delimited(multispace0, separated_list(multispace1, expr(state)), multispace0)(code)
}
