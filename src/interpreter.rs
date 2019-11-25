use crate::parser::{Ident, Expr};

use std::collections::HashMap;

struct State {
    functions: HashMap<Ident, Expr>,

}

enum Value {
    Text(String),
    Boolean(bool),
    Vector(Vec<i64>), // TODO: Big integerize?
    Empty,
}

fn interpret(state: &mut State, ast: &[Expr]) -> Value {
    // TODO: What should be the value of top level? Where you can define functions? Etc.
    use self::Expr::*;
    for expr in ast {
        match expr {
            Scope(e) => {},
            Op(lhs, op, rhs) => {},
            Conditional {
                condition,
                success,
                failure,
            } => {},
            Definition(name, body) => {},
            Call(name, params) => {},
            Param(param) => {},
            Text(text) => {},
            Vector(components) => {},
            WriteIO(src) => {},
            ReadIO => {},
        }
    }
}