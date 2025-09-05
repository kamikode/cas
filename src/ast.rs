use crate::fmt::Latex;
use crate::types::{Integer, Symbol};
use core::fmt;
use core::ops::Add;

/// A mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// An integer.
    Integer(Integer),
    /// A variable specified by a symbol, like 'x'.
    Variable(Symbol),
    /// Binary addition operation.
    Add(Box<Expression>, Box<Expression>),
}

impl Add<Expression> for Expression {
    type Output = Expression;

    fn add(self, other: Expression) -> Expression {
        Expression::Add(Box::new(self), Box::new(other))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{i}"),
            Expression::Variable(s) => write!(f, "{s}"),
            Expression::Add(a, b) => write!(f, "({a} + {b})"),
        }
    }
}

impl Latex for Expression {
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{i}"),
            Expression::Variable(s) => write!(f, "{s}"),
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
    fn add_works() {
        let x = Expression::Variable(Symbol::try_from("x".to_string()).unwrap());
        let y = Expression::Variable(Symbol::try_from("y".to_string()).unwrap());
        let expected = Expression::Add(Box::new(x.clone()), Box::new(y.clone()));
        assert_eq!(x + y, expected);
    }
}
