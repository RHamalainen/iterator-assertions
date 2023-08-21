use core::ops::Not;

pub trait IteratorAssert {
    #[must_use]
    fn assert(self, assertion: fn(&Self) -> bool) -> Self;
}

impl<I> IteratorAssert for I
where
    I: IntoIterator,
{
    fn assert(self, assertion: fn(&Self) -> bool) -> Self {
        if (assertion)(&self).not() {
            panic!("iterator assertion did not hold");
        }
        self
    }
}
