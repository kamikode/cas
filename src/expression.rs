use crate::fmt::Latex;
use crate::number::Number;
use crate::symbol::Symbol;
use core::fmt;
use core::ops::Add;
use std::collections::VecDeque;

/// A mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A number.
    Number(Number),
    /// A variable specified by a symbol, like 'x'.
    Variable(Symbol),
    /// A sum over subexpressions.
    Sum(VecDeque<Expression>),
}

impl Add<Expression> for Expression {
    type Output = Expression;

    fn add(self, other: Expression) -> Expression {
        match (self, other) {
            (Expression::Sum(mut summands_self), Expression::Sum(mut summands_other)) => {
                summands_self.append(&mut summands_other);
                Expression::Sum(summands_self)
            }
            (Expression::Sum(mut summands), x) => {
                summands.push_back(x);
                Expression::Sum(summands)
            }
            (x, Expression::Sum(mut summands)) => {
                summands.push_front(x);
                Expression::Sum(summands)
            }
            (x, y) => Expression::Sum(vec![x, y].into()),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(i) => write!(f, "{i}"),
            Expression::Variable(s) => write!(f, "{s}"),
            Expression::Sum(es) => {
                write!(f, "(")?;
                for (i, e) in es.iter().enumerate() {
                    write!(f, "{e}")?;
                    if i < es.len() - 1 {
                        write!(f, " + ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl Latex for Expression {
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(i) => write!(f, "{i}"),
            Expression::Variable(s) => write!(f, "{s}"),
            Expression::Sum(es) => {
                write!(f, r"\left(")?;
                for (i, e) in es.iter().enumerate() {
                    e.latex(f)?;
                    if i < es.len() - 1 {
                        write!(f, r" + ")?;
                    }
                }
                write!(f, r"\right)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_op_works() {
        let x = Expression::Variable(Symbol::try_from("x".to_string()).unwrap());
        let y = Expression::Variable(Symbol::try_from("y".to_string()).unwrap());
        let z = Expression::Variable(Symbol::try_from("z".to_string()).unwrap());
        let expected = Expression::Sum(VecDeque::from([x.clone(), y.clone(), z.clone()]));
        assert_eq!(x + y + z, expected);
    }
}
