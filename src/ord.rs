//! Ordering relations

use core::cmp::Ordering;
use core::cmp::Ordering::*;

pub trait TotalOrderRelation<A> {
    fn cmp(&A, &A) -> Ordering;

    fn less(x: &A, y: &A) -> bool { Self::cmp(x, y) == Less }

    fn less_or_equal(x: &A, y: &A) -> bool { Self::cmp(x, y) != Greater }
}

pub enum Ord {}
pub struct Dual<Rel>(Rel);

impl<A: ::core::cmp::Ord> TotalOrderRelation<A> for Ord {
    fn cmp(x: &A, y: &A) -> Ordering { ::core::cmp::Ord::cmp(x, y) }
}

impl<A, Rel: TotalOrderRelation<A>> TotalOrderRelation<A> for Dual<Rel> {
    fn cmp(x: &A, y: &A) -> Ordering { Rel::cmp(y, x) }
}
