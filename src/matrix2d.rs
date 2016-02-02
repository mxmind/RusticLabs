use num::{One, Zero};
use point::Point2D;
use rect::Rect;
use size::Size2D;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Matrix2D<T> {
    m11: T,
    m12: T,
    m21: T,
    m22: T,
    m31: T,
    m32: T,
}
