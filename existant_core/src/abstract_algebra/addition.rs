use crate::{Addition, ClosedUnder, Division, Groupoid, Multiplication, Subtraction};

impl<T: core::ops::Add<Output = T> + ClosedUnder<Addition> + Copy> Groupoid<Addition> for T {
    fn op(&self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}
impl<T: core::ops::Sub<Output = T> + ClosedUnder<Subtraction> + Copy> Groupoid<Subtraction> for T {
    fn op(&self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}
impl<T: core::ops::Mul<Output = T> + ClosedUnder<Multiplication> + Copy> Groupoid<Multiplication> for T {
    fn op(&self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}
impl<T: core::ops::Div<Output = T> + ClosedUnder<Division> + Copy> Groupoid<Division> for T {
    fn op(&self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}