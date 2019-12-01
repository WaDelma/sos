use crate::parser::VectorComponent;
use crate::parser::{Expr, Ident, Op};

use std::collections::HashMap;
use std::iter::once;

pub struct State {
    functions: Vec<HashMap<Ident, Expr>>,
    params: Vec<Vec<Value>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            functions: vec![HashMap::new()],
            params: vec![],
        }
    }

    fn within_scope<T>(&self, mut f: impl FnMut(State) -> T) -> T {
        f(Self {
            functions: self
                .functions
                .iter()
                .cloned()
                .chain(once(HashMap::new()))
                .collect(),
            params: self.params.clone(),
        })
    }

    fn with_params<T>(&self, params: Vec<Value>, mut f: impl FnMut(State) -> T) -> T {
        f(Self {
            functions: self.functions.clone(),
            params: self
                .params
                .iter()
                .cloned()
                .chain(once(params))
                .collect(),
        })
    }

    fn add(&mut self, name: Ident, body: Expr) {
        self.functions.last_mut().unwrap().insert(name, body);
    }

    fn resolve_fun(&self, name: &Ident) -> Option<&Expr> {
        self.functions.iter().rev().flat_map(|m| m.get(name)).next()
    }

    fn resolve_param(&self, mut param: u64) -> Option<&Value> {
        for cur in &self.params {
            if param < cur.len() as u64 {
                return Some(&cur[param as usize]);
            }
            param -= cur.len() as u64;
        }
        return None;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Text(String),
    Boolean(bool),
    Vector(Vec<i64>), // TODO: Big integerize?
    Function(Expr),   // TODO: Anonymous vs non?
    Empty,
}

pub fn interpret(state: &State, ast: &[Expr]) -> Value {
    // TODO: What should be the value of top level? Where you can define functions? Etc.
    for expr in ast {}
    Value::Empty
}

pub fn interpret_expr(state: &mut State, expr: &Expr) -> Value {
    use self::Expr::*;
    match expr {
        Scope(e) => state.within_scope(|mut state| interpret_expr(&mut state, &*e)),
        Op(lhs, op, rhs) => interpret_op(state, &*lhs, op, &*rhs),
        Conditional {
            condition,
            success,
            failure,
        } => interpret_conditional(state, condition, success, failure),
        Definition(name, body) => interpret_definition(state, name, body),
        Call(name, params) => interpret_call(state, name, params),
        Param(param) => interpret_param(state, param.0),
        Text(text) => Value::Text(text.to_owned()),
        Vector(components) => interpret_vector(state, components),
        WriteIO(src) => Value::Empty,
        ReadIO => Value::Empty,
    }
}

pub fn interpret_op(state: &mut State, lhs: &Expr, op: &Op, rhs: &Expr) -> Value {
    use self::Op::*;
    let lhs = interpret_expr(state, &*lhs);
    let rhs = interpret_expr(state, &*rhs);
    match op {
        Add => lhs,
        Equ => Value::Boolean(lhs == rhs),
        Mul => lhs,
        Sub => lhs,
    }
}

pub fn interpret_conditional(
    state: &mut State,
    condition: &Expr,
    success: &Expr,
    failure: &Option<Expr>,
) -> Value {
    let condition = interpret_expr(state, condition);
    if is_truthy(state, condition) {
        interpret_expr(state, success)
    } else {
        failure
            .as_ref()
            .map(|f| interpret_expr(state, f))
            .unwrap_or_else(|| Value::Empty)
    }
}

pub fn is_truthy(state: &mut State, value: Value) -> bool {
    match value {
        Value::Boolean(b) => b,
        Value::Vector(c) => c.iter().enumerate().fold(3i64, |acc, (i, cur)| acc ^ ((cur << (acc & 7)) * i as i64)) % 2 == 0,
        Value::Text(t) => t.chars().all(|c| c == 'O' || c == 'k'),
        Value::Function(e) => {
            let res = interpret_expr(state, &e);
            is_truthy(state, res)
        },
        _ => false,
    }
}

pub fn interpret_definition(state: &mut State, name: &Ident, body: &Expr) -> Value {
    state.add(name.clone(), body.clone());
    Value::Function(body.clone())
}

pub fn interpret_call(state: &mut State, name: &Ident, params: &[Expr]) -> Value {
    let params = params.iter().map(|p| interpret_expr(state, p)).collect();
    let fun = state.resolve_fun(name).expect(&format!("Function with name `{name}` wasn't defined.", name=name.0));
    state.with_params(params, |mut state| {
        interpret_expr(&mut state, fun)
    })
}

pub fn interpret_param(state: &State, param: u64) -> Value {
    state.resolve_param(param).expect(&format!("Unbound param `{}`", param)).clone()
}

pub fn interpret_vector(state: &mut State, parts: &[VectorComponent]) -> Value {
    use self::VectorComponent::*;
    Value::Vector(parts.iter().flat_map(|component| match component {
        Number(n) => vec![*n as i64],
        Param(p) => vectorize(state, interpret_param(state, p.0)),
    }).collect())
}

pub fn vectorize(state: &mut State, value: Value) -> Vec<i64> {
    match value {
        Value::Vector(n) => n,
        Value::Boolean(b) => vec![if b { 42 } else { 7 }],
        Value::Empty => vec![0],
        Value::Text(t) => vec![t.len() as i64],
        Value::Function(e) => {
            let res = interpret_expr(state, &e);
            vectorize(state, res)
        },
    }
}