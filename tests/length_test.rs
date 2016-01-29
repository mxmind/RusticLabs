extern crate matrix;

#[cfg(test)]
mod tests {

    use matrix::length::Length;
    use matrix::scale_factor::ScaleFactor;

    // @section:aliases;

    #[derive(Debug, Copy, Clone)]
    enum Inch {}

    #[derive(Debug, Copy, Clone)]
    enum Mm {}

    // @section:end;

    #[test]
    fn test_length_add_sub_operations() {

        let one: Length<Inch, f32> = Length::new(12.0);

        // do test;
        let two = one.clone() + one.clone();
        let zero = one.clone() - one.clone();

        assert_eq!(one.get(), 12.0);
        assert_eq!(two.get(), 24.0);
        assert_eq!(zero.get(), 0.0);
    }

    #[test]
    fn test_length_equals_operation() {

        let one: Length<Inch, f32> = Length::new(12.0);

        // do test;
        let two = one.clone() + one.clone();

        assert!(one == one);
        assert!(two != one);
    }

    #[test]
    fn test_length_comparision_operation() {

        let one: Length<Inch, f32> = Length::new(12.0);

        // do test;
        let two = one.clone() + one.clone();
        let zero = one.clone() - one.clone();

        assert!(zero < one);
        assert!(two > one);

        assert!(zero <= one);
        assert!(two >= one);

        assert!(two <= two);
        assert!(two >= two);

        assert!(!(two > two));
        assert!(!(two < two));
    }

    #[test]
    fn test_mul_scaling() {

        let one: Length<Inch, f32> = Length::new(12.0);
        let mm_per_inch: ScaleFactor<Inch, Mm, f32> = ScaleFactor::new(25.4);

        // do test;
        let one_foot_in_mm: Length<Mm, f32> = one * mm_per_inch;

        assert_eq!(one_foot_in_mm, Length::new(304.8));
        assert_eq!(one_foot_in_mm / one, mm_per_inch);
    }

    #[test]
    fn test_unwrap_to_literal_value() {
        let one: Length<Inch, f32> = Length::new(12.0);

        // do test;
        let int_foot: Length<Inch, isize> = one.cast().unwrap();

        assert_eq!(int_foot.get(), 12);
    }

    #[test]
    fn test_negative_length() {
        let one: Length<Inch, f32> = Length::new(12.0);

        // do test;
        let negative_one = -one;

        assert_eq!(negative_one.get(), -12.0);
    }

    #[test]
    fn test_negative_zero_length() {
        let zero: Length<Inch, f32> = Length::new(0.0);

        // do test;
        let negative_zero = -zero;

        assert_eq!(negative_zero.get(), 0.0);
    }
}
