use sos::{expr, Expr, State};

fn b<T>(t: T) -> Box<T> {
    Box::new(t)
}

fn r<'a, T>(t: T) -> nom::IResult<&'a str, Box<T>> {
    Ok(("", b(t)))
}

#[test]
fn parse_numbers() {
    assert_eq!(r(Expr::Number(1)), expr(&State::default(), "."));
    assert_eq!(r(Expr::Number(2)), expr(&State::default(), ":"));
    assert_eq!(r(Expr::Number(3)), expr(&State::default(), ".:"));
    assert_eq!(r(Expr::Number(4)), expr(&State::default(), "::"));
    assert_eq!(r(Expr::Number(5)), expr(&State::default(), ".::"));
    assert_eq!(r(Expr::Number(6)), expr(&State::default(), ":::"));
    assert_eq!(r(Expr::Number(7)), expr(&State::default(), ".:::"));
}

#[test]
fn parse_conditional_without_else() {
    assert_eq!(
        r(Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: None,
        }),
        expr(&State::default(), "given that.{.")
    );
}

#[test]
fn parse_conditional_without_else_whitespace() {
    assert_eq!(
        r(Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: None,
        }),
        expr(&State::default(), "given that . { . ")
    );
}

#[test]
fn parse_conditional_with_else() {
    assert_eq!(
        r(Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: Some(b(Expr::Number(1))),
        }),
        expr(&State::default(), "given that.{.otherwise.")
    );
}
