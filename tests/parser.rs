use sos::parser::{parse, Expr as E, State, Ident, Op, VectorComponent, Param};

fn b<T>(t: T) -> Box<T> {
    Box::new(t)
}

fn r<'a, T>(t: T) -> nom::IResult<&'a str, T> {
    Ok(("", t))
}

fn number(n: u64) -> E {
    E::Vector(vec![VectorComponent::Number(n)])
}

fn param(n: u64) -> E {
    E::Param(Param(n))
}

fn scope(e: E) -> E {
    E::Scope(b(e))
}

#[test]
fn parse_numbers() {
    assert_eq!(r(vec![number(1)]), parse(&State::default(), "."));
    assert_eq!(r(vec![number(2)]), parse(&State::default(), ":"));
    assert_eq!(r(vec![number(3)]), parse(&State::default(), ".:"));
    assert_eq!(r(vec![number(4)]), parse(&State::default(), "::"));
    assert_eq!(r(vec![number(5)]), parse(&State::default(), ".::"));
    assert_eq!(r(vec![number(6)]), parse(&State::default(), ":::"));
    assert_eq!(r(vec![number(7)]), parse(&State::default(), ".:::"));
}

#[test]
fn parse_vectors() {
    assert_eq!(r(vec![E::Vector(vec![
        VectorComponent::Number(1),
        VectorComponent::Number(2),
    ])]), parse(&State::default(), ". :"));
    assert_eq!(r(vec![E::Vector(vec![
        VectorComponent::Number(1),
        VectorComponent::Number(2),
        VectorComponent::Number(3),
    ])]), parse(&State::default(), ". : .:"));
    assert_eq!(r(vec![E::Vector(vec![
        VectorComponent::Number(1),
        VectorComponent::Number(2),
        VectorComponent::Number(3),
        VectorComponent::Number(4),
    ])]), parse(&State::default(), ". : .: ::"));
}


#[test]
fn parse_conditional() {
    assert_eq!(
        r(vec![E::Conditional {
            condition: b(number(1)),
            success: b(scope(number(1))),
            failure: b(None),
        }]),
        parse(&State::default(), "given that.{.")
    );
}

#[test]
fn parse_conditional_whitespace() {
    assert_eq!(
        r(vec![E::Conditional {
            condition: b(number(1)),
            success: b(scope(number(1))),
            failure: b(None),
        }]),
        parse(&State::default(), "given that . { . ")
    );
}

#[test]
fn parse_conditional_with_else() {
    assert_eq!(
        r(vec![E::Conditional {
            condition: b(number(1)),
            success: b(scope(number(1))),
            failure: b(Some(number(1))),
        }]),
        parse(&State::default(), "given that.{.)otherwise.")
    );
}

#[test]
fn parse_conditional_with_else_whitespace() {
    assert_eq!(
        r(vec![E::Conditional {
            condition: b(number(1)),
            success: b(scope(number(1))),
            failure: b(Some(number(1))),
        }]),
        parse(&State::default(), "given that . { . ) otherwise . ")
    );
}

#[test]
fn parse_function_definition() {
    assert_eq!(
        r(vec![E::Definition(Ident("öäå".into()), b(number(1)))]),
        parse(&State::default(), "öäå¤.")
    );
}

#[test]
fn parse_function_definition_whitespace() {
    assert_eq!(
        r(vec![E::Definition(Ident("öäå".into()), b(number(1)))]),
        parse(&State::default(), "öäå ¤ .")
    );
}

#[test]
fn parse_function_definition_with_param() {
    assert_eq!(
        r(vec![E::Definition(Ident("ö".into()), b(param(1)))]),
        parse(&State::default(), r"ö ¤ \.")
    );
}

#[test]
fn parse_function_definition_with_all_params_as_vector() {
    assert_eq!(
        r(vec![E::Definition(Ident("ö".into()), b(param(0)))]),
        parse(&State::default(), r"ö ¤ \\")
    );
}

#[test]
fn parse_multiple_function_definitions() {
    assert_eq!(
        r(vec![
            E::Definition(Ident("ö".into()), b(param(1))),
            E::Definition(Ident("ä".into()), b(param(1))),
        ]),
        parse(&State::default(), r"ö ¤ \.
        ä ¤ \.")
    );
}

#[test]
fn parse_preceding_whitespace() {
    assert_eq!(
        r(vec![number(1)]),
        parse(&State::default(), r"     .")
    );
}

#[test]
fn parse_scope() {
    assert_eq!(
        r(vec![scope(number(1))]),
        parse(&State::default(), r"{.)")
    );
}

#[test]
fn parse_scope_whitespace() {
    assert_eq!(
        r(vec![scope(number(1))]),
        parse(&State::default(), r"{ . ) ")
    );
}

#[test]
fn parse_scope_ending_to_eof() {
    assert_eq!(
        r(vec![scope(number(1))]),
        parse(&State::default(), r"{.")
    );
}

#[test]
fn parse_scope_ending_to_linebreak() {
    assert_eq!(
        r(vec![scope(number(1))]),
        parse(&State::default(), r"{.
        ")
    );
}

#[test]
fn parse_addition() {
    assert_eq!(
        r(vec![E::Op(b(number(1)), Op::Add, b(number(1)))]),
        parse(&State::default(), ".+.")
    )
}

#[test]
fn parse_addition_whitespace() {
    assert_eq!(
        r(vec![E::Op(b(number(1)), Op::Add, b(number(1)))]),
        parse(&State::default(), ". + .")
    )
}

#[test]
fn parse_equality() {
    assert_eq!(
        r(vec![E::Op(b(number(1)), Op::Equ, b(number(1)))]),
        parse(&State::default(), ".=.")
    )
}

#[test]
fn parse_equality_whitespace() {
    assert_eq!(
        r(vec![E::Op(b(number(1)), Op::Equ, b(number(1)))]),
        parse(&State::default(), ". = .")
    )
}


#[test]
fn parse_realer_function() {
    assert_eq!(
        r(vec![E::Definition(Ident("ö".into()), b(
            E::Op(
                b(scope(E::Op(
                    b(param(1)),
                    Op::Add,
                    b(param(2))
                ))), 
                Op::Mul,
                b(E::Op(b(param(1)), Op::Add, b(number(7))))
        )))]),
        parse(&State::default(), r"ö ¤ {\. + \:) * \. + .:::")
    )
}

#[test]
fn parse_writing_io() {
    assert_eq!(
        r(vec![E::WriteIO(b(number(1)))]),
        parse(&State::default(), "@ << .")
    )
}

#[test]
fn parse_simple_text() {
    assert_eq!(
        r(vec![E::Text("simple".into())]),
        parse(&State::default(), "/simple")
    )
}

#[test]
fn parse_space_text() {
    assert_eq!(
        r(vec![E::Text("  ".into())]),
        parse(&State::default(), "/ / ")
    )
}

#[test]
fn parse_phrase_text() {
    assert_eq!(
        r(vec![E::Text("Hello, World!".into())]),
        parse(&State::default(), "/Hello,/ World!")
    )
}

#[test]
fn parse_slash_text() {
    assert_eq!(
        r(vec![E::Text("/".into())]),
        parse(&State::default(), "//")
    )
}

#[test]
fn parse_slash_space_text() {
    assert_eq!(
        r(vec![E::Text("/ ".into())]),
        parse(&State::default(), "// ")
    )
}

#[test]
fn parse_space_slash_text() {
    assert_eq!(
        r(vec![E::Text(" /".into())]),
        parse(&State::default(), "/ //")
    )
}

#[test]
fn parse_slashmiddle_text() {
    assert_eq!(
        r(vec![E::Text("eyey".into())]),
        parse(&State::default(), "/ey/ey")
    )
}

#[test]
fn parse_slashmiddle_space_text() {
    assert_eq!(
        r(vec![E::Text("ey/ ey".into())]),
        parse(&State::default(), "/ey// ey")
    )
}
#[test]
fn parse_text_ending_linebreak() {
    assert_eq!(
        r(vec![E::Text("aaa".into())]),
        parse(&State::default(), "/aaa
")
    )
}

#[test]
fn parse_printing_text_to_io() {
    assert_eq!(
        r(vec![E::WriteIO(b(E::Text("true".into())))]),
        parse(&State::default(), "@ << /true ")
    )
}

#[test]
fn parse_call_function_in_scope() {
        assert_eq!(
        r(vec![scope(E::Call(
            Ident("ö".into()),
            vec![
                scope(number(1)),
                number(3)
            ]
        ))]),
        parse(&State::default(), "{ö { . ) .:)")
    )
}

#[test]
fn parse_compare_equality_of_number_and_function_call() {
    assert_eq!(
        r(vec![E::Op(
            b(number(24)),
            Op::Equ,
            b(scope(E::Call(
                Ident("ö".into()),
                vec![
                    number(1),
                    scope(number(3))
                ]
            )))
        )]),
        parse(&State::default(), ":::::::::::: = {ö . { .:")
    )
}

#[test]
fn parse_condition_that_prints() {
    assert_eq!(
        r(vec![
            E::Conditional {
                condition: b(E::Op(
                    b(number(2)),
                    Op::Equ,
                    b(number(1)),
                )),
                success: b(
                    E::WriteIO(b(E::Text("true".into())))
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
            E::Conditional {
                condition: b(E::Op(
                    b(number(24)),
                    Op::Equ,
                    b(scope(E::Call(
                        Ident("ö".into()),
                        vec![
                            // TODO: Is the parameter always a vector?
                            E::Vector(vec![
                                VectorComponent::Number(1),
                                VectorComponent::Number(3),
                            ])
                        ])
                    ))
                )),
                success: b(
                    E::WriteIO(b(E::Text("true".into())))
                ),
                failure: b(
                    Some(E::WriteIO(b(E::Text("false".into()))))
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
            E::Definition(Ident("ö".into()), b(
                E::Op(
                    b(scope(E::Op(
                        b(param(1)),
                        Op::Add,
                        b(param(2))
                    ))), 
                    Op::Mul,
                    b(E::Op(b(param(1)), Op::Add, b(number(7))))
            ))),
            E::Conditional {
                condition: b(E::Op(
                    b(number(24)),
                    Op::Equ,
                    b(scope(E::Call(
                        Ident("ö".into()),
                        vec![
                            E::Vector(vec![
                                VectorComponent::Number(1),
                                VectorComponent::Number(3),
                            ])
                        ]
                    )))
                )),
                success: b(
                    E::WriteIO(b(E::Text("true".into())))
                ),
                failure: b(
                    Some(E::WriteIO(b(E::Text("false".into()))))
                )
            }
        ]),
        parse(&State::default(), r"
ö ¤ {\. + \:) * \. + .:::

given that :::::::::::: = {ö . .:) @ << /true otherwise @ << /false
        ")
    )
}