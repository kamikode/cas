use crate::fmt::Latex;
use crate::number::Number;
use crate::symbol::Symbol;
use core::fmt;
use core::ops::{Add, Neg};
use std::collections::VecDeque;

/// A mathematical term.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A constant value specified by a number.
    Constant(Number),
    /// A variable specified by a symbol, like 'x'.
    Variable(Symbol),
    /// Unary negation operator (additive inverse).
    Neg(Box<Term>),
    /// A sum over subexpressions (n-ary addition).
    Sum(VecDeque<Term>),
    // TODO: Product, Power, etc. Note that Power can also be used for representing
    // division/reciprocals, so this does not need to be an extra case.
}

impl Add<Term> for Term {
    type Output = Term;

    fn add(self, other: Term) -> Term {
        match (self, other) {
            (Term::Sum(mut summands_self), Term::Sum(mut summands_other)) => {
                summands_self.append(&mut summands_other);
                Term::Sum(summands_self)
            }
            (Term::Sum(mut summands), x) => {
                summands.push_back(x);
                Term::Sum(summands)
            }
            (x, Term::Sum(mut summands)) => {
                summands.push_front(x);
                Term::Sum(summands)
            }
            (x, y) => Term::Sum(vec![x, y].into()),
        }
    }
}

impl Neg for Term {
    type Output = Term;

    fn neg(self) -> Term {
        Term::Neg(Box::new(self))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(i) => write!(f, "{i}"),
            Term::Variable(s) => write!(f, "{s}"),
            Term::Neg(e) => write!(f, "-({e})"),
            Term::Sum(es) => {
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

impl Latex for Term {
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(i) => write!(f, "{i}"),
            Term::Variable(s) => write!(f, "{s}"),
            Term::Neg(e) => write!(f, r"-\left({e}\right)"),
            Term::Sum(es) => {
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
        let x = Term::Variable(Symbol::try_from("x".to_string()).unwrap());
        let y = Term::Variable(Symbol::try_from("y".to_string()).unwrap());
        let z = Term::Variable(Symbol::try_from("z".to_string()).unwrap());
        let expected = Term::Sum(VecDeque::from([x.clone(), y.clone(), z.clone()]));
        assert_eq!(x + y + z, expected);
    }

    #[test]
    fn neg_op_works() {
        let x = Term::Variable(Symbol::try_from("x".to_string()).unwrap());
        let expected = Term::Neg(Box::new(x.clone()));
        assert_eq!(-x, expected);
    }
}
