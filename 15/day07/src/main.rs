use std::collections::HashMap;

enum WireTerm<'a> {
    Constant(u32),
    Variable(&'a str),
}

enum WireExpression<'a> {
    Term(&'a WireTerm<'a>),
    And(&'a WireTerm<'a>, &'a WireTerm<'a>),
    Or(&'a WireTerm<'a>, &'a WireTerm<'a>),
    LShift(&'a WireTerm<'a>, u32),
    RShift(&'a WireTerm<'a>, u32),
    Not(&'a WireTerm<'a>),
}

struct Environment<'a> {
    // Technically a binding from variable to expression only
    bindings: HashMap<&'a WireTerm<'a>, &'a WireExpression<'a>>,
}

impl<'a> Environment<'a> {
    fn new() -> Environment<'a> {
        Environment {
            bindings: HashMap::new(),
        }
    }

    fn add_binding(&mut self, variable: &WireTerm, expr: &WireExpression) {
        self.bindings.insert(variable, expr);
    }
}

trait Evaluate {
    fn eval(&self, environment: &Environment) -> u32;
}

impl Evaluate for WireExpression<'_> {
    fn eval(&self, environment: &Environment) -> u32 {
        match self {
            WireExpression::Term(WireTerm::Constant(v)) => *v,
            WireExpression::Term(WireTerm::Variable(_)) => {
                todo!()
            }
            WireExpression::And(_, _) => todo!(),
            WireExpression::Or(_, _) => todo!(),
            WireExpression::LShift(_, _) => todo!(),
            WireExpression::RShift(_, _) => todo!(),
            WireExpression::Not(_) => todo!(),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{Environment, Evaluate, WireExpression, WireTerm};

    #[test]
    fn test_wire_expr_constant() {
        let expr = WireExpression::Term(&WireTerm::Constant(123));
        let environ = Environment::new();

        assert_eq!(expr.eval(&environ), 123);
    }

    fn test_wire_expr_var() {
        let expr = WireExpression::Term(&WireTerm::Constant(123));
        let environ = Environment::new();

        assert_eq!(expr.eval(&environ), 123);
    }
}
