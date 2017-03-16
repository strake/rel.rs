pub trait PartialEquivalenceRelation<A: ?Sized> {
    fn equal(&self, &A, &A) -> bool;

    #[inline]
    fn inequal(&self, x: &A, y: &A) -> bool { !self.equal(x, y) }
}

pub trait EquivalenceRelation<A: ?Sized> : PartialEquivalenceRelation<A> {}

impl<A: ?Sized + PartialEq> PartialEquivalenceRelation<A> for ::Core {
    fn equal(&self, x: &A, y: &A) -> bool { x == y }
    fn inequal(&self, x: &A, y: &A) -> bool { x != y }
}

impl<A: ?Sized + Eq> EquivalenceRelation<A> for ::Core {}
