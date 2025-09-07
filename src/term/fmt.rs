use super::Term;
use crate::fmt::Latex;
use core::fmt;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Undefined => write!(f, "undefined"),
            Term::Constant(x) => write!(f, "{x}"),
            Term::Variable(x) => write!(f, "{x}"),
            Term::Neg(x) => write!(f, "-({x})"),
            Term::Sum(xs) => {
                write!(f, "(")?;
                for (i, x) in xs.iter().enumerate() {
                    write!(f, "{x}")?;
                    if i < xs.len() - 1 {
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
            Term::Undefined => write!(f, r"\text{{undefined}}"),
            Term::Constant(x) => write!(f, "{x}"),
            Term::Variable(x) => write!(f, "{x}"),
            Term::Neg(x) => {
                write!(f, r"-\left(")?;
                x.latex(f)?;
                write!(f, r"\right)")
            }
            Term::Sum(xs) => {
                write!(f, r"\left(")?;
                for (i, x) in xs.iter().enumerate() {
                    x.latex(f)?;
                    if i < xs.len() - 1 {
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
    use crate::fmt::ToLatex;
    use crate::symbol::Symbol;
    use std::collections::VecDeque;

    #[test]
    fn display_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let n = Term::Constant(123.into());
        let neg_x = Term::Neg(Box::new(x.clone()));
        let sum = Term::Sum(VecDeque::from([x.clone(), n.clone()]));
        let undefined = Term::Undefined;

        assert_eq!(x.to_string(), "x");
        assert_eq!(n.to_string(), "123");
        assert_eq!(neg_x.to_string(), "-(x)");
        assert_eq!(sum.to_string(), "(x + 123)");
        assert_eq!(undefined.to_string(), "undefined");
    }

    #[test]
    fn latex_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let n = Term::Constant(123.into());
        let neg_x = Term::Neg(Box::new(x.clone()));
        let sum = Term::Sum(VecDeque::from([x.clone(), n.clone()]));
        let undefined = Term::Undefined;
        let neg_neg_x = Term::Neg(Box::new(neg_x.clone()));

        assert_eq!(x.to_latex(), "x");
        assert_eq!(n.to_latex(), "123");
        assert_eq!(neg_x.to_latex(), r"-\left(x\right)");
        assert_eq!(sum.to_latex(), r"\left(x + 123\right)");
        assert_eq!(undefined.to_latex(), r"\text{undefined}");
        assert_eq!(neg_neg_x.to_latex(), r"-\left(-\left(x\right)\right)");
    }
}
