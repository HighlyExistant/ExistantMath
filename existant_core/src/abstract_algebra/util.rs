use crate::{Addition, Field, Multiplication};

pub trait BasicField: 
    Field<Multiplication, Addition> +
    Sized +
    Copy +
    core::ops::Add<Output = Self> + 
    core::ops::Neg<Output = Self> + 
    core::ops::Sub<Output = Self> +
    core::ops::Mul<Output = Self> +
    core::ops::Div<Output = Self> {
    
}

impl<T> BasicField for T 
    where T: Field<Multiplication, Addition> +
    Sized +
    Copy +
    core::ops::Add<Output = Self> + 
    core::ops::Neg<Output = Self> + 
    core::ops::Sub<Output = Self> +
    core::ops::Mul<Output = Self> +
    core::ops::Div<Output = Self> {
    
}