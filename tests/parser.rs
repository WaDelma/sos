use sos::{parse, Expr, State, Ident, Op};

fn b<T>(t: T) -> Box<T> {
    Box::new(t)
}

fn r<'a, T>(t: T) -> nom::IResult<&'a str, T> {
    Ok(("", t))
}

#[test]
fn parse_numbers() {
    assert_eq!(r(vec![Expr::Number(1)]), parse(&State::default(), "."));
    assert_eq!(r(vec![Expr::Number(2)]), parse(&State::default(), ":"));
    assert_eq!(r(vec![Expr::Number(3)]), parse(&State::default(), ".:"));
    assert_eq!(r(vec![Expr::Number(4)]), parse(&State::default(), "::"));
    assert_eq!(r(vec![Expr::Number(5)]), parse(&State::default(), ".::"));
    assert_eq!(r(vec![Expr::Number(6)]), parse(&State::default(), ":::"));
    assert_eq!(r(vec![Expr::Number(7)]), parse(&State::default(), ".:::"));
}

#[test]
fn parse_conditional() {
    assert_eq!(
        r(vec![Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: b(None),
        }]),
        parse(&State::default(), "given that..")
    );
}

#[test]
fn parse_conditional_whitespace() {
    assert_eq!(
        r(vec![Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: b(None),
        }]),
        parse(&State::default(), "given that . . ")
    );
}

#[test]
fn parse_conditional_with_else() {
    assert_eq!(
        r(vec![Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: b(Some(Expr::Number(1))),
        }]),
        parse(&State::default(), "given that..otherwise.")
    );
}

#[test]
fn parse_conditional_with_else_whitespace() {
    assert_eq!(
        r(vec![Expr::Conditional {
            condition: b(Expr::Number(1)),
            success: b(Expr::Number(1)),
            failure: b(Some(Expr::Number(1))),
        }]),
        parse(&State::default(), "given that . . otherwise . ")
    );
}

#[test]
fn parse_function_definition() {
    assert_eq!(
        r(vec![Expr::Definition(Ident("öäå".into()), b(Expr::Number(1)))]),
        parse(&State::default(), "öäå¤.")
    );
}

#[test]
fn parse_function_definition_whitespace() {
    assert_eq!(
        r(vec![Expr::Definition(Ident("öäå".into()), b(Expr::Number(1)))]),
        parse(&State::default(), "öäå ¤ .")
    );
}

#[test]
fn parse_function_definition_with_param() {
    assert_eq!(
        r(vec![Expr::Definition(Ident("ö".into()), b(Expr::Param(1)))]),
        parse(&State::default(), r"ö ¤ \.")
    );
}

#[test]
fn parse_multiple_function_definitions() {
    assert_eq!(
        r(vec![
            Expr::Definition(Ident("ö".into()), b(Expr::Param(1))),
            Expr::Definition(Ident("ä".into()), b(Expr::Param(1))),
        ]),
        parse(&State::default(), r"ö ¤ \.
        ä ¤ \.")
    );
}

#[test]
fn parse_preceding_whitespace() {
    assert_eq!(
        r(vec![Expr::Number(1)]),
        parse(&State::default(), r"     .")
    );
}

#[test]
fn parse_scope() {
    assert_eq!(
        r(vec![Expr::Scope(b(Expr::Number(1)))]),
        parse(&State::default(), r"{.)")
    );
}

#[test]
fn parse_scope_whitespace() {
    assert_eq!(
        r(vec![Expr::Scope(b(Expr::Number(1)))]),
        parse(&State::default(), r"{ . ) ")
    );
}

#[test]
fn parse_scope_ending_to_eof() {
    assert_eq!(
        r(vec![Expr::Scope(b(Expr::Number(1)))]),
        parse(&State::default(), r"{.")
    );
}

#[test]
fn parse_scope_ending_to_linebreak() {
    assert_eq!(
        r(vec![Expr::Scope(b(Expr::Number(1)))]),
        parse(&State::default(), r"{.
        ")
    );
}

#[test]
fn parse_addition() {
    assert_eq!(
        r(vec![Expr::Op(b(Expr::Number(1)), Op::Add, b(Expr::Number(1)))]),
        parse(&State::default(), ".+.")
    )
}

#[test]
fn parse_addition_whitespace() {
    assert_eq!(
        r(vec![Expr::Op(b(Expr::Number(1)), Op::Add, b(Expr::Number(1)))]),
        parse(&State::default(), ". + .")
    )
}

#[test]
fn parse_equality() {
    assert_eq!(
        r(vec![Expr::Op(b(Expr::Number(1)), Op::Equ, b(Expr::Number(1)))]),
        parse(&State::default(), ".=.")
    )
}

#[test]
fn parse_equality_whitespace() {
    assert_eq!(
        r(vec![Expr::Op(b(Expr::Number(1)), Op::Equ, b(Expr::Number(1)))]),
        parse(&State::default(), ". = .")
    )
}


#[test]
fn parse_realer_function() {
    assert_eq!(
        r(vec![Expr::Definition(Ident("ö".into()), b(
            Expr::Op(
                b(Expr::Scope(b(Expr::Op(b(Expr::Param(1)), Op::Add, b(Expr::Param(2)))))), 
                Op::Mul,
                b(Expr::Op(b(Expr::Param(1)), Op::Add, b(Expr::Number(7))))
        )))]),
        parse(&State::default(), r"ö ¤ {\. + \:) * \. + .:::")
    )
}

#[test]
fn parse_writing_io() {
    assert_eq!(
        r(vec![Expr::WriteIO(b(Expr::Number(1)))]),
        parse(&State::default(), "@ << .")
    )
}

#[test]
fn parse_simple_text() {
    assert_eq!(
        r(vec![Expr::Text("simple".into())]),
        parse(&State::default(), "/simple")
    )
}

#[test]
fn parse_space_text() {
    assert_eq!(
        r(vec![Expr::Text("  ".into())]),
        parse(&State::default(), "/ / ")
    )
}

#[test]
fn parse_phrase_text() {
    assert_eq!(
        r(vec![Expr::Text("Hello, World!".into())]),
        parse(&State::default(), "/Hello,/ World!")
    )
}

#[test]
fn parse_slash_text() {
    assert_eq!(
        r(vec![Expr::Text("/".into())]),
        parse(&State::default(), "//")
    )
}

#[test]
fn parse_slash_space_text() {
    assert_eq!(
        r(vec![Expr::Text("/ ".into())]),
        parse(&State::default(), "// ")
    )
}

#[test]
fn parse_space_slash_text() {
    assert_eq!(
        r(vec![Expr::Text(" /".into())]),
        parse(&State::default(), "/ //")
    )
}

#[test]
fn parse_slashmiddle_text() {
    assert_eq!(
        r(vec![Expr::Text("eyey".into())]),
        parse(&State::default(), "/ey/ey")
    )
}

#[test]
fn parse_slashmiddle_space_text() {
    assert_eq!(
        r(vec![Expr::Text("ey/ ey".into())]),
        parse(&State::default(), "/ey// ey")
    )
}
#[test]
fn parse_text_ending_linebreak() {
    assert_eq!(
        r(vec![Expr::Text("aaa".into())]),
        parse(&State::default(), "/aaa
")
    )
}

#[test]
fn parse_printing_text_to_io() {
    assert_eq!(
        r(vec![Expr::WriteIO(b(Expr::Text("true".into())))]),
        parse(&State::default(), "@ << /true ")
    )
}

#[test]
fn parse_call_function_in_scope() {
        assert_eq!(
        r(vec![Expr::Scope(b(Expr::Call(Ident("ö".into()), vec![Expr::Number(1), Expr::Number(3)])))]),
        parse(&State::default(), "{ö . .:)")
    )
}

#[test]
fn parse_compare_equality_of_number_and_function_call() {
    assert_eq!(
        r(vec![Expr::Op(
            b(Expr::Number(24)),
            Op::Equ,
            b(Expr::Scope(b(Expr::Call(Ident("ö".into()), vec![Expr::Number(1), Expr::Number(3)]))))
        )]),
        parse(&State::default(), ":::::::::::: = {ö . .:)")
    )
}

#[test]
fn parse_condition_that_prints() {
    assert_eq!(
        r(vec![
            Expr::Conditional {
                condition: b(Expr::Op(
                    b(Expr::Number(2)),
                    Op::Equ,
                    b(Expr::Number(1)),
                )),
                success: b(
                    Expr::WriteIO(b(Expr::Text("true".into())))
                ),
                failure: b(
                    None
                )
            }
        ]),
        parse(&State::default(), r"given that : = . @ << /true")
    )
}

#[test]
fn parse_realer_expression() {
    assert_eq!(
        r(vec![
            Expr::Conditional {
                condition: b(Expr::Op(
                    b(Expr::Number(24)),
                    Op::Equ,
                    b(Expr::Scope(b(Expr::Call(Ident("ö".into()), vec![Expr::Number(1), Expr::Number(3)]))))
                )),
                success: b(
                    Expr::WriteIO(b(Expr::Text("true".into())))
                ),
                failure: b(
                    Some(Expr::WriteIO(b(Expr::Text("false".into()))))
                )
            }
        ]),
        parse(&State::default(), r"given that :::::::::::: = {ö . .:) @ << /true otherwise @ << /false")
    )
}

#[test]
fn parse_example() {
    assert_eq!(
        r(vec![
            Expr::Definition(Ident("ö".into()), b(
                Expr::Op(
                    b(Expr::Scope(b(Expr::Op(b(Expr::Param(1)), Op::Add, b(Expr::Param(2)))))), 
                    Op::Mul,
                    b(Expr::Op(b(Expr::Param(1)), Op::Add, b(Expr::Number(7))))
            ))),
            Expr::Conditional {
                condition: b(Expr::Op(
                    b(Expr::Number(24)),
                    Op::Equ,
                    b(Expr::Scope(b(Expr::Call(Ident("ö".into()), vec![Expr::Number(1), Expr::Number(3)]))))
                )),
                success: b(
                    Expr::WriteIO(b(Expr::Text("true".into())))
                ),
                failure: b(
                    Some(Expr::WriteIO(b(Expr::Text("false".into()))))
                )
            }
        ]),
        parse(&State::default(), r"
ö ¤ {\. + \:) * \. + .:::

given that :::::::::::: = {ö . .:) @ << /true otherwise @ << /false
        ")
    )
}