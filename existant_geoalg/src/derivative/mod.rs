/*
    This section of the math code is dedicated purely to derivatives of functions, as well as
    the Derivative trait. This might be the weirdest section of the math code, specifically
    because most of these will not really be used when coding more advanced projects. These
    are simply here as fun little things I wrote for study.
*/

macro_rules! impl_derivative {
    ($($ty:tt),*) => {
        $(
            impl Derivative for $ty {
                type Output = Self;
                fn derive(&self) -> Self::Output {
                    <Self as Identity<Addition>>::IDENTITY
                }
            }
        )*
    };
}
use existant_core::{Addition, FloatingPoint, Identity};

pub trait Derivative {
    type Output;
    fn derive(&self) -> Self::Output; 
}

impl_derivative!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sin<T: FloatingPoint>(pub T);
impl<T: FloatingPoint> Sin<T> {
    fn call(&self) -> T {
        self.0.sin()
    }
}
impl<T: FloatingPoint> Derivative for Sin<T> {
    type Output = Cos<T>;
    fn derive(&self) -> Self::Output {
        Cos(self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cos<T: FloatingPoint>(pub T);
impl<T: FloatingPoint> Cos<T> {
    fn call(&self) -> T {
        self.0.cos()
    }
}
impl<T: FloatingPoint> Derivative for Cos<T> {
    type Output = Sin<T>;
    fn derive(&self) -> Self::Output {
        Sin(-self.0)
    }
}