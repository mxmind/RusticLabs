use num_lib as num;

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl<T: num::Zero> Zero for T {
    fn zero() -> T {
        num::Zero::zero()
    }
}

impl<T: num::One> One for T {
    fn one() -> T {
        num::One::one()
    }
}
