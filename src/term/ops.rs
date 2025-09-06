use super::Term;
use core::ops::{Add, Neg};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;
    use std::collections::VecDeque;

    #[test]
    fn add_op_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let y = Term::Variable(Symbol::try_from("y").unwrap());
        let z = Term::Variable(Symbol::try_from("z").unwrap());
        let w = Term::Variable(Symbol::try_from("w").unwrap());
        let expected = Term::Sum(VecDeque::from([x.clone(), y.clone(), z.clone(), w.clone()]));
        assert_eq!(x + ((y + z) + w), expected);
    }

    #[test]
    fn neg_op_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let expected = Term::Neg(Box::new(x.clone()));
        assert_eq!(-x, expected);
    }
}
