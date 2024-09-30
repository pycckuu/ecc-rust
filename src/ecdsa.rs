use crate::{EllipticCurve, FiniteField, Point, WeierstrassCurve};
use log::{debug, info, warn};
use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

pub struct ECDSA<T: EllipticCurve> {
    curve: T,
}

impl<T: EllipticCurve> ECDSA<T> {
    pub fn new(curve: T) -> Self {
        debug!("Creating new ECDSA instance");
        ECDSA { curve }
    }

    pub fn generate_keypair(&self) -> (BigUint, Point) {
        debug!("Generating new keypair");
        let private_key = self.generate_random_private_key();
        let public_key = self.generate_public_key(&private_key);

        self.validate_public_key(&public_key);
        info!("Keypair generated successfully");
        (private_key, public_key)
    }

    pub fn generate_public_key(&self, private_key: &BigUint) -> Point {
        debug!("Generating public key from private key");
        self.curve.mul(self.curve.base_point(), private_key)
    }

    pub fn sign(
        &self,
        message: &BigUint,
        private_key: &BigUint,
    ) -> Result<(BigUint, BigUint), &'static str> {
        self.validate_input(message, private_key)?;
        debug!("Signing message");
        let k = self.generate_random_private_key();
        self.sign_with_k(message, private_key, &k)
    }

    pub fn verify(
        &self,
        message: &BigUint,
        signature: &(BigUint, BigUint),
        public_key: &Point,
    ) -> bool {
        debug!("Verifying signature");
        let (r, s) = signature;

        if !self.is_valid_signature(r, s) {
            return false;
        }

        let s_inv = FiniteField::inv_mul(s, self.curve.order());
        let u1 = FiniteField::mul(message, &s_inv, self.curve.order());
        let u2 = FiniteField::mul(r, &s_inv, self.curve.order());
        let point = self.calculate_verification_point(&u1, &u2, public_key);

        self.is_signature_valid(point, r)
    }

    fn validate_input(&self, message: &BigUint, private_key: &BigUint) -> Result<(), &'static str> {
        if private_key >= self.curve.order() {
            return Err("Private key must be less than the order of the curve");
        }
        if message >= self.curve.order() {
            return Err("Message must be less than the order of the curve");
        }
        Ok(())
    }

    fn sign_with_k(
        &self,
        message: &BigUint,
        private_key: &BigUint,
        k: &BigUint,
    ) -> Result<(BigUint, BigUint), &'static str> {
        if k >= self.curve.order() {
            return Err("k must be less than the order of the curve");
        }

        let r = self.calculate_r(k);
        let s = self.calculate_s(message, private_key, k, &r);

        info!("Message signed successfully");
        Ok((r, s))
    }

    fn calculate_r(&self, k: &BigUint) -> BigUint {
        match self.curve.mul(self.curve.base_point(), k) {
            Point::Coordinates(x, _) => x,
            Point::Identity => {
                warn!("Unexpected point at infinity during signing");
                panic!("Unexpected point at infinity");
            }
        }
    }

    fn calculate_s(
        &self,
        message: &BigUint,
        private_key: &BigUint,
        k: &BigUint,
        r: &BigUint,
    ) -> BigUint {
        let s = FiniteField::mul(r, private_key, self.curve.order());
        let s = FiniteField::add(message, &s, self.curve.order());
        let k_inv = FiniteField::inv_mul(k, self.curve.order());
        FiniteField::mul(&s, &k_inv, self.curve.order())
    }

    fn is_valid_signature(&self, r: &BigUint, s: &BigUint) -> bool {
        if r >= self.curve.order() || s >= self.curve.order() {
            warn!("Invalid signature: r or s is too large");
            return false;
        }
        true
    }

    fn calculate_verification_point(
        &self,
        u1: &BigUint,
        u2: &BigUint,
        public_key: &Point,
    ) -> Point {
        let u1a = self.curve.mul(self.curve.base_point(), u1);
        let u2b = self.curve.mul(public_key, u2);
        self.curve.add(&u1a, &u2b)
    }

    fn is_signature_valid(&self, point: Point, r: &BigUint) -> bool {
        match point {
            Point::Coordinates(x, _) => x == *r,
            Point::Identity => {
                warn!("Unexpected point at infinity during verification");
                false
            }
        }
    }

    fn generate_random_private_key(&self) -> BigUint {
        debug!("Generating random private key");
        thread_rng().gen_biguint_range(&BigUint::from(1u32), self.curve.order())
    }

    fn validate_public_key(&self, public_key: &Point) {
        assert!(
            self.curve.is_on_curve(public_key),
            "Generated public key is not on the curve"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn create_test_ecdsa() -> ECDSA<WeierstrassCurve> {
        let curve = WeierstrassCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
            BigUint::from(19u32),
            Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32)),
        );
        ECDSA::new(curve)
    }

    #[test]
    fn test_generate_keypair() {
        init();
        let ecdsa = create_test_ecdsa();
        let (private_key, public_key) = ecdsa.generate_keypair();
        assert!(private_key < *ecdsa.curve.order());
        assert!(
            ecdsa.curve.is_on_curve(&public_key),
            "Generated public key is not on the curve"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        init();
        let ecdsa = create_test_ecdsa();
        let (private_key, public_key) = (
            BigUint::from(7u32),
            ecdsa.generate_public_key(&BigUint::from(7u32)),
        );
        let message = BigUint::from(10u32);
        let signature = ecdsa
            .sign_with_k(&message, &private_key, &BigUint::from(18u32))
            .unwrap();
        assert!(ecdsa.verify(&message, &signature, &public_key));
    }

    #[test]
    fn test_verify_invalid_signature() {
        init();
        let ecdsa = create_test_ecdsa();
        let (private_key, public_key) = (
            BigUint::from(7u32),
            ecdsa.generate_public_key(&BigUint::from(7u32)),
        );
        let message = BigUint::from(5u32);
        let mut signature = ecdsa.sign(&message, &private_key).unwrap();
        signature.1 += BigUint::from(1u32); // Modify the signature to make it invalid
        assert!(!ecdsa.verify(&message, &signature, &public_key));
    }
}
