use num_bigint::BigUint;
use crate::ec::EllipticCurve;
use crate::point::Point;

/// Returns the base point (generator) for the secp256k1 curve
pub fn base_point() -> Point {
    let x = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let y = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
    Point::Coordinates(x, y)
}

/// Returns the secp256k1 curve parameters
pub fn secp256k1() -> EllipticCurve {
    let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
    let a = BigUint::from(0u32);
    let b = BigUint::from(7u32);

    EllipticCurve { a, b, p }
}

/// Returns the order of the secp256k1 curve
pub fn curve_order() -> BigUint {
    BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secp256k1_params() {
        let curve = secp256k1();
        let g = base_point();
        let n = curve_order();

        // Verify that the base point is on the curve
        assert!(curve.is_on_curve(&g), "Base point is not on the curve");

        // Verify that n * G = O (point at infinity)
        let result = curve.mul(&g, &n);
        assert_eq!(result, Point::Identity, "n * G did not result in the point at infinity");
    }
}