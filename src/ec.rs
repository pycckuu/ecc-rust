use crate::ff::FiniteField;
use crate::point::Point;
use num_bigint::BigUint;

/// Represents an elliptic curve with the equation y^2 = x^3 + ax + b (mod p)
#[derive(Clone, Debug)]
pub struct EllipticCurve {
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
}

impl EllipticCurve {
    pub fn add(&self, p1: &Point, p2: &Point) -> Point {
        match (p1, p2) {
            (Point::Identity, _) => p2.clone(),
            (_, Point::Identity) => p1.clone(),
            (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                if x1 == x2 {
                    let y2_plus_y1 = FiniteField::add(y2, y1, &self.p);
                    if y2_plus_y1 == BigUint::from(0u32) {
                        return Point::Identity;
                    }
                    if y1 == y2 {
                        return self.double(p1);
                    }
                }
                self.add_distinct(x1, y1, x2, y2)
            }
        }
    }

    fn add_distinct(&self, x1: &BigUint, y1: &BigUint, x2: &BigUint, y2: &BigUint) -> Point {
        let s = self.calculate_slope(x1, y1, x2, y2);
        let x3 = self.calculate_x3(&s, x1, x2);
        let y3 = self.calculate_y3(&s, x1, &x3, y1);
        Point::Coordinates(x3, y3)
    }

    fn calculate_slope(&self, x1: &BigUint, y1: &BigUint, x2: &BigUint, y2: &BigUint) -> BigUint {
        let numerator = FiniteField::sub(y2, y1, &self.p);
        let denominator = FiniteField::sub(x2, x1, &self.p);
        FiniteField::mul(
            &numerator,
            &FiniteField::inv_mul(&denominator, &self.p),
            &self.p,
        )
    }

    fn calculate_x3(&self, s: &BigUint, x1: &BigUint, x2: &BigUint) -> BigUint {
        FiniteField::sub(
            &FiniteField::sub(&s.modpow(&BigUint::from(2u32), &self.p), x1, &self.p),
            x2,
            &self.p,
        )
    }

    fn calculate_y3(&self, s: &BigUint, x1: &BigUint, x3: &BigUint, y1: &BigUint) -> BigUint {
        FiniteField::sub(
            &FiniteField::mul(s, &FiniteField::sub(x1, x3, &self.p), &self.p),
            y1,
            &self.p,
        )
    }

    pub fn double(&self, p: &Point) -> Point {
        match p {
            Point::Identity => Point::Identity,
            Point::Coordinates(x, y) => {
                let s = self.calculate_tangent_slope(x, y);
                let x3 = self.calculate_x3(&s, x, x);
                let y3 = self.calculate_y3(&s, x, &x3, y);
                Point::Coordinates(x3, y3)
            }
        }
    }

    fn calculate_tangent_slope(&self, x: &BigUint, y: &BigUint) -> BigUint {
        let numerator = FiniteField::add(
            &FiniteField::mul(
                &BigUint::from(3u32),
                &x.modpow(&BigUint::from(2u32), &self.p),
                &self.p,
            ),
            &self.a,
            &self.p,
        );
        let denominator = FiniteField::mul(&BigUint::from(2u32), y, &self.p);
        FiniteField::mul(
            &numerator,
            &FiniteField::inv_mul(&denominator, &self.p),
            &self.p,
        )
    }

    pub fn mul(&self, p: &Point, scalar: &BigUint) -> Point {
        let mut result = Point::Identity;
        let mut temp = p.clone();
        let mut n = scalar.clone();

        while n > BigUint::from(0u32) {
            if n.bit(0) {
                result = self.add(&result, &temp);
            }
            temp = self.double(&temp);
            n >>= 1;
        }

        result
    }

    pub fn is_on_curve(&self, a: &Point) -> bool {
        match a {
            Point::Coordinates(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mul(&self.a, x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);
                y2 == FiniteField::add(&x3plusax, &self.b, &self.p)
            }
            Point::Identity => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_curve() -> EllipticCurve {
        EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        }
    }

    mod point_operations {
        use super::*;

        #[test]
        fn test_add() {
            let curve = create_test_curve();
            let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
            let p2 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
            let expected = Point::Coordinates(BigUint::from(10u32), BigUint::from(6u32));

            assert_eq!(curve.add(&p1, &p2), expected);
        }

        #[test]
        fn test_add_with_identity() {
            let curve = create_test_curve();
            let p = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));

            assert_eq!(curve.add(&Point::Identity, &p), p);
            assert_eq!(curve.add(&p, &Point::Identity), p);
            assert_eq!(
                curve.add(&Point::Identity, &Point::Identity),
                Point::Identity
            );
        }

        #[test]
        fn test_add_mirror_points() {
            let curve = create_test_curve();
            let p1 = Point::Coordinates(BigUint::from(5u32), BigUint::from(16u32));
            let p2 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));

            assert_eq!(curve.add(&p1, &p2), Point::Identity);
        }

        #[test]
        fn test_double() {
            let curve = create_test_curve();
            let p = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
            let expected = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));

            assert_eq!(curve.double(&p), expected);
        }
    }

    mod scalar_operations {
        use super::*;

        #[test]
        fn test_scalar_multiplication() {
            let curve = create_test_curve();
            let p = Point::Coordinates(BigUint::from(3u32), BigUint::from(1u32));
            let scalar = BigUint::from(5u32);
            let result = curve.mul(&p, &scalar);

            assert!(matches!(result, Point::Coordinates(_, _)));
            assert!(curve.is_on_curve(&result));
        }
    }

    mod curve_properties {
        use super::*;

        #[test]
        fn test_is_on_curve() {
            let curve = create_test_curve();
            let p = Point::Coordinates(BigUint::from(3u32), BigUint::from(1u32));
            assert!(curve.is_on_curve(&p));

            let not_on_curve = Point::Coordinates(BigUint::from(4u32), BigUint::from(2u32));
            assert!(!curve.is_on_curve(&not_on_curve));
        }
    }
}
