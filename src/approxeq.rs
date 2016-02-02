/// Trait for testing approximate equality
pub trait ApproxEq<Eps> {
    fn approx_epsilon() -> Eps;

    fn approx_eq(&self, other: &Self) -> bool;

    fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool;
}

impl ApproxEq<f32> for f32 {
    #[inline]
    fn approx_epsilon() -> f32 {
        1.0e-6
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f32, approx_epsilon: &f32) -> bool {
        (*self - *other).abs() < *approx_epsilon
    }

    #[inline]
    fn approx_eq(&self, other: &f32) -> bool {
        self.approx_eq_eps(other, &1.0e-6)
    }
}

impl ApproxEq<f64> for f64 {
    #[inline]
    fn approx_epsilon() -> f64 {
        1.0e-6
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f64, approx_epsilon: &f64) -> bool {
        (*self - *other).abs() < *approx_epsilon
    }

    #[inline]
    fn approx_eq(&self, other: &f64) -> bool {
        self.approx_eq_eps(other, &1.0e-6)
    }
}
