use num_bigint::BigUint;

pub struct FiniteField;

impl FiniteField {
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c + d) % p
    }

    pub fn mul(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c * d) % p
    }

    pub fn inv_add(c: &BigUint, p: &BigUint) -> BigUint {
        assert!(c < p, "number must be less than p");
        p - c
    }

    pub fn inv_mul(c: &BigUint, p: &BigUint) -> BigUint {
        assert!(c < p, "number must be less than p");
        c.modpow(&(p - 2u32), p)
    }

    pub fn sub(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let inv_d = FiniteField::inv_add(d, p);
        FiniteField::add(c, &inv_d, p)
    }
}

#[cfg(test)]
mod test {
  use super::*;

  fn setup() -> (BigUint, BigUint, BigUint) {
      (
          BigUint::from(4u32),
          BigUint::from(10u32),
          BigUint::from(11u32),
      )
  }

  #[test]
  fn test_finite_field_operations() {
      let (c, d, p) = setup();
      assert_eq!(FiniteField::add(&c, &d, &p), BigUint::from(3u32));
      assert_eq!(FiniteField::mul(&c, &d, &p), BigUint::from(7u32));
      assert_eq!(FiniteField::inv_add(&c, &p), BigUint::from(7u32));
      assert_eq!(FiniteField::inv_mul(&c, &p), BigUint::from(3u32));
      assert_eq!(
          FiniteField::mul(&c, &FiniteField::inv_mul(&c, &p), &p),
          BigUint::from(1u32)
      );
      assert_eq!(FiniteField::sub(&d, &c, &p), BigUint::from(6u32));
  }

  #[test]
  fn test_inv_add_with_larger_prime() {
      let c = BigUint::from(4u32);
      let p = BigUint::from(51u32);
      let c_inv = FiniteField::inv_add(&c, &p);
      assert_eq!(c_inv, BigUint::from(47u32));
      assert_eq!(FiniteField::add(&c_inv, &c, &p), BigUint::from(0u32));
  }
}