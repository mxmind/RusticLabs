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
}
