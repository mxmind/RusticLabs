extern crate matrix;

#[cfg(test)]
mod tests {

    use matrix::scale_factor::ScaleFactor;

    // @section:aliases;

    #[derive(Debug, Copy, Clone)]
    enum Inch {}

    #[derive(Debug, Copy, Clone)]
    enum Cm {}

    #[derive(Debug, Copy, Clone)]
    enum Mm {}

    // @section:end;

    #[test]
    fn test_scale_factor() {
        let mm_per_inch: ScaleFactor<Inch, Mm, f32> = ScaleFactor::new(25.4);
        let cm_per_mm: ScaleFactor<Mm, Cm, f32> = ScaleFactor::new(0.1);

        // do test;
        let cm_per_inch: ScaleFactor<Inch, Cm, f32> = mm_per_inch * cm_per_mm;
        assert_eq!(cm_per_inch, ScaleFactor::new(2.54));
    }

    #[test]
    fn test_inverse_scale_factor() {
        let cm_per_mm: ScaleFactor<Mm, Cm, f32> = ScaleFactor::new(0.1);

        // do test;
        let mm_per_cm: ScaleFactor<Cm, Mm, f32> = cm_per_mm.inv();
        assert_eq!(mm_per_cm.get(), 10.0);
    }

    #[test]
    fn test_scale_factor_ops() {
        let a: ScaleFactor<Inch, Inch, isize> = ScaleFactor::new(2);
        let b: ScaleFactor<Inch, Inch, isize> = ScaleFactor::new(3);

        // do test;
        assert!(a != b);
        assert_eq!(a, a.clone());
        assert_eq!(a.clone() + b.clone(), ScaleFactor::new(5));
        assert_eq!(a - b, ScaleFactor::new(-1));
    }
}
