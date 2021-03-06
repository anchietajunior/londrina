use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, &str) {
        let s = if s.starts_with("let") {
            &s[3..]
        } else {
            panic!("expected let")
        };

        let (s, _) = utils::extract_whitespace(s);

        let (s, name) = utils::extract_ident(s); // Not Implemented
        let (s, _) = utils::extract_whitespace(s);

        let s = if s.starts_with('=') {
            &s[1..]
        } else {
            panic!("expected equals sign")
        };

        let (s, _) = utils::extract_whitespace(s);
        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}
