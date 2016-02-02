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

    #[test]
    pub fn test_min() {
        let p1 = Point2D::new(1.0, 3.0);
        let p2 = Point2D::new(2.0, 2.0);

        assert_eq!(p1.min(p2), Point2D::new(1.0, 2.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point2D::new(1.0, 3.0);
        let p2 = Point2D::new(2.0, 2.0);

        assert_eq!(p1.max(p2), Point2D::new(2.0, 3.0));
    }
}

#[cfg(test)]
mod typedpoint2d_tests {

    use matrix::point::TypedPoint2D;
    use matrix::scale_factor::ScaleFactor;

    #[derive(Debug, Copy, Clone)]
    pub enum Mm {}

    #[derive(Debug, Copy, Clone)]
    pub enum Cm {}

    pub type Point2DMm<T> = TypedPoint2D<Mm, T>;

    pub type Point2DCm<T> = TypedPoint2D<Cm, T>;

    #[test]
    pub fn test_add() {
        let p1 = Point2DMm::typed(1.0, 2.0);
        let p2 = Point2DMm::typed(3.0, 4.0);

        assert_eq!(p1 + p2, Point2DMm::typed(4.0, 6.0));
    }

    #[test]
    pub fn test_scalar_mul() {
        let p1 = Point2DMm::typed(1.0, 2.0);
        let cm_per_mm: ScaleFactor<Mm, Cm, f32> = ScaleFactor::new(0.1);

        assert_eq!(p1 * cm_per_mm, Point2DCm::typed(0.1, 0.2));
    }
}

#[cfg(test)]
mod point3d_tests {

    use matrix::point::Point3D;

    #[test]
    fn test_dot() {
        let p1 = Point3D::new(7.0, 21.0, 32.0);
        let p2 = Point3D::new(43.0, 5.0, 16.0);

        assert_eq!(p1.dot(p2), 918.0);
    }

    #[test]
    pub fn test_cross() {
        let p1 = Point3D::new(4.0, 7.0, 9.0);
        let p2 = Point3D::new(13.0, 8.0, 3.0);

        assert_eq!(p1.cross(p2), Point3D::new(-51.0, 105.0, -59.0));
    }

    #[test]
    pub fn test_min() {
        let p1 = Point3D::new(1.0, 3.0, 5.0);
        let p2 = Point3D::new(2.0, 2.0, -1.0);

        assert_eq!(p1.min(p2), Point3D::new(1.0, 2.0, -1.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point3D::new(1.0, 3.0, 5.0);
        let p2 = Point3D::new(2.0, 2.0, -1.0);

        assert_eq!(p1.max(p2), Point3D::new(2.0, 3.0, 5.0));
    }
}

#[cfg(test)]
mod point4d_tests {
    use matrix::point::Point4D;

    #[test]
    pub fn test_add() {
        let p1 = Point4D::new(7.0, 21.0, 32.0, 1.0);
        let p2 = Point4D::new(43.0, 5.0, 16.0, 2.0);

        assert_eq!(p1 + p2, Point4D::new(50.0, 26.0, 48.0, 3.0));
    }

    #[test]
    pub fn test_sub() {
        let p1 = Point4D::new(7.0, 21.0, 32.0, 1.0);
        let p2 = Point4D::new(43.0, 5.0, 16.0, 2.0);

        assert_eq!(p1 - p2, Point4D::new(-36.0, 16.0, 16.0, -1.0));
    }

    #[test]
    pub fn test_min() {
        let p1 = Point4D::new(1.0, 3.0, 5.0, 7.0);
        let p2 = Point4D::new(2.0, 2.0, -1.0, 10.0);

        assert_eq!(p1.min(p2), Point4D::new(1.0, 2.0, -1.0, 7.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point4D::new(1.0, 3.0, 5.0, 7.0);
        let p2 = Point4D::new(2.0, 2.0, -1.0, 10.0);

        assert_eq!(p1.max(p2), Point4D::new(2.0, 3.0, 5.0, 10.0));
    }
}
