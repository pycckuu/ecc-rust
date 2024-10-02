use log::debug;
use num_bigint::BigUint;
use num_traits::{identities::Zero, One};

pub struct FiniteField;

impl FiniteField {
    pub fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        (a + b) % p
    }

    pub fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        if a >= b {
            (a - b) % p
        } else {
            (p - (b - a) % p) % p
        }
    }

    pub fn mul(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        (a * b) % p
    }

    pub fn div(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        Self::mul(a, &Self::inv_mul(b, p), p)
    }

    pub fn exp(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        a.modpow(b, p)
    }

    pub fn inv_mul(a: &BigUint, p: &BigUint) -> BigUint {
        let (mut t, mut r) = ((BigUint::zero(), BigUint::one()), (p.clone(), a.clone()));

        while !r.1.is_zero() {
            let q = &r.0 / &r.1;
            r = (r.1.clone(), &r.0 - &q * &r.1);
            t = (t.1.clone(), Self::sub(&t.0, &(&q * &t.1), p));
        }

        if r.0 > BigUint::one() {
            panic!("Multiplicative inverse does not exist");
        }

        t.0 % p
    }

    pub fn inv_add(a: &BigUint, p: &BigUint) -> BigUint {
        if a.is_zero() {
            BigUint::zero()
        } else {
            p - a % p
        }
    }

    pub fn sqrt(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        if a.is_zero() || a.is_one() {
            return Some(a.clone());
        }

        if p % 4u32 == BigUint::from(3u32) {
            Self::sqrt_for_p_mod_4_eq_3(a, p)
        } else {
            Self::sqrt_tonelli_shanks(a, p)
        }
    }

    fn sqrt_for_p_mod_4_eq_3(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        let exp = (p + BigUint::one()) / 4u32;
        let root = a.modpow(&exp, p);
        let root_squared = Self::mul(&root, &root, p);
        debug!(
            "p ≡ 3 (mod 4): a = {}, root = {}, root^2 = {}",
            a, root, root_squared
        );

        if root_squared == *a {
            Some(if root <= p / 2u32 { root } else { p - &root })
        } else {
            None
        }
    }

    fn sqrt_tonelli_shanks(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        let (q, s) = Self::factor_p_minus_1(p);

        if s == 1 {
            return Self::sqrt_for_s_eq_1(a, p);
        }

        let z = Self::find_quadratic_non_residue(p);
        let mut c = z.modpow(&q, p);
        let mut r = a.modpow(&((q.clone() + BigUint::one()) / 2u32), p);
        let mut t = a.modpow(&q, p);
        let mut m = s;

        while t != BigUint::one() {
            let i = Self::find_least_i(&t, &m, p);
            if i == m {
                return None;
            }
            let b = c.modpow(&BigUint::from(2u32).pow(m - i - 1), p);
            r = Self::mul(&r, &b, p);
            c = Self::mul(&b, &b, p);
            t = Self::mul(&t, &c, p);
            m = i;
        }

        Some(r)
    }

    fn factor_p_minus_1(p: &BigUint) -> (BigUint, u32) {
        let mut q = p - BigUint::one();
        let mut s = 0u32;
        while &q % 2u32 == BigUint::zero() {
            s += 1;
            q /= 2u32;
        }
        (q, s)
    }

    fn sqrt_for_s_eq_1(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        let r = a.modpow(&((p + BigUint::one()) / 4u32), p);
        if Self::mul(&r, &r, p) == *a {
            Some(r)
        } else {
            None
        }
    }

    fn find_quadratic_non_residue(p: &BigUint) -> BigUint {
        let mut z = BigUint::from(2u32);
        while z.modpow(&((p - BigUint::one()) / 2u32), p) != p - BigUint::one() {
            z += 1u32;
        }
        z
    }

    fn find_least_i(t: &BigUint, m: &u32, p: &BigUint) -> u32 {
        let mut i = 0u32;
        let mut zz = t.clone();
        while zz != BigUint::one() && i < *m {
            zz = Self::mul(&zz, &zz, p);
            i += 1;
        }
        i
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use env_logger;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

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
            assert_eq!(FiniteField::mul(&r, &r, &p), c);
        }

        // Test non-square number
        let non_square = BigUint::from(2u32);
        assert!(FiniteField::sqrt(&non_square, &p).is_none());

        // Test with a larger prime
        let large_p = BigUint::from(1009u32);
        let c = BigUint::from(25u32); // 25 is a quadratic residue modulo 1009
        let root = FiniteField::sqrt(&c, &large_p).unwrap();
        assert_eq!(FiniteField::mul(&root, &root, &large_p), c);

        // Test edge cases
        assert_eq!(
            FiniteField::sqrt(&BigUint::zero(), &p),
            Some(BigUint::zero())
        );
        assert_eq!(FiniteField::sqrt(&BigUint::one(), &p), Some(BigUint::one()));
    }

    #[test]
    fn test_sqrt_edge_cases() {
        init();

        // Test with p mod 4 == 3
        let p = BigUint::from(7u32); // 7 mod 4 == 3
        let sqrt_2 = FiniteField::sqrt(&BigUint::from(2u32), &p);
        debug!("Square root of 2 mod 7: {:?}", sqrt_2);
        assert_eq!(
            sqrt_2,
            Some(BigUint::from(3u32)),
            "Square root of 2 mod 7 should be 3"
        );
        assert_eq!(
            FiniteField::sqrt(&BigUint::zero(), &p),
            Some(BigUint::zero())
        );
        assert_eq!(FiniteField::sqrt(&BigUint::one(), &p), Some(BigUint::one()));

        // Test with p mod 4 == 1
        let p = BigUint::from(13u32); // 13 mod 4 == 1
        let sqrt_3 = FiniteField::sqrt(&BigUint::from(3u32), &p).unwrap();
        debug!("Square root of 3 mod 13: {}", sqrt_3);
        assert!(
            sqrt_3 == BigUint::from(4u32) || sqrt_3 == BigUint::from(9u32),
            "Square root of 3 mod 13 should be either 4 or 9"
        );
        assert_eq!(
            FiniteField::sqrt(&BigUint::zero(), &p),
            Some(BigUint::zero())
        );
        assert_eq!(FiniteField::sqrt(&BigUint::one(), &p), Some(BigUint::one()));
    }

    #[test]
    fn test_inv_add() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(51u32);
        let c_inv = FiniteField::inv_add(&c, &p);
        assert_eq!(c_inv, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&c_inv, &c, &p), BigUint::zero());
    }
}
