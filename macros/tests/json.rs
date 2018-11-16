extern crate gll;
extern crate gll_macros;
extern crate quote;

mod json_like {
    ::gll_macros::proc_macro_parser! {
        Value =
            | Null:"null"
            | False:"false"
            | True:"true"
            | Literal:LITERAL
            | Array:{ "[" elems:Value* % "," "]" }
            | Object:{ "{" fields:Field* % "," "}" }
            | InterpolateRust:{ "(" TOKEN_TREE+ ")" }
            ;
        Field = name:IDENT ":" value:Value;
    }
}

#[test]
fn json_like_proc_macro() {
    json_like::Value::parse_with(
        quote::quote! {
            // Example from `serde_json`.
            {
                name: "John Doe",
                age: 43,
                address: {
                    street: "10 Downing Street",
                    city: "London"
                },
                phones: [
                    "+44 1234567",
                    "+44 2345678"
                ],

                test: [null, false, true, (format!("{:?}", Some(1 + 2)))]
            }
        },
        |_, result| {
            let result = format!("{:#?}", result.unwrap());
            // HACK(eddyb) clean up the result, as we have no span info.
            let result = result
                .replace("Span..Span => ", "")
                .replace("Span..Span", "?");
            let expected = "\
Value::Object {
    fields: [
        Field {
            name: ?,
            value: Value::Literal(
                ?
            )
        },
        Field {
            name: ?,
            value: Value::Literal(
                ?
            )
        },
        Field {
            name: ?,
            value: Value::Object {
                fields: [
                    Field {
                        name: ?,
                        value: Value::Literal(
                            ?
                        )
                    },
                    Field {
                        name: ?,
                        value: Value::Literal(
                            ?
                        )
                    }
                ]
            }
        },
        Field {
            name: ?,
            value: Value::Array {
                elems: [
                    Value::Literal(
                        ?
                    ),
                    Value::Literal(
                        ?
                    )
                ]
            }
        },
        Field {
            name: ?,
            value: Value::Array {
                elems: [
                    Value::Null(
                        ?
                    ),
                    Value::False(
                        ?
                    ),
                    Value::True(
                        ?
                    ),
                    Value::InterpolateRust(
                        ?
                    )
                ]
            }
        }
    ]
}";
            assert!(
                result == expected,
                "mismatched output, expected:\n{}\n\nfound:\n{}",
                expected,
                result
            );
        },
    )
}
