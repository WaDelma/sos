use crate::parser::{Expr, Ident, Op};

use std::collections::HashMap;
use std::iter::once;

struct State {
    functions: Vec<HashMap<Ident, Expr>>,
}

impl State {
    fn new() -> Self {
        Self {
            functions: vec![HashMap::new()],
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
        })
    }

    fn add(&mut self, name: Ident, body: Expr) {
        self.functions.last_mut().unwrap().insert(name, body);
    }
}

enum Value {
    Text(String),
    Boolean(bool),
    Vector(Vec<i64>), // TODO: Big integerize?
    Function(Expr),   // TODO: Anonymous vs non?
    Empty,
}

fn interpret(state: &State, ast: &[Expr]) -> Value {
    // TODO: What should be the value of top level? Where you can define functions? Etc.
    for expr in ast {}
    Value::Empty
}

fn interpret_expr(state: &mut State, expr: &Expr) -> Value {
    use self::Expr::*;
    match expr {
        Scope(e) => state.within_scope(|state| interpret_expr(&mut state, &*e)),
        Op(lhs, op, rhs) => interpret_op(state, &*lhs, op, &*rhs),
        Conditional {
            condition,
            success,
            failure,
        } => interpret_conditional(state, condition, success, failure),
        Definition(name, body) => interpret_definition(state, name, body),
        Call(name, params) => {}
        Param(param) => {}
        Text(text) => {}
        Vector(components) => {}
        WriteIO(src) => {}
        ReadIO => {}
    }
}

fn interpret_op(state: &mut State, lhs: &Expr, op: &Op, rhs: &Expr) -> Value {
    use self::Op::*;
    let lhs = interpret_expr(state, &*lhs);
    let rhs = interpret_expr(state, &*rhs);
    match op {
        Add => lhs,
        Equ => lhs,
        Mul => lhs,
        Sub => lhs,
    }
}

fn interpret_conditional(
    state: &mut State,
    condition: &Expr,
    success: &Expr,
    failure: &Option<Expr>,
) -> Value {
    let condition = interpret_expr(state, condition);
    if is_truthy(condition) {
        interpret_expr(state, success)
    } else {
        failure
            .map(|f| interpret_expr(state, &f))
            .unwrap_or_else(|| Value::Empty)
    }
}

fn is_truthy(value: Value) -> bool {
    true
}

fn interpret_definition(state: &mut State, name: &Ident, body: &Expr) -> Value {
    state.add(name.clone(), body.clone());
    Value::Function(body.clone())
}
