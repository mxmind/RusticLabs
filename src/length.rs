#[cfg(feature = "plugins")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use num::Zero;
use scale_factor::ScaleFactor;

use num_lib::NumCast;

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, RustcDecodable, RustcEncodable, Debug)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
pub struct Length<U, T>(pub T, PhantomData<U>);

#[cfg(feature = "plugins")]
impl<U, T> Deserialize for Length<U, T> where T: Deserialize
{
    #[rustfmt_skip]
    fn deserialize<D>(deserializer: &mut D) -> Result<Length<U, T>, D::Error> where D: Deserilizer {
        Ok(Length(try!(Deserialize::deserialize(deserializer)), PhantomData))
    }
}

#[cfg(feature = "plugins")]
impl<U, T> Serialize for Length<U, T> where T: Serialize
{
    #[rustfmt_skip]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        self.0.serilize(serializer)
    }
}
/// Creates new instance an Length implementation.
impl<U, T> Length<U, T> {
    pub fn new(x: T) -> Length<U, T> {
        Length(x, PhantomData)
    }
}

/// Gets the immutable value of itself as <T> data type.
impl<U, T: Clone> Length<U, T> {
    pub fn get(&self) -> T {
        self.0.clone()
    }
}

/// Overloads "+" add operator.
impl<U, T: Clone + Add<T, Output = T>> Add for Length<U, T> {
    type Output = Length<U, T>;

    fn add(self, other: Length<U, T>) -> Length<U, T> {
        Length::new(self.get() + other.get())
    }
}

/// Overloads "+=" increment assignment operator.
// impl<U, T: Clone + AddAssign<T>> AddAssign for Length<U, T> {
//     fn add_assign(&mut self, other: Length<U, T>) {
//         self.0 += other.get();
//     }
// }
/// Overloads "-" subtract operator.
impl<U, T: Clone + Sub<T, Output = T>> Sub<Length<U, T>> for Length<U, T> {
    type Output = Length<U, T>;

    fn sub(self, other: Length<U, T>) -> <Self as Sub>::Output {
        Length::new(self.get() - other.get())
    }
}

/// Overloads "-=" decrement assignment operator.
// impl<U, T: Clone + SubAssign<T>> SubAssign for Length<U, T> {
//     fn sub_assign(&mut self, other: Length<U, T>) {
//         self.0 -= other.get()
//     }
// }
/// Overloads "/" divide operator.
impl<S, D, T: Clone + Div<T, Output = T>> Div<Length<S, T>> for Length<D, T> {
    type Output = ScaleFactor<S, D, T>;

    #[inline]
    fn div(self, other: Length<S, T>) -> ScaleFactor<S, D, T> {
        ScaleFactor::new(self.get() / other.get())
    }
}

/// Overloads "*" multiply operator for scaling length by scale factor.
impl<S, D, T: Clone + Mul<T, Output = T>> Mul<ScaleFactor<S, D, T>> for Length<S, T> {
    type Output = Length<D, T>;

    #[inline]
    fn mul(self, scale: ScaleFactor<S, D, T>) -> Length<D, T> {
        Length::new(self.get() * scale.get())
    }
}

/// Overloads "/" divide operator for scaling length by scale factor.
impl<S, D, T: Clone + Div<T, Output = T>> Div<ScaleFactor<S, D, T>> for Length<D, T> {
    type Output = Length<S, T>;

    #[inline]
    fn div(self, scale: ScaleFactor<S, D, T>) -> Length<S, T> {
        Length::new(self.get() / scale.get())
    }
}

/// Overloads "-t" negative operator.
impl<U, T: Clone + Neg<Output = T>> Neg for Length<U, T> {
    type Output = Length<U, T>;

    fn neg(self) -> Length<U, T> {
        Length::new(-self.get())
    }
}

/// Implements numeric casting.
impl<U, S: NumCast + Clone> Length<U, S> {
    pub fn cast<D: NumCast + Clone>(&self) -> Option<Length<U, D>> {
        NumCast::from(self.get()).map(Length::new)
    }
}

/// Implements clone.
impl<U, T: Clone> Clone for Length<U, T> {
    fn clone(&self) -> Length<U, T> {
        Length::new(Clone::clone(&self.0))
    }
}

/// Overloads "==" equals operator.
impl<U, T: Clone + PartialEq> PartialEq for Length<U, T> {
    fn eq(&self, other: &Length<U, T>) -> bool {
        self.get().eq(&other.get())
    }
}

impl<U, T: Clone + PartialOrd> PartialOrd for Length<U, T> {
    fn partial_cmp(&self, other: &Length<U, T>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<U, T: Clone + Eq> Eq for Length<U, T> {
    // nothing special, just for typing purpose;
}

impl<U, T: Clone + Ord> Ord for Length<U, T> {
    fn cmp(&self, other: &Length<U, T>) -> Ordering {
        self.get().cmp(&other.get())
    }
}

// Defines the zero value for given implementation.
impl<U, T: Zero> Zero for Length<U, T> {
    fn zero() -> Length<U, T> {
        Length::new(Zero::zero())
    }
}
