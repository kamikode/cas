use super::Term;
use core::mem;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

impl Add<Term> for Term {
    type Output = Term;

    #[inline]
    fn add(mut self, other: Term) -> Term {
        self += other;
        self
    }
}

impl AddAssign<Term> for Term {
    fn add_assign(&mut self, other: Term) {
        *self = match (mem::replace(self, Term::zero()), other) {
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
        };
    }
}

impl Neg for Term {
    type Output = Term;

    #[inline]
    fn neg(self) -> Term {
        Term::Neg(Box::new(self))
    }
}

impl Sub<Term> for Term {
    type Output = Term;

    #[inline]
    fn sub(self, other: Term) -> Term {
        self + (-other)
    }
}

impl SubAssign<Term> for Term {
    #[inline]
    fn sub_assign(&mut self, other: Term) {
        *self += -other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;
    use std::collections::VecDeque;

    #[test]
    fn add_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let y = Term::Variable(Symbol::try_from("y").unwrap());
        let expected = Term::Sum(VecDeque::from([x.clone(), y.clone()]));
        assert_eq!(x + y, expected);
    }

    #[test]
    fn add_assign_works() {
        let mut x1 = Term::Variable(Symbol::try_from("x1").unwrap());
        let mut x2 = Term::Variable(Symbol::try_from("x2").unwrap());
        let x3 = Term::Variable(Symbol::try_from("x3").unwrap());
        let x4 = Term::Variable(Symbol::try_from("x4").unwrap());
        let x5 = Term::Variable(Symbol::try_from("x5").unwrap());
        let x6 = Term::Variable(Symbol::try_from("x6").unwrap());
        let x5_plus_x6 = Term::Sum(vec![x5.clone(), x6.clone()].into());
        let expected = Term::Sum(VecDeque::from([
            x1.clone(),
            x2.clone(),
            x3.clone(),
            x4.clone(),
            x5.clone(),
            x6.clone(),
        ]));
        x2 += x3; // not Sum += not Sum
        x2 += x4; // Sum += not Sum
        x1 += x2; // not Sum += Sum
        x1 += x5_plus_x6; // Sum += Sum
        assert_eq!(x1, expected);
    }

    #[test]
    fn neg_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let expected = Term::Neg(Box::new(x.clone()));
        assert_eq!(-x, expected);
    }

    #[test]
    fn sub_works() {
        let x = Term::Variable(Symbol::try_from("x").unwrap());
        let y = Term::Variable(Symbol::try_from("y").unwrap());
        let expected = Term::Sum(VecDeque::from([x.clone(), Term::Neg(Box::new(y.clone()))]));
        assert_eq!(x - y, expected);
    }

    #[test]
    fn sub_assign_works() {
        let mut x = Term::Variable(Symbol::try_from("x").unwrap());
        let y = Term::Variable(Symbol::try_from("y").unwrap());
        let expected = Term::Sum(VecDeque::from([x.clone(), Term::Neg(Box::new(y.clone()))]));
        x -= y;
        assert_eq!(x, expected);
    }
}
