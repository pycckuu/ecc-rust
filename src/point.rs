use num_bigint::BigUint;

/// Represents a point on an elliptic curve
#[derive(PartialEq, Clone, Debug)]
pub enum Point {
    Coordinates(BigUint, BigUint),
    Identity,
}