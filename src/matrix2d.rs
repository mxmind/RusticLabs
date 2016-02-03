use num::{One, Zero};
use point::Point2D;
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

#[cfg_attr(rustfmt, rustfmt_skip)]
impl <T: Copy + Copy + PartialOrd +
         Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> +
         One + Zero> Matrix2D<T> {

    pub fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Matrix2D<T> {
        Matrix2D {
            m11 : m11,
            m12 : m12,
            m21 : m21,
            m22 : m22,
            m31 : m31,
            m32 : m32,
        }
    }

    pub fn mul(&self, m: &Matrix2D<T>) -> Matrix2D<T> {
        Matrix2D::new(
            m.m11*self.m11 + m.m12*self.m21,
            m.m11*self.m12 + m.m12*self.m22,
            m.m21*self.m11 + m.m22*self.m21,
            m.m21*self.m12 + m.m22*self.m22,
            m.m31*self.m11 + m.m32*self.m21 + self.m31,
            m.m31*self.m12 + m.m32*self.m22 + self.m32
        )
    }

    pub fn translate(&self, x: T, y:T) -> Matrix2D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let matrix = Matrix2D::new(_1.clone(), _0.clone(), _0.clone(), _1.clone(), x, y);

        return self.mul(&matrix);
    }

    pub fn scale(&self, x: T, y: T) -> Matrix2D<T> {
        Matrix2D::new (
            self.m11 * x,
            self.m12.clone(),
            self.m21.clone(),
            self.m22 * y,
            self.m31.clone(),
            self.m32.clone(),
        )
    }

    pub fn indentity() -> Matrix2D<T> {
        let (_0, _1):(T, T) = (Zero::zero(), One::one());

        return Matrix2D::new(
            _1.clone(),
            _0.clone(),
            _0.clone(),
            _1.clone(),
            _0.clone(),
            _0.clone(),
        );
    }

    pub fn to_array(&self) -> [T; 6] {
        [
            self.m11.clone(),
            self.m12.clone(),
            self.m21.clone(),
            self.m22.clone(),
            self.m31.clone(),
            self.m32.clone()
        ]
    }

    pub fn transform_point(&self, point : &Point2D<T>) -> Point2D<T> {
        Point2D::new(
            point.x * self.m11 + point.y * self.m21 + self.m31,
            point.x + self.m12 + point.y * self.m22 + self.m32
        )
    }
}
