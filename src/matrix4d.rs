use approxeq::ApproxEq;
use point::{Point2D, Point4D};

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Matrix4D {
    m11: f32, m12: f32, m13: f32, m14: f32,
    m21: f32, m22: f32, m23: f32, m24: f32,
    m31: f32, m32: f32, m33: f32, m34: f32,
    m41: f32, m42: f32, m43: f32, m44: f32,
}

impl Matrix4D {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
            m11: f32, m12: f32, m13: f32, m14: f32,
            m21: f32, m22: f32, m23: f32, m24: f32,
            m31: f32, m32: f32, m33: f32, m34: f32,
            m41: f32, m42: f32, m43: f32, m44: f32) -> Matrix4D {

        Matrix4D {
            m11 : m11, m12 : m12, m13 : m13, m14 : m14,
            m21 : m21, m22 : m22, m23 : m23, m24 : m24,
            m31 : m31, m32 : m32, m33 : m33, m34 : m34,
            m41 : m41, m42 : m42, m43 : m43, m44 : m44,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4D {
        let tx = -((left + right) / (left - right));
        let ty = -((bottom + top) / (bottom - top));
        let tz = -((near + far) / (near - far));

        Matrix4D::new(
            2.0 / (right - left), 0.0, 0.0, 0.0,
            0.0, 2.0 / (top - bottom), 0.0, 0.0,
            0.0, 0.0, -2.0 / (far - near), 0.0,
            tx, ty, tz, 1.0
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn indentity() -> Matrix4D {
        Matrix4D::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn mul(&self, m: &Matrix4D) -> Matrix4D {
        Matrix4D::new(
            m.m11*self.m11 + m.m12*self.m21 + m.m13*self.m31 + m.m14*self.m41,
            m.m11*self.m12 + m.m12*self.m22 + m.m13*self.m32 + m.m14*self.m42,
            m.m11*self.m13 + m.m12*self.m23 + m.m13*self.m33 + m.m14*self.m43,
            m.m11*self.m14 + m.m12*self.m24 + m.m13*self.m34 + m.m14*self.m44,
            m.m21*self.m11 + m.m22*self.m21 + m.m23*self.m31 + m.m24*self.m41,
            m.m21*self.m12 + m.m22*self.m22 + m.m23*self.m32 + m.m24*self.m42,
            m.m21*self.m13 + m.m22*self.m23 + m.m23*self.m33 + m.m24*self.m43,
            m.m21*self.m14 + m.m22*self.m24 + m.m23*self.m34 + m.m24*self.m44,
            m.m31*self.m11 + m.m32*self.m21 + m.m33*self.m31 + m.m34*self.m41,
            m.m31*self.m12 + m.m32*self.m22 + m.m33*self.m32 + m.m34*self.m42,
            m.m31*self.m13 + m.m32*self.m23 + m.m33*self.m33 + m.m34*self.m43,
            m.m31*self.m14 + m.m32*self.m24 + m.m33*self.m34 + m.m34*self.m44,
            m.m41*self.m11 + m.m42*self.m21 + m.m43*self.m31 + m.m44*self.m41,
            m.m41*self.m12 + m.m42*self.m22 + m.m43*self.m32 + m.m44*self.m42,
            m.m41*self.m13 + m.m42*self.m23 + m.m43*self.m33 + m.m44*self.m43,
            m.m41*self.m14 + m.m42*self.m24 + m.m43*self.m34 + m.m44*self.m44
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn approx_eq(&self, other: &Matrix4D) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m13.approx_eq(&other.m13) && self.m14.approx_eq(&other.m14) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m23.approx_eq(&other.m23) && self.m24.approx_eq(&other.m24) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32) &&
        self.m33.approx_eq(&other.m33) && self.m34.approx_eq(&other.m34) &&
        self.m41.approx_eq(&other.m41) && self.m42.approx_eq(&other.m42) &&
        self.m43.approx_eq(&other.m43) && self.m44.approx_eq(&other.m44)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn spread(&self, x: f32) -> Matrix4D {
        Matrix4D::new(
            self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
            self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
            self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
            self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x,
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn scale(&self, x: f32, y: f32, z: f32) -> Matrix4D {
        Matrix4D::new(
            self.m11 * x, self.m12, self.m13, self.m14,
            self.m21, self.m22 * y, self.m23, self.m24,
            self.m31, self.m32, self.m33 * z, self.m34,
            self.m41, self.m42, self.m43, self.m44
        )
    }

    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn transform_point2d(&self, point: &Point2D<f32>) -> Point2D<f32> {
        Point2D::new(
            point.x * self.m11 + point.y * self.m21 + self.m41,
            point.x * self.m12 + point.y * self.m22 + self.m42
        )
    }

    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn transform_point4d(&self, point: &Point4D<f32>) -> Point4D<f32> {
        let x = point.x * self.m11 + point.y * self.m12 + point.z * self.m13 + self.m14;
        let y = point.x * self.m21 + point.y * self.m22 + point.z * self.m23 + self.m24;
        let z = point.x * self.m31 + point.y * self.m32 + point.z * self.m33 + self.m34;
        let t = point.x * self.m41 + point.y * self.m42 + point.z * self.m43 + self.m44;

        Point4D::new(x, y, z, t)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn to_array(&self) -> [f32; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        ]
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Matrix4D {
        let matrix = Matrix4D::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, z, 1.0,
        );

        self.mul(&matrix)
    }
}
