use std::{fmt, iter, ops};

pub trait Numeric:
    Copy + fmt::Debug + iter::Sum + ops::AddAssign + ops::DivAssign + ops::Mul<Self, Output = Self>
{
    fn zero() -> Self;
}

macro_rules! numeric_impl {
    ($t:ty, $v:expr) => {
        impl Numeric for $t {
            fn zero() -> Self {
                $v
            }
        }
    };
}

numeric_impl!(f64, 0.0);
numeric_impl!(f32, 0.0);
numeric_impl!(i64, 0);
numeric_impl!(i32, 0);
