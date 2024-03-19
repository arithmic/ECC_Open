use crypto_bigint::consts::U32;
use crypto_bigint::{subtle::Choice, U256};

use curve_traits::{
    affinepoint, arithmeticops::EquationIsONE, AffinePoint, OnCurve, ProjectivePoint,
};

///imports
pub use curve_traits::{Curve, CurveArithmetic, CurveParams};
use stark252::{
    field::Fp,
    scalar::{Scalar, SCALAR_MODULUS},
};
///Elliptic curve requirements
///
/// field element type global
pub type FieldElement = Fp;

pub type FieldBytesSize = u32;

/// 3B = 3*B
pub const EQUATION_3B: FieldElement = Fp(U256::from_be_hex(
    "04d63c3bcf3ac2783f2b0c4858e6fa5021d07744415b4145de69f62cd6cbdb99",
));

/// ec curve equation
/// y^2 = X^3 + x + b
/// ec curve struct
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct Stark252;

/// impl curve, curve params, curve arithmetic on stark252
/// impl Curve trait for Stark252
impl Curve for Stark252 {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: Self::Uint = SCALAR_MODULUS;
}

/// impl Curve Params trait for stark 252 of prime order
/// short Weierstrass equation
impl CurveParams for Stark252 {
    type FieldElement = FieldElement;
    type PointArithmetic = EquationIsONE;
    const EQUATION_A: Self::FieldElement = Fp::ONE;
    const EQUATION_B: Self::FieldElement = Fp(U256::from_be_hex(
        "06F21413EFBE40DE150E596D72F7A8C5609AD26C15C915C1F4CDFCB99CEE9E89",
    ));
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = (
        Fp(U256::from_be_hex(
            "01EF15C18599971B7BECED415A40F0C7DEACFD9B0D1819E03D723D8BC943CFCA",
        )),
        Fp(U256::from_be_hex(
            "005668060AA49730B7BE4801DF46EC62DE53ECD11ABE43A32873000C36E8DC1F",
        )),
    );
    const IDENTITY: (Self::FieldElement, Self::FieldElement) =
        (FieldElement::ZERO, FieldElement::ZERO,);
}

/// impl curve arithmetic trait on stark 252 curve
impl CurveArithmetic for Stark252 {
    type AffinePoint = AffinePoint<Stark252>;
    type ProjectivePoint = ProjectivePoint<Stark252>;
    type Scalar = Scalar;
}

impl<C: CurveParams> OnCurve<C> for Stark252 {
    fn is_on_curve(point: AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }


}
