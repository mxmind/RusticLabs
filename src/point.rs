use length::Length;
use size::Size2D;
use num::Zero;

use num_lib::NumCast;
use num_lib::traits;
use std::fmt as f;
use std::ops::{Add, Neg, Mul, Sub, Div};

// .
// . Two dimensions point.
// .

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

/// Creates an instance of Point2D.
impl<T> Point2D<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D {
            x: x,
            y: y
        }
    }
}

/// Create the singleton instance of Point2D that has the zero values.
impl<T: Zero> Point2D<T> {
    #[inline]
    pub fn zero() -> Point2D<T> {
        Point2D {
            x: Zero::zero(),
            y: Zero::zero(),
        }
    }
}

/// Formats output for debugging purpose.
impl<T: f::Debug> f::Debug for Point2D<T> {
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({:?}, {:?})", self.x, self.y)
    }
}

/// Formats output.
impl<T: f::Display> f::Display for Point2D<T> {
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy> Point2D<T> {
    #[inline]
    pub fn dot(&self, other: Point2D<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(&self, other: Point2D<T>) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl<T: Clone + Add<T, Output = T>> Add for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Clone + Add<T, Output = T>> Add<Size2D<T>> for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, size: Size2D<T>) -> Point2D<T> {
        Point2D::new(self.x + size.width, self.y + size.height)
    }
}

impl<T: Copy + Add<T, Output = T>> Point2D<T> {
    pub fn add_size(&self, size: &Size2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x + size.width,
            y: self.y + size.height,
        }
    }
}

impl<T: Clone + Sub<T, Output = T>> Sub for Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, othert: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x - othert.x, self.y - othert.y)
    }
}

impl<T: Clone + Neg<Output = T>> Neg for Point2D<T> {
    type Output = Point2D<T>;

    #[inline]
    fn neg(self) -> Point2D<T> {
        Point2D::new(-self.x, -self.y)
    }
}

impl<T: traits::Float> Point2D<T> {
    pub fn min(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl<S: Copy, T: Mul<S, Output = R>, R: Clone> Mul<S> for Point2D<T> {
    type Output = Point2D<R>;

    #[inline]
    fn mul(self, scale: S) -> Point2D<R> {
        Point2D::new(self.x * scale, self.y * scale)
    }
}

impl<S: Copy, T: Div<S, Output = R>, R: Clone> Div<S> for Point2D<T> {
    type Output = Point2D<R>;

    #[inline]
    fn div(self, scale: S) -> Point2D<R> {
        Point2D::new(self.x / scale, self.y / scale)
    }
}

// @section:begin Convenient aliases for Point2D with typed units.

pub type TypedPoint2D<U, T> = Point2D<Length<U, T>>;

impl<U, T: Clone> TypedPoint2D<U, T> {
    pub fn typed(x: T, y: T) -> TypedPoint2D<U, T> {
        Point2D::new(Length::new(x), Length::new(y))
    }

    pub fn to_untyped(&self) -> Point2D<T> {
        Point2D::new(self.x.get(), self.x.get())
    }

    pub fn from_untyped(point: &Point2D<T>) -> TypedPoint2D<U, T> {
        Point2D::new(Length::new(point.x.clone()), Length::new(point.y.clone()))
    }
}

// @section:end

impl<U, T: Clone + NumCast> Point2D<Length<U, T>> {
    pub fn cast<R: Clone + NumCast>(&self) -> Option<Point2D<Length<U, R>>> {
        match (self.x.cast(), self.y.cast()) {
            (Some(x), Some(y)) => Some(Point2D::new(x, y)),
            _ => None,
        }
    }
}

impl<U, T: Clone + NumCast> Point2D<Length<U, T>> {
    pub fn as_f32(&self) -> Point2D<Length<U, f32>> {
        self.cast().unwrap()
    }

    pub fn as_usize(&self) -> Point2D<Length<U, usize>> {
        self.cast().unwrap()
    }
}

// .
// . Three dimensions point.
// .
#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Creates an instance of Point2D.
impl<T> Point3D<T> {
    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D {
            x: x,
            y: y,
            z: z
        }
    }
}

/// Create the singleton instance of Point2D that has the zero values.
impl<T: Zero> Point3D<T> {
    #[inline]
    pub fn zero() -> Point3D<T> {
        Point3D {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
        }
    }
}

/// Formats output for debugging purpose.
impl<T: f::Debug> f::Debug for Point3D<T> {
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

/// Formats output.
impl<T: f::Display> f::Display for Point3D<T> {
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({}, {}, {})", self.x, self.y, self.z)
    }
}

 #[cfg_attr(rustfmt, rustfmt_skip)]
impl <T:Copy + Mul<T, Output=T> + Add<T, Output=T> + Sub<T, Output=T>> Point3D<T> {
    #[inline]
    pub fn dot(self, other: Point3D<T>) -> T {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Point3D<T>) -> T {
        self.x * other.x -
        self.y * other.y -
        self.z * other.z
    }
}

impl<T: Clone + Add<T, Output = T>> Add for Point3D<T> {
    type Output = Point3D<T>;

     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn add(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl<T: Clone + Sub<T, Output = T>> Sub for Point3D<T> {
    type Output = Point3D<T>;

     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn sub(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

impl<T: Clone + Neg<Output = T>> Neg for Point3D<T> {
    type Output = Point3D<T>;

    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn neg(self) -> Point3D<T> {
        Point3D::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl<T: traits::Float> Point3D<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn min(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z)
        )
    }

     #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn max(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z)
        )
    }
}

// .
// . Four dimensions point.
// .
#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Point4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub t: T,
}

impl<T> Point4D<T> {
    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(x: T, y: T, z: T, t: T) -> Point4D<T> {
        Point4D {
            x: x,
            y: y,
            z: z,
            t: t
        }
    }
}

impl<T: Zero> Point4D<T> {
    #[inline]
    pub fn zero() -> Point4D<T> {
        Point4D {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
            t: Zero::zero(),
        }
    }
}

/// Formats output for debugging purpose.
impl<T: f::Debug> f::Debug for Point4D<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({:?}, {:?}, {:?}, {:?})", self.x, self.y, self.z, self.t)
    }
}

/// Formats output.
impl<T: f::Display> f::Display for Point4D<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn fmt(&self, formatter: &mut f::Formatter) -> f::Result {
        write!(formatter, "({}, {}, {}, {})", self.x, self.y, self.z, self.t)
    }
}

impl<T: Clone + Add<T, Output = T>> Add for Point4D<T> {
    type Output = Point4D<T>;

    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn add(self, other: Point4D<T>) -> Point4D<T> {
        Point4D::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.t + other.t
        )
    }
}

impl<T: Clone + Sub<T, Output = T>> Sub for Point4D<T> {
    type Output = Point4D<T>;

    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn sub(self, other:Point4D<T>) -> Point4D<T> {
        Point4D::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.t - other.t
        )
    }
}

impl<T: Clone + Neg<Output = T>> Neg for Point4D<T> {
    type Output = Point4D<T>;

    #[inline]
     #[cfg_attr(rustfmt, rustfmt_skip)]
    fn neg(self) -> Point4D<T> {
        Point4D::new(
            -self.x,
            -self.y,
            -self.z,
            -self.t
        )
    }
}

impl<T: traits::Float> Point4D<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn min(self, other: Point4D<T>) -> Point4D<T> {
        Point4D::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.t.min(other.t)
        )
    }

     #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn max(self, other:Point4D<T>) -> Point4D<T> {
        Point4D::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.t.max(other.t)
        )
    }
}
