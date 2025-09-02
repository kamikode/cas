use crate::fmt::Latex;

use core::fmt;

/// A mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A symbol, like 'x'.
    Symbol(String),
    /// Binary addition operation.
    Add(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Symbol(s) => write!(f, "{s}"),
            Expression::Add(a, b) => write!(f, "({a} + {b})"),
        }
    }
}

impl Latex for Expression {
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Symbol(s) => write!(f, "{s}"),
            Expression::Add(a, b) => {
                write!(f, r"\left(")?;
                a.latex(f)?;
                write!(f, r" + ")?;
                b.latex(f)?;
                write!(f, r"\right)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_display() {
        let expr = Expression::Add(
            Box::new(Expression::Symbol("x".to_string())),
            Box::new(Expression::Symbol("y".to_string())),
        );
        assert_eq!(expr.to_string(), "(x + y)",);
    }
}
