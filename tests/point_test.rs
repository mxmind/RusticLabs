extern crate matrix;

#[cfg(test)]
mod point2d_tests {

    use matrix::point::Point2D;

    #[test]
    pub fn test_scalar_mul() {
        let p1 = Point2D::new(3.0, 5.0);
        let result = p1 * 5.0;

        assert_eq!(result, Point2D::new(15.0, 25.0));
    }

    #[test]
    pub fn test_dot() {
        let p1 = Point2D::new(2.0, 7.0);
        let p2 = Point2D::new(13.0, 11.0);

        assert_eq!(p1.dot(p2), 103.0);
    }

    #[test]
    pub fn test_cross() {
        let p1 = Point2D::new(4.0, 7.0);
        let p2 = Point2D::new(13.0, 8.0);

        assert_eq!(p1.cross(p2), -59.0);
    }
}
