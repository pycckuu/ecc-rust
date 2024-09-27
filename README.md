# Elliptic Curve Cryptography (ECC) Implementation in Rust

This project implements Elliptic Curve Cryptography (ECC) operations in Rust. It provides a foundation for working with elliptic curves, points on these curves, and various cryptographic operations.

## Features

- Elliptic curve representation and operations (in Weierstrass form: y^2 = x^3 + ax + b)
- Point arithmetic on elliptic curves (addition, doubling, scalar multiplication)
- Finite field arithmetic
- Comprehensive test suite for all implemented operations

## Structure

The project is organized into several modules:

### src/lib.rs

This is the main entry point of the library. It re-exports the public items from other modules.

### src/ec.rs

Contains the `EllipticCurve` struct and its implementation, which represents an elliptic curve in Weierstrass form (y^2 = x^3 + ax + b) and provides methods for curve operations.

### src/point.rs

Defines the `Point` struct, representing a point on an elliptic curve, including the point at infinity (Identity).

### src/ff.rs

Implements the `FiniteField` struct with finite field arithmetic operations such as addition, multiplication, and inversion.

### src/curves/mod.rs and src/curves/secp256k1.rs

These files  contain implementations of specific elliptic curves

Each module contains its own tests, ensuring the correctness of the implemented operations.

### Supported Curves

This implementation currently supports the following elliptic curve:

#### secp256k1

The secp256k1 curve is widely used in cryptocurrencies, most notably Bitcoin. It is defined by the following parameters:

- **Equation**: y^2 = x^3 + 7 (over a finite field)
- **Prime field**: p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
- **Base point (G) coordinates**:
  - x = 79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798 (hex)
  - y = 483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8 (hex)
- **Order (n)**: FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141 (hex)

The implementation of the secp256k1 curve can be found in: `src/curves/secp256k1.rs`

## Usage

To use this library in your Rust project, add it as a dependency in your `Cargo.toml` file:


```toml
[dependencies]
ecc-rust = { git = "https://github.com/pycckuu/ecc-rust.git" }
```

Example usage:

```rust
use ecc_rust::{EllipticCurve, Point, FiniteField};
use num_bigint::BigUint;

fn main() {
    // Create a new elliptic curve in Weierstrass form: y^2 = x^3 + 2x + 2 over F_17
    let curve = EllipticCurve {
        a: BigUint::from(2u32),
        b: BigUint::from(2u32),
        p: BigUint::from(17u32)
    };

	// Create two points on the curve
    let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
    let p2 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));

	// Add the points
    let sum = curve.add(&p1, &p2);
    println!("Sum: {:?}", sum);

	// Perform scalar multiplication
    let scalar = BigUint::from(5u32);
    let product = curve.mul(&p1, &scalar);
    println!("5 P1: {:?}", product);

	// Demonstrate finite field operations
    let a = BigUint::from(7u32);
    let b = BigUint::from(10u32);
    let p = BigUint::from(17u32);
    let sum_ff = FiniteField::add(&a, &b, &p);
    let product_ff = FiniteField::mul(&a, &b, &p);

	println!("(7 + 10) mod 17 = {}", sum_ff);
    println!("(7 * 10) mod 17 = {}", product_ff);
}
```

This example demonstrates:

1. Creating an elliptic curve
2. Defining points on the curve
3. Adding points on the curve
4. Performing scalar multiplication
5. Using finite field operations

Make sure to handle any potential errors or invalid inputs in your actual implementation.


## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
