# Elliptic Curve Cryptography (ECC) Implementation in Rust

This project implements Elliptic Curve Cryptography (ECC) operations in Rust. It provides a foundation for working with elliptic curves, points on these curves, and various cryptographic operations.

## Features

- Elliptic curve representation and operations (in Weierstrass form: y^2 = x^3 + ax + b)
- Point arithmetic on elliptic curves (addition, doubling, scalar multiplication)
- Finite field arithmetic
- ECDSA (Elliptic Curve Digital Signature Algorithm) implementation
- Comprehensive test suite for all implemented operations
- Chaum-Pedersen zero-knowledge proof protocol implementation

## Structure

The project is organized into several modules:

### src/lib.rs

This is the main entry point of the library. It re-exports the public items from other modules.

### src/ec.rs

Contains the `EllipticCurve` trait and `WeierstrassCurve` struct implementation, which represents an elliptic curve in Weierstrass form (y^2 = x^3 + ax + b) and provides methods for curve operations.

### src/point.rs

Defines the `Point` struct, representing a point on an elliptic curve, including the point at infinity (Identity).

### src/ff.rs

Implements the `FiniteField` struct with finite field arithmetic operations such as addition, multiplication, and inversion.

### src/ecdsa.rs

Implements the ECDSA algorithm for digital signatures using elliptic curves.

### src/curves/mod.rs and src/curves/secp256k1.rs

These files contain implementations of specific elliptic curves, currently supporting the secp256k1 curve.

### src/zk/mod.rs and src/zk/chaum_pedersen.rs

These files contain the implementation of the Chaum-Pedersen zero-knowledge proof protocol.

### src/group.rs

Defines the `Group` struct, which represents a cyclic group used in various cryptographic protocols, including Chaum-Pedersen.

Each module contains its own tests, ensuring the correctness of the implemented operations.

## Usage

To use this library in your Rust project, add it as a dependency in your `Cargo.toml` file:


```toml
[dependencies]
ecc-rust = { git = "https://github.com/yourusername/ecc-rust.git" }
```

Example usage:

```rust
use ecc_rust::{WeierstrassCurve, Point, FiniteField, EllipticCurve, ECDSA, create_secp256k1_curve, ChaumPedersen, Group};
use num_bigint::BigUint;

fn main() {
    // Create the secp256k1 curve
    let curve = create_secp256k1_weierstrass();

    // Create an ECDSA instance
    let ecdsa = ECDSA::new(curve);

    // Generate a keypair
    let (private_key, public_key) = ecdsa.generate_keypair();
    println!("Private key: {}", private_key);
    println!("Public key: {:?}", public_key);

    // Sign a message
    let message = BigUint::from(12345u32);
    let signature = ecdsa.sign(&message, &private_key).unwrap();
    println!("Signature: {:?}", signature);

    // Verify the signature
    let is_valid = ecdsa.verify(&message, &signature, &public_key);
    println!("Signature is valid: {}", is_valid);

    // Perform point addition
    let p1 = ecdsa.curve.mul(&g, &BigUint::from(2u32));
    let p2 = ecdsa.curve.mul(&g, &BigUint::from(3u32));
    let sum = ecdsa.curve.add(&p1, &p2);
    println!("Point addition result: {:?}", sum);

    // Perform scalar multiplication
    let scalar = BigUint::from(5u32);
    let product = ecdsa.curve.mul(&g, &scalar);
    println!("Scalar multiplication result: {:?}", product);

    // Chaum-Pedersen protocol example
    let group = Group::new(
        BigUint::from(23u32), // p
        BigUint::from(11u32), // q
        BigUint::from(4u32),  // g
        BigUint::from(9u32),  // h
    );
    let chaum_pedersen = ChaumPedersen::new(group);

    let x = BigUint::from(2u32); // secret
    let k = BigUint::from(3u32); // random value

    let commitment = chaum_pedersen.commit(&x, &k);
    let challenge = chaum_pedersen.challenge(&BigUint::from(1u32));
    let proof = chaum_pedersen.proof(&k, &challenge.c, &x);

    let is_valid = chaum_pedersen.verify(&commitment, &challenge, &proof);
    println!("Chaum-Pedersen proof is valid: {}", is_valid);
}
```

This example demonstrates:

1. Creating the secp256k1 curve
2. Initializing an ECDSA instance
3. Generating a keypair
4. Signing and verifying a message
5. Performing point addition and scalar multiplication on the curve
6. Using the Chaum-Pedersen zero-knowledge proof protocol

Make sure to handle any potential errors or invalid inputs in your actual implementation.


## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
