// Chaum-Pedersen Protocol

use crate::ff::FiniteField;
use crate::group::Group;
use num_bigint::BigUint;
use log::debug;

pub struct ChaumPedersen {
    pub group: Group,
}

pub struct Commitment {
    pub r1: BigUint, // r1 = g^k mod p
    pub r2: BigUint, // r2 = h^k mod p
    pub y1: BigUint, // y1 = g^x mod p
    pub y2: BigUint, // y2 = h^x mod p
}

pub struct Challenge {
    pub c: BigUint, // random challenge
}

pub struct Proof {
    pub s: BigUint, // s = k - cx mod q
}

impl ChaumPedersen {
    // The Chaum-Pedersen Protocol is a zero-knowledge proof system that allows
    // a prover to demonstrate knowledge of a discrete logarithm without
    // revealing the actual value. It's used to prove that two discrete
    // logarithms are equal without disclosing the shared exponent.
    //
    // The protocol works as follows:
    // 1. Setup:
    //    - A cyclic group with prime order q is chosen, typically a subgroup of
    //      Z_p*.
    //    - Two generators g and h are selected from this group.
    //    - The prover knows a secret x, and wants to prove that y1 = g^x and y2
    //      = h^x.
    //
    // 2. Commitment:
    //    - Prover chooses a random k from Z_q.
    //    - Computes r1 = g^k mod p and r2 = h^k mod p.
    //    - Sends (r1, r2, y1, y2) to the verifier.
    //
    // 3. Challenge:
    //    - Verifier chooses a random challenge c from Z_q and sends it to the
    //      prover.
    //
    // 4. Proof:
    //    - Prover computes s = k - cx mod q.
    //    - Sends s to the verifier.
    //
    // 5. Verification:
    //    - Verifier checks if: g^s * y1^c ≡ r1 (mod p)  and  h^s * y2^c ≡ r2
    //      (mod p)
    //    - If both equations hold, the proof is accepted; otherwise, it's
    //      rejected.
    //
    // This protocol ensures that the prover knows x without revealing its
    // value, and that the same x is used in both y1 and y2.

    pub fn new(group: Group) -> Self {
        Self { group }
    }

    pub fn commit(&self, x: &BigUint, k: &BigUint) -> Commitment {
        debug!("Generating commitment");
        let commitment = Commitment {
            r1: FiniteField::exp(&self.group.g, k, &self.group.p),
            r2: FiniteField::exp(&self.group.h, k, &self.group.p),
            y1: FiniteField::exp(&self.group.g, x, &self.group.p),
            y2: FiniteField::exp(&self.group.h, x, &self.group.p),
        };
        debug!("Commitment generated: r1={}, r2={}, y1={}, y2={}", commitment.r1, commitment.r2, commitment.y1, commitment.y2);
        commitment
    }

    pub fn challenge(&self, c: &BigUint) -> Challenge {
        debug!("Creating challenge: c={}", c);
        Challenge { c: c.clone() }
    }

    // generates s = k - cx mod q
    pub fn proof(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> Proof {
        debug!("Generating proof");
        let cx = FiniteField::mul(c, x, &self.group.q);
        let s = FiniteField::sub(k, &cx, &self.group.q);
        debug!("Proof generated: s={}", s);
        Proof { s }
    }

    // r1 == g^s * y1^c mod p
    // r2 == h^s * y2^c mod p
    // returns true if valid, false otherwise
    pub fn verify(&self, commitment: &Commitment, challenge: &Challenge, proof: &Proof) -> bool {
        debug!("Verifying Chaum-Pedersen proof");
        debug!("Commitment: r1={}, r2={}, y1={}, y2={}", commitment.r1, commitment.r2, commitment.y1, commitment.y2);
        debug!("Challenge: c={}", challenge.c);
        debug!("Proof: s={}", proof.s);

        let g_s = FiniteField::exp(&self.group.g, &proof.s, &self.group.p);
        let y1_c = FiniteField::exp(&commitment.y1, &challenge.c, &self.group.p);
        let left_side = FiniteField::mul(&g_s, &y1_c, &self.group.p);
        debug!("Left side verification: g^s * y1^c mod p = {}", left_side);

        let h_s = FiniteField::exp(&self.group.h, &proof.s, &self.group.p);
        let y2_c = FiniteField::exp(&commitment.y2, &challenge.c, &self.group.p);
        let right_side = FiniteField::mul(&h_s, &y2_c, &self.group.p);
        debug!("Right side verification: h^s * y2^c mod p = {}", right_side);

        let result = left_side == commitment.r1 && right_side == commitment.r2;
        debug!("Verification result: {}", result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::group::Group;
    use num_bigint::BigUint;
    use env_logger;

    fn init() {
      let _ = env_logger::builder().is_test(true).try_init();
  }

    mod toy_example {
        use super::*;

        fn setup() -> (ChaumPedersen, BigUint, BigUint) {
            init();
            let group = Group::new(
                BigUint::from(23u32),
                BigUint::from(11u32),
                BigUint::from(4u32),
                BigUint::from(9u32),
            );
            let chaum_pedersen = ChaumPedersen::new(group);
            let x = BigUint::from(2u32);
            let k = BigUint::from(3u32);
            (chaum_pedersen, x, k)
        }

        #[test]
        fn test_positive_case() {
            let (chaum_pedersen, x, k) = setup();

            let commitment = chaum_pedersen.commit(&x, &k);
            let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
            let proof = chaum_pedersen.proof(&k, &challenge.c, &x);

            assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));
        }

        #[test]
        fn test_negative_case() {
            let (chaum_pedersen, x, k) = setup();

            let commitment = chaum_pedersen.commit(&x, &k);
            let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));

            // Use a different x value for the proof
            let wrong_x = BigUint::from(3u32);
            let proof = chaum_pedersen.proof(&k, &challenge.c, &wrong_x);

            assert!(!chaum_pedersen.verify(&commitment, &challenge, &proof));
        }

        #[test]
        fn test_edge_cases() {
            let (chaum_pedersen, _, k) = setup();

            // Edge case: x = 0
            let x = BigUint::from(0u32);
            let commitment = chaum_pedersen.commit(&x, &k);
            let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
            let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
            assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));

            // Edge case: x = p - 1 (largest possible value)
            let x = &chaum_pedersen.group.p - BigUint::from(1u32);
            let commitment = chaum_pedersen.commit(&x, &k);
            let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
            let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
            assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));

            // Edge case: challenge = 0
            let x = BigUint::from(2u32);
            let commitment = chaum_pedersen.commit(&x, &k);
            let challenge = chaum_pedersen.challenge(&BigUint::from(0u32));
            let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
            assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));
        }
    }

    mod real_example {
        use super::*;
        // https://datatracker.ietf.org/doc/rfc5114/

        mod rfc5114_1024bit {
            // 1024-bit MODP Group with 160-bit Prime Order Subgroup

            // The hexadecimal value of the prime is:

            // p = B10B8F96 A080E01D DE92DE5E AE5D54EC 52C99FBC FB06A3C6
            //     9A6A9DCA 52D23B61 6073E286 75A23D18 9838EF1E 2EE652C0
            //     13ECB4AE A9061123 24975C3C D49B83BF ACCBDD7D 90C4BD70
            //     98488E9C 219A7372 4EFFD6FA E5644738 FAA31A4F F55BCCC0
            //     A151AF5F 0DC8B4BD 45BF37DF 365C1A65 E68CFDA7 6D4DA708
            //     DF1FB2BC 2E4A4371

            // The hexadecimal value of the generator is:

            // g = A4D1CBD5 C3FD3412 6765A442 EFB99905 F8104DD2 58AC507F
            //     D6406CFF 14266D31 266FEA1E 5C41564B 777E690F 5504F213
            //     160217B4 B01B886A 5E91547F 9E2749F4 D7FBD7D3 B9A92EE1
            //     909D0D22 63F80A76 A6A24C08 7A091F53 1DBF0A01 69B6A28A
            //     D662A4D1 8E73AFA3 2D779D59 18D08BC8 858F4DCE F97C2A24
            //     855E6EEB 22B3B2E5

            // The generator generates a prime-order subgroup of size:

            // q = F518AA87 81A8DF27 8ABA4E7D 64B7CB9D 49462353

            use super::*;

            fn setup() -> (ChaumPedersen, BigUint, BigUint) {
                init();

                let p = BigUint::parse_bytes(
                    b"B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371",
                    16,
                )
                .unwrap();
                let q = BigUint::parse_bytes(
                    b"F518AA8781A8DF278ABA4E7D64B7CB9D49462353",
                    16,
                )
                .unwrap();
                let g = BigUint::parse_bytes(
                    b"A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5",
                    16,
                )
                .unwrap();

                let h = g.clone().modpow(&BigUint::from(2u32), &p);

                let group = Group::new(p, q, g, h);

                let chaum_pedersen = ChaumPedersen::new(group);
                let x = BigUint::from(2u32);
                let k = BigUint::from(3u32);
                (chaum_pedersen, x, k)
            }

            #[test]
            fn test_positive_case() {
                let (chaum_pedersen, x, k) = setup();

                let commitment = chaum_pedersen.commit(&x, &k);
                let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
                let proof = chaum_pedersen.proof(&k, &challenge.c, &x);

                assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));
            }

            #[test]
            fn test_negative_case() {
                let (chaum_pedersen, x, k) = setup();

                let commitment = chaum_pedersen.commit(&x, &k);
                let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));

                // Use a different x value for the proof
                let wrong_x = BigUint::from(3u32);
                let proof = chaum_pedersen.proof(&k, &challenge.c, &wrong_x);

                assert!(!chaum_pedersen.verify(&commitment, &challenge, &proof));
            }

            #[test]
            fn test_edge_cases() {
                let (chaum_pedersen, _, k) = setup();

                // Edge case: x = 0
                let x = BigUint::from(0u32);
                let commitment = chaum_pedersen.commit(&x, &k);
                let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
                let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
                assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));

                // Edge case: x = q - 1 (largest possible value)
                let x = &chaum_pedersen.group.q - BigUint::from(1u32);
                let commitment = chaum_pedersen.commit(&x, &k);
                let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
                let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
                assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));

                // Edge case: challenge = 0
                let x = BigUint::from(2u32);
                let commitment = chaum_pedersen.commit(&x, &k);
                let challenge = chaum_pedersen.challenge(&BigUint::from(0u32));
                let proof = chaum_pedersen.proof(&k, &challenge.c, &x);
                assert!(chaum_pedersen.verify(&commitment, &challenge, &proof));
            }
        }
    }
}