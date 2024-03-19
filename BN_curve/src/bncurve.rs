///Imports
use bn254::scalar::Scalar;
use bn254::fp::Fp;
use crypto_bigint::{consts::U32, subtle::Choice, U256};
use curve_traits::{
    affinepoint::{self}, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};

//254-bit prime field Weierstrass curve
///a = 0x0000000000000000000000000000000000000000000000000000000000000000
pub const EQUATION_A: Fp = Fp::ZERO;

///b = 0x0000000000000000000000000000000000000000000000000000000000000003
pub const EQUATION_B: Fp = Fp(U256::from_u8(3));

pub const GENERATOR: (Fp, Fp) = (Fp::ONE, Fp(U256::from_u8(2)));

///Elliptic curve struct
/// BN equation is y^2 = x^3 + 3 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BNCurve;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<BNCurve>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<BNCurve>;

/// impl curve, curve params, curve arithmetic on BN curve
/// impl Curve trait for BN curve
impl Curve for BNCurve {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: Self::Uint =
    // order of curve is the order of the scalar field 21888242871839275222246405745257275088548364400416034343698204186575808495617
        U256::from_be_hex("30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001");
}

/// impl curveArithmetic trait on BN curve
impl CurveArithmetic for BNCurve {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar; 
}

///impl CurveParams for for BN curve
impl CurveParams for BNCurve {
    type FieldElement = Fp;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = EQUATION_A;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp::ZERO, Fp::ONE);
}

impl<C: CurveParams> OnCurve<C> for BNCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

}



