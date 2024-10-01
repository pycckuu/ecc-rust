use num_bigint::BigUint;
use num_traits::identities::Zero;

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

    pub fn sqrt(c: &BigUint, p: &BigUint) -> Option<BigUint> {
        if c.is_zero() || c == &BigUint::from(1u32) {
            return Some(c.clone());
        }

        if p % 4u32 == 3u32.into() {
            let exp = (p + 1u32) / 4u32;
            let root = c.modpow(&exp, p);
            if root.modpow(&BigUint::from(2u32), p) == *c {
                let root_clone = root.clone();
                return Some(root.min(p - root_clone));
            }
        } else {
            // Tonelli-Shanks algorithm
            let mut q = p - 1u32;
            let mut s = 0u32;
            while &q % 2u32 == 0u32.into() {
                q /= 2u32;
                s += 1;
            }

            let mut z = BigUint::from(2u32);
            while z.modpow(&((p - 1u32) / 2u32), p) != (p - 1u32).into() {
                z += 1u32;
            }

            let mut m = s;
            let mut c_temp = z.modpow(&q, p);
            let mut t = c.modpow(&q, p);
            let mut r = c.modpow(&((q + 1u32) / 2u32), p);

            while t != 1u32.into() {
                let mut i = 0u32;
                let mut t2 = t.clone();
                while t2 != 1u32.into() && i < m {
                    t2 = t2.modpow(&BigUint::from(2u32), p);
                    i += 1;
                }
                if i == m {
                    return None;
                }
                let b = c_temp.modpow(&BigUint::from(1u32 << (m - i - 1)), p);
                m = i;
                c_temp = b.modpow(&BigUint::from(2u32), p);
                t = (t * &c_temp) % p;
                r = (r * &b) % p;
            }

            if r.modpow(&BigUint::from(2u32), p) == *c {
                let r_clone = r.clone();
                return Some(r.min(p - r_clone));
            }
        }

        None
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

  #[test]
  fn test_sqrt() {
      let p = BigUint::from(11u32);

      // Test square numbers
      for i in 0..11 {
          let c = BigUint::from((i * i % 11) as u32);
          let root = FiniteField::sqrt(&c, &p);
          assert!(root.is_some());
          let r = root.unwrap();
          assert!(r <= p.clone() / 2u32);
          assert_eq!(r.modpow(&BigUint::from(2u32), &p), c);
      }

      // Test non-square number
      let non_square = BigUint::from(2u32);
      assert!(FiniteField::sqrt(&non_square, &p).is_none());

      // Test with a larger prime
      let large_p = BigUint::from(1009u32);
      let c = BigUint::from(25u32); // 25 is a quadratic residue modulo 1009
      let root = FiniteField::sqrt(&c, &large_p).unwrap();
      assert_eq!(root.modpow(&BigUint::from(2u32), &large_p), c);

      // Test edge cases
      assert_eq!(FiniteField::sqrt(&BigUint::from(0u32), &p), Some(BigUint::from(0u32)));
      assert_eq!(FiniteField::sqrt(&BigUint::from(1u32), &p), Some(BigUint::from(1u32)));
  }

  #[test]
  fn test_sqrt_edge_cases() {
      // Test with p mod 4 == 3
      let p = BigUint::from(7u32); // 7 mod 4 == 3
      assert_eq!(FiniteField::sqrt(&BigUint::from(2u32), &p), Some(BigUint::from(3u32)));
      assert_eq!(FiniteField::sqrt(&BigUint::from(0u32), &p), Some(BigUint::from(0u32)));
      assert_eq!(FiniteField::sqrt(&BigUint::from(1u32), &p), Some(BigUint::from(1u32)));

      // Test with p mod 4 == 1
      let p = BigUint::from(13u32); // 13 mod 4 == 1
      assert_eq!(FiniteField::sqrt(&BigUint::from(3u32), &p), Some(BigUint::from(4u32)));
      assert_eq!(FiniteField::sqrt(&BigUint::from(0u32), &p), Some(BigUint::from(0u32)));
      assert_eq!(FiniteField::sqrt(&BigUint::from(1u32), &p), Some(BigUint::from(1u32)));
  }
}