use bn254::fp::Fp;
use bn254::fp12::Fp12;
use bn254::scalar::Scalar;
use crypto_bigint::subtle::{Choice, ConstantTimeEq};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use traits::traits::Field;
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Gt(pub Fp12<Fp>);
impl Gt {
    // constants zero and one
    pub const ZERO: Gt = Gt(Fp12::ZERO);
    pub const ONE: Gt = Gt(Fp12::ONE);

    // Returns self + other
    pub fn add(&self, other: &Self) -> Self {
        let result = Fp12::mul(self.0, other.0);
        Gt(result)
    }

    // Returns -self
    pub fn neg(self) -> Self {
        let result = Fp12::neg(self.0);
        Self(result)
    }

    // Returns self - other
    pub fn sub(&self, other: &Self) -> Self {
        let result = Fp12::sub(self.0, other.0);
        Self(result)
    }

    // Returns 2 * self
    pub fn double(self) -> Self {
        let result = Fp12::square(self.0);
        Self(result)
    }

    // returns the self**pow where pow is given as array of u64(word)
    pub fn mul(self, pow: Scalar) -> Self {
        let result = Fp12::power_by(self.0, pow.0.to_words());
        Self(result)
    }
}

impl Mul<&Scalar> for Gt {
    type Output = Gt;

    fn mul(self, other: &Scalar) -> Gt {
        Gt::mul(self, *other)
    }
}

impl Add<&Gt> for Gt {
    type Output = Gt;

    fn add(self, other: &Gt) -> Gt {
        Gt::add(&self, other)
    }
}

impl AddAssign<Gt> for Gt {
    fn add_assign(&mut self, rhs: Gt) {
        *self = Gt::add(&self, &rhs);
    }
}

impl Sub<&Gt> for Gt {
    type Output = Gt;

    fn sub(self, other: &Gt) -> Gt {
        Gt::sub(&self, other)
    }
}

impl SubAssign<Gt> for Gt {
    fn sub_assign(&mut self, rhs: Gt) {
        *self = Gt::sub(&self, &rhs);
    }
}

impl ConstantTimeEq for Gt {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl PartialEq for Gt {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl PartialOrd for Gt {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Gt {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
