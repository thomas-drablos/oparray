use std::{fmt, iter, ops};

use rand_distr::{Distribution, StandardNormal};

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

pub trait Float:
    Numeric + Sized + sealed::MakeAssociated<StandardNormal, Same: Distribution<Self>>
{
}

mod sealed {
    pub trait MakeAssociated<A: ?Sized>: MakeAssociatedInner<A, Same = A> {}
    impl<A: ?Sized, B: ?Sized> MakeAssociated<A> for B {}

    pub trait MakeAssociatedInner<A: ?Sized> {
        type Same: ?Sized;
    }
    impl<A: ?Sized, B: ?Sized> MakeAssociatedInner<A> for B {
        type Same = A;
    }
}

macro_rules! float_impl {
    ($t:ty) => {
        impl Float for $t {}
    };
}

float_impl!(f64);
float_impl!(f32);
