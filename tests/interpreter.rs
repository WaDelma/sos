use sos::interpreter::{self, State, Value};
use sos::parser;

fn interpret_expr(code: &str) -> Value {
    let res = parser::expr(&parser::State::default())(code)
        .expect("Parsing failed")
        .1;
    interpreter::interpret_expr(&mut State::new(), &res)
}

#[test]
fn interpret_numbers() {
    assert_eq!(Value::Vector(vec![1]), interpret_expr("."));
    assert_eq!(Value::Vector(vec![2]), interpret_expr(":"));
    assert_eq!(Value::Vector(vec![3]), interpret_expr(".:"));
    assert_eq!(Value::Vector(vec![4]), interpret_expr("::"));
    assert_eq!(Value::Vector(vec![5]), interpret_expr(".::"));
    assert_eq!(Value::Vector(vec![6]), interpret_expr(":::"));
    assert_eq!(Value::Vector(vec![7]), interpret_expr(".:::"));
}

#[test]
fn interpret_vectors() {
    assert_eq!(Value::Vector(vec![1, 2]), interpret_expr(". :"));
    assert_eq!(Value::Vector(vec![1, 2, 3]), interpret_expr(". : .:"));
    assert_eq!(Value::Vector(vec![1, 2, 3, 4]), interpret_expr(". : .: ::"));
}

#[test]
fn interpret_equality_of_integers() {
    assert_eq!(Value::Boolean(true), interpret_expr(". = ."));
    assert_eq!(Value::Boolean(false), interpret_expr(". = :"));
    assert_eq!(Value::Boolean(false), interpret_expr(". = .:"));
    assert_eq!(Value::Boolean(false), interpret_expr(". = ::"));

    assert_eq!(Value::Boolean(false), interpret_expr(": = ."));
    assert_eq!(Value::Boolean(true), interpret_expr(": = :"));
    assert_eq!(Value::Boolean(false), interpret_expr(": = .:"));
    assert_eq!(Value::Boolean(false), interpret_expr(": = ::"));

    assert_eq!(Value::Boolean(false), interpret_expr(".: = ."));
    assert_eq!(Value::Boolean(false), interpret_expr(".: = :"));
    assert_eq!(Value::Boolean(true), interpret_expr(".: = .:"));
    assert_eq!(Value::Boolean(false), interpret_expr(".: = ::"));

    assert_eq!(Value::Boolean(false), interpret_expr(":: = ."));
    assert_eq!(Value::Boolean(false), interpret_expr(":: = :"));
    assert_eq!(Value::Boolean(false), interpret_expr(":: = .:"));
    assert_eq!(Value::Boolean(true), interpret_expr(":: = ::"));
}

#[test]
fn interpret_equality_of_vectors() {
    assert_eq!(Value::Boolean(true), interpret_expr(". : = . :"));
    assert_eq!(Value::Boolean(false), interpret_expr(". : = : :"));
    assert_eq!(Value::Boolean(false), interpret_expr(". : = ."));
}

#[test]
fn interpret_equality_of_functions() {
    assert_eq!(
        Value::Boolean(true),
        interpret_expr(r#"{ö ¤ \. + \:) = {ä ¤ \. + \:)"#)
    );
    assert_eq!(
        Value::Boolean(false),
        interpret_expr(r#"{ö ¤ \. + \:) = {ä ¤ \: + \:)"#)
    );
    assert_eq!(
        Value::Boolean(true),
        interpret_expr(r#"{ö ¤ å ¤ \.) = {ä ¤ 🆘 ¤ \.)"#)
    );
}

#[test]
fn interpret_addition_of_numbers() {
    assert_eq!(Value::Vector(vec![2]), interpret_expr(r#". + ."#));
    assert_eq!(Value::Vector(vec![3]), interpret_expr(r#". + :"#));
    assert_eq!(Value::Vector(vec![3]), interpret_expr(r#": + ."#));
    assert_eq!(Value::Vector(vec![4]), interpret_expr(r#": + :"#));
}
