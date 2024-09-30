use crate::ec::WeierstrassCurve;
use crate::point::Point;
use num_bigint::BigUint;

/// Returns the secp256k1 curve parameters
pub fn create_secp256k1_weierstrass() -> WeierstrassCurve {
    let p = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap();
    let a = BigUint::from(0u32);
    let b = BigUint::from(7u32);

    let x = BigUint::parse_bytes(
        b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        16,
    )
    .unwrap();
    let y = BigUint::parse_bytes(
        b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16,
    )
    .unwrap();
    let g = Point::Coordinates(x, y);
    let n = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap();

    WeierstrassCurve::new(a, b, p, n, g)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ec::EllipticCurve; // Add this line to import the trait

    #[test]
    fn test_secp256k1_params() {
        let curve = create_secp256k1_weierstrass();

        // Verify that the base point is on the curve
        assert!(
            curve.is_on_curve(&curve.base_point()),
            "Base point is not on the curve"
        );

        // Verify that n * G = O (point at infinity)
        let result = curve.mul(&curve.base_point(), &curve.order());
        assert_eq!(
            result,
            Point::Identity,
            "n * G did not result in the point at infinity"
        );
    }
}
