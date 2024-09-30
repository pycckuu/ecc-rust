use crate::point::Point;
use num_bigint::BigUint;

pub mod weierstrass;
pub use weierstrass::WeierstrassCurve;

/// Base trait for all elliptic curves
pub trait EllipticCurve {
  fn add(&self, p1: &Point, p2: &Point) -> Point;
  fn double(&self, p: &Point) -> Point;
  fn mul(&self, p: &Point, scalar: &BigUint) -> Point;
  fn is_on_curve(&self, p: &Point) -> bool;
  fn order(&self) -> &BigUint;
  fn base_point(&self) -> &Point;
  fn field_modulus(&self) -> &BigUint;
}