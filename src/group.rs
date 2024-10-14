use num_bigint::BigUint;

pub struct Group {
  pub p: BigUint,
  pub q: BigUint,
  pub g: BigUint,
  pub h: BigUint,
}

impl Group {
  pub fn new(p: BigUint, q: BigUint, g: BigUint, h: BigUint) -> Self {
    Self { p, q, g, h }
  }
}

