///Imports
use bls381::jubjub_scalar::JubScalar as Scalar;
use bls381::scalar::Scalar as Fp;
use crypto_bigint::{consts::U48, subtle::Choice, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsGeneric, Curve, CurveArithmetic, CurveParams, OnCurve,
};

///a = 52296097456646850916096512823759002727550416093741407922227928430486925478210
pub const EQUATION_A: Fp = Fp(U256::from_be_hex(
    "739e8acf6e7266e8976220a0378a232820535e745e639334c50d34dcd4c20942",
));

///b = 48351165704696163914533707656614864561753505123260775585269522553028192119009
pub const EQUATION_B: Fp = Fp(U256::from_be_hex(
    "6ae5ca3c3f667667f4c60001b55614b0bbf51483ff8366ac7120c33b4c6628e1",
));

pub const GENERATOR: (Fp, Fp) = (
    Fp(U256::from_be_hex(
        "4ace6c5be1410abd4c147b613b38a4e65c3c3cefd50627cc9c7d2809edadbbcb",
    )),
    Fp(U256::from_be_hex(
        "60c90ea806b4993eda65d96da6670461fee090b1357412745ce5305e368eff18",
    )),
);

///Elliptic curve struct
/// having equation y^2 = x^3 + ax + b
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Jubjub;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<Jubjub>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<Jubjub>;

/// impl curve, curve params, curve arithmetic on Jubjub curve
/// impl Curve trait for Jubjub curve
impl Curve for Jubjub {
    type FieldBytesSize = U48;
    type Uint = U256;
    const ORDER: Self::Uint =
        U256::from_be_hex("0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7");
}

/// impl curveArithmetic trait on Jubjub curve
impl CurveArithmetic for Jubjub {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar; // should be different
}

///impl CurveParams for for Jubjub curve
impl CurveParams for Jubjub {
    type FieldElement = Fp;
    type PointArithmetic = EquationIsGeneric;
    const EQUATION_A: Self::FieldElement = EQUATION_A;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp::ZERO, Fp::ONE);
}

impl<C: CurveParams> OnCurve<C> for Jubjub {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

}
