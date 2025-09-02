use crate::fmt::Latex;
use crate::num::Integer;
use core::fmt;
use core::ops::Add;

/// A mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// An integer.
    Integer(Integer),
    /// A symbol, like 'x'.
    Symbol(String), // TODO: Strings for symbols should be restricted.
    /// Binary addition operation.
    Add(Box<Expression>, Box<Expression>),
}

impl Add<Expression> for Expression {
    type Output = Expression;

    fn add(self, other: Expression) -> Expression {
        Expression::Add(Box::new(self), Box::new(other))
    }
}

impl Add<&Expression> for Expression {
    type Output = Expression;

    fn add(self, other: &Expression) -> Expression {
        Expression::Add(Box::new(self), Box::new(other.clone()))
    }
}

impl Add<Expression> for &Expression {
    type Output = Expression;

    fn add(self, other: Expression) -> Expression {
        Expression::Add(Box::new(self.clone()), Box::new(other))
    }
}

impl Add<&Expression> for &Expression {
    type Output = Expression;

    fn add(self, other: &Expression) -> Expression {
        Expression::Add(Box::new(self.clone()), Box::new(other.clone()))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{i}"),
            Expression::Symbol(s) => write!(f, "{s}"),
            Expression::Add(a, b) => write!(f, "({a} + {b})"),
        }
    }
}

impl Latex for Expression {
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{i}"),
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
    fn add_works() {
        let x = Expression::Symbol("x".to_string());
        let y = Expression::Symbol("y".to_string());
        let expected = Expression::Add(Box::new(x.clone()), Box::new(y.clone()));
        assert_eq!(x.clone() + y.clone(), expected);
        assert_eq!(x.clone() + &y, expected);
        assert_eq!(&x + y.clone(), expected);
        assert_eq!(&x + &y, expected);
    }
}
