//! Ordering relations

use core::cmp::Ordering;
use core::cmp::Ordering::*;

use eq::*;

pub trait PartialOrderRelation<A: ?Sized>: PartialEquivalenceRelation<A> {
    fn partial_cmp(&self, &A, &A) -> Option<Ordering>;

    #[inline]
    fn less(&self, x: &A, y: &A) -> bool { Some(Less) == self.partial_cmp(x, y) }

    #[inline]
    fn less_or_equal(&self, x: &A, y: &A) -> bool {
        match self.partial_cmp(x, y) {
            Some(Less) => true,
            Some(Equal) => true,
            _ => false,
        }
    }
}

pub trait TotalOrderRelation<A: ?Sized>: PartialOrderRelation<A> + EquivalenceRelation<A> {
    fn cmp(&self, &A, &A) -> Ordering;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dual<Rel>(Rel);

impl<A: ?Sized + PartialOrd> PartialOrderRelation<A> for ::Core {
    fn partial_cmp(&self, x: &A, y: &A) -> Option<Ordering> { PartialOrd::partial_cmp(x, y) }
}

impl<A: ?Sized + Ord> TotalOrderRelation<A> for ::Core {
    fn cmp(&self, x: &A, y: &A) -> Ordering { Ord::cmp(x, y) }
}

impl<A: ?Sized, Rel: PartialEquivalenceRelation<A>> PartialEquivalenceRelation<A> for Dual<Rel> {
    fn equal(&self, x: &A, y: &A) -> bool { self.0.equal(x, y) }
    fn inequal(&self, x: &A, y: &A) -> bool { self.0.inequal(x, y) }
}

impl<A: ?Sized, Rel: EquivalenceRelation<A>> EquivalenceRelation<A> for Dual<Rel> {}

impl<A: ?Sized, Rel: PartialOrderRelation<A>> PartialOrderRelation<A> for Dual<Rel> {
    fn partial_cmp(&self, x: &A, y: &A) -> Option<Ordering> { self.0.partial_cmp(y, x) }
}

impl<A: ?Sized, Rel: TotalOrderRelation<A>> TotalOrderRelation<A> for Dual<Rel> {
    fn cmp(&self, x: &A, y: &A) -> Ordering { self.0.cmp(y, x) }
}
