extern crate matrix;

#[cfg(test)]
mod tests {

    use matrix::length::Length;
    use matrix::size::Size2D;
    use matrix::size::TypedSize2D;

    #[derive(Debug, Copy, Clone)]
    enum Mm {}

    #[test]
    fn test_size() {
        let size = Size2D::new(3, 5);

        // do test;
        assert_eq!(15, size.area());
    }

    #[test]
    fn test_scale_size() {
        let size = Size2D::new(3, 5);

        // do test;
        let scaled_size = size * 10;
        assert_eq!(30, scaled_size.width);
        assert_eq!(50, scaled_size.height);
        assert_eq!(1500, scaled_size.area());
    }

    #[test]
    fn test_descale_size() {
        let size = Size2D::new(30, 50);

        // do test;
        let scaled_size = size / 10;
        assert_eq!(3, scaled_size.width);
        assert_eq!(5, scaled_size.height);
        assert_eq!(15, scaled_size.area());
    }

    #[test]
    fn test_scale_size_f32() {
        let size = Size2D::new(3.0, 5.0);

        // do test;
        let scaled_size = size * 10.0;
        assert_eq!(30.0, scaled_size.width);
        assert_eq!(50.0, scaled_size.height);
        assert_eq!(1500.0, scaled_size.area());
    }

    #[test]
    fn test_descale_size_f32() {
        let size = Size2D::new(30.0, 50.0);

        // do test;
        let scaled_size = size / 10.0;
        assert_eq!(3.0, scaled_size.width);
        assert_eq!(5.0, scaled_size.height);
        assert_eq!(15.0, scaled_size.area());
    }

    #[test]
    fn test_cast_size_to_f32() {
        let width: Length<Mm, f32> = Length::new(3.0);
        let height: Length<Mm, f32> = Length::new(5.0);

        let casted_size = Size2D::new(width, height);
        assert_eq!(3.0, casted_size.width.get());
        assert_eq!(5.0, casted_size.height.get());
        assert_eq!(15.0, casted_size.untyped().area());

        let typed_size: TypedSize2D<Mm, f32> = TypedSize2D::typed(3.0, 5.0);
        assert!(casted_size == typed_size);

        // let size = Size2D::new(3.0, 5.0);
        // let untyped_size: typed_size.untyped();
        // assert!(size == typed_size);
    }
}
