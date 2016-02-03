#[cfg(feature = "plugins")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Add, Mul, Sub, Div};
use std::marker::PhantomData;

use num_lib::NumCast;

use num::One;

#[derive(Copy, RustcDecodable, RustcEncodable, Debug)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
pub struct ScaleFactor<Src, Dst, T>(pub T, PhantomData<(Src, Dst)>);

#[cfg(feature = "plugins")]
impl<S, D, T> Deserialize for ScaleFactor<S, D, T> where T: Deserialize
{
    fn deserialize(deserializer: &mut D) -> Result<ScaleFactor<S, D, T>, D::Error> {
        Ok(ScaleFactor(try!(Deserialize::deserialize(deserializer)), PhantomData))
    }
}

#[cfg(feature = "plugins")]
impl<S, D, T> Serialize for ScaleFactor<S, D, T> where T: Serialize
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn serialize(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serialize {
        self.0.serialize(serializer)
    }
}

/// Creates an instance of new ScaleFactor.
impl<S, D, T> ScaleFactor<S, D, T> {
    pub fn new(x: T) -> ScaleFactor<S, D, T> {
        ScaleFactor(x, PhantomData)
    }
}

/// Clones existing scale factor literal value.
impl<S, D, T: Clone> ScaleFactor<S, D, T> {
    pub fn get(&self) -> T {
        self.0.clone()
    }
}

/// Clones existing scale factor to new one.
impl<S, D, T: Clone> Clone for ScaleFactor<S, D, T> {
    fn clone(&self) -> ScaleFactor<S, D, T> {
        ScaleFactor::new(Clone::clone(&self.0))
    }
}

/// Implements numeric casting of ScaleFactor
impl<S, D, T: Clone + NumCast> ScaleFactor<S, D, T> {
    pub fn cast<R: Clone + NumCast>(&self) -> Option<ScaleFactor<S, D, R>> {
        NumCast::from(self.get()).map(ScaleFactor::new)
    }
}

/// Overloads "==" equal operation.
impl<S, D, T: Clone + PartialEq> PartialEq for ScaleFactor<S, D, T> {
    fn eq(&self, other: &ScaleFactor<S, D, T>) -> bool {
        self.get().eq(&other.get())
    }
}

/// Implements of inverse operation.
impl<S, D, T: Clone + One + Div<T, Output = T>> ScaleFactor<S, D, T> {
    pub fn inv(&self) -> ScaleFactor<D, S, T> {
        let one: T = One::one();
        ScaleFactor::new(one / self.get())
    }
}

/// Overloads "*" multiply operator.
impl<A, B, C, T: Clone + Mul<T, Output = T>> Mul<ScaleFactor<B, C, T>> for ScaleFactor<A, B, T> {
    type Output = ScaleFactor<A, C, T>;

    #[inline]
    fn mul(self, other: ScaleFactor<B, C, T>) -> ScaleFactor<A, C, T> {
        ScaleFactor::new(self.get() * other.get())
    }
}

/// Overloads "+" add operator.
impl<S, D, T: Clone + Add<T, Output = T>> Add for ScaleFactor<S, D, T> {
    type Output = ScaleFactor<S, D, T>;

    fn add(self, other: ScaleFactor<S, D, T>) -> ScaleFactor<S, D, T> {
        ScaleFactor::new(self.get() + other.get())
    }
}

/// Overloads "-" substract operator.
impl<S, D, T: Clone + Sub<T, Output = T>> Sub for ScaleFactor<S, D, T> {
    type Output = ScaleFactor<S, D, T>;

    fn sub(self, other: ScaleFactor<S, D, T>) -> ScaleFactor<S, D, T> {
        ScaleFactor::new(self.get() - other.get())
    }
}
