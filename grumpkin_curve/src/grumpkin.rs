
///Imports
use bn254::fp::Fp as Scalar;
use bn254::scalar::Scalar as Fp;
use crypto_bigint::{consts::U32, subtle::Choice, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};

// Y^2 = x^3 -17

///a = 0
pub const EQUATION_A: Fp = Fp::ZERO;

///b = -17
pub const EQUATION_B: Fp = Fp(U256::from_be_hex(
    "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593EFFFFFF0",
));

pub const GENERATOR: (Fp, Fp) = (
    Fp::ONE,
    Fp(U256::from_be_hex(
        "0000000000000002CF135E7506A45D632D270D45F1181294833FC48D823F272C",
    )),
);

///Elliptic curve struct
/// having equation y^2 = x^3 - 17
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Grumpkin;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<Grumpkin>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<Grumpkin>;

/// impl curve, curve params, curve arithmetic on Jubjub curve
/// impl Curve trait for Jubjub curve
impl Curve for Grumpkin {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: Self::Uint =
        U256::from_be_hex("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47");
}

/// impl curveArithmetic trait on Jubjub curve
impl CurveArithmetic for Grumpkin {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

///impl CurveParams for for Jubjub curve
impl CurveParams for Grumpkin {
    type FieldElement = Fp;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = EQUATION_A;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp::ZERO, Fp::ZERO);
}

impl<C: CurveParams> OnCurve<C> for Grumpkin {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }
}
