use length::Length;
use num::Zero;

use num_lib::NumCast;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Mul, Div};

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

/// Implements type-safe formatting for debug operations.
impl<T: Debug> Debug for Size2D<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}x{:?}", self.width, self.height)
    }
}

/// Implements console-friendly format printing.
impl<T: Display> Display for Size2D<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "({}x{})", self.width, self.height)
    }
}

/// Create an instance of Size2D.
impl<T: Clone> Size2D<T> {
    pub fn new(width: T, height: T) -> Size2D<T> {
        Size2D {
            width: width,
            height: height,
        }
    }
}

/// Create a zero singleton instance of Size2D.
impl<T: Zero> Size2D<T> {
    pub fn zero() -> Size2D<T> {
        Size2D {
            width: Zero::zero(),
            height: Zero::zero(),
        }
    }
}

impl<T: Zero> Zero for Size2D<T> {
    fn zero() -> Size2D<T> {
        Size2D {
            width: Zero::zero(),
            height: Zero::zero(),
        }
    }
}


/// Calculates the area of size.
impl<T: Copy + Clone + Mul<T, Output = U>, U> Size2D<T> {
    pub fn area(&self) -> U {
        self.width * self.height
    }
}

/// Scales area of size.
impl<S: Copy, T: Mul<S, Output = R>, R: Clone> Mul<S> for Size2D<T> {
    type Output = Size2D<R>;

    #[inline]
    fn mul(self, scale: S) -> Size2D<R> {
        Size2D::new(self.width * scale, self.height * scale)
    }
}

/// Descales area of size.
impl<S: Copy, T: Div<S, Output = R>, R: Clone> Div<S> for Size2D<T> {
    type Output = Size2D<R>;

    #[inline]
    fn div(self, scale: S) -> Size2D<R> {
        Size2D::new(self.width / scale, self.height / scale)
    }
}
// @section:begin
// Type-safe operations.

// Convenient aliases for Size2D with typed units
pub type TypedSize2D<U, T> = Size2D<Length<U, T>>;

impl<U, T: Clone> Size2D<Length<U, T>> {
    /// Add measurement unit to type numeric values.
    pub fn typed(width: T, height: T) -> TypedSize2D<U, T> {
        Size2D::new(Length::new(width), Length::new(height))
    }

    /// Drop  measurement unit and preserve only numeric values.
    pub fn untyped(&self) -> Size2D<T> {
        Size2D::new(self.width.get(), self.height.get())
    }

    /// Creates the typed size from untyped one.
    pub fn from_untyped(other: &Size2D<T>) -> TypedSize2D<U, T> {
        Size2D::new(Length::new(other.width.clone()),
                    Length::new(other.height.clone()))
    }
}

// @section:end

impl<U, T: NumCast + Clone> Size2D<Length<U, T>> {
    pub fn cast<R: NumCast + Clone>(&self) -> Option<Size2D<Length<U, R>>> {
        match (self.width.cast(), self.height.cast()) {
            (Some(width), Some(height)) => Some(Size2D::new(width, height)),
            _ => None,
        }
    }
}

/// TODO: replace on macros to create cast operation for most used numeric types.
impl<U, T: NumCast + Clone> Size2D<Length<U, T>> {
    // to f32;
    pub fn as_f32(&self) -> Size2D<Length<U, f32>> {
        self.cast().unwrap()
    }

    // to usize;
    pub fn as_usize(&self) -> Size2D<Length<U, usize>> {
        self.cast().unwrap()
    }
}
