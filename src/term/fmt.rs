use super::Term;
use crate::fmt::Latex;
use core::fmt;

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

    #[test]
    fn display_works() {
        todo!()
    }

    #[test]
    fn latex_works() {
        todo!()
    }
}
