use bls381::{fp::Fp, scalar::Scalar};
use crypto_bigint::uint::U384;
use crypto_bigint::{consts::U48, subtle::Choice, U256};
use crypto_bigint::{Limb, Uint};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};

///Generator
pub const GENERATOR: (U384, U384) = (U384::from_be_hex("17F1D3A73197D7942695638C4FA9AC0FC3688C4F9774B905A14E3A3F171BAC586C55E83FF97A1AEFFB3AF00ADB22C6BB"),
U384::from_be_hex("08B3F481E3AAA0F1A09E30ED741D8AE4FCF5E095D5D00AF600DB18CB2C04B3EDD03CC744A2888AE40CAA232946C5E7E1"));

// B = 4 in montgomery form
pub const B: Fp = Fp(Uint {
    limbs: [
        Limb(12260768510540316659),
        Limb(6038201419376623626),
        Limb(5156596810353639551),
        Limb(12813724723179037911),
        Limb(10288881524157229871),
        Limb(708830206584151678),
    ],
});

pub const B_3: Fp = Fp(Uint {
    limbs: [
        Limb(4933130441833534766),
        Limb(15904462746612662304),
        Limb(8034115857496836953),
        Limb(12755092135412849606),
        Limb(7007796720291435703),
        Limb(252692002104915169),
    ],
});

pub type FieldElement = Fp;

///Elliptic curve struct
/// having equation y^2 = x^3 + 4
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlsCurve;
// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<BlsCurve>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<BlsCurve>;

/// impl curve, curve params, curve arithmetic on Jubjub curve
/// impl Curve trait for BLS381 curve
impl Curve for BlsCurve {
    type FieldBytesSize = U48;
    type Uint = U256;
    const ORDER: U256 =
        U256::from_be_hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");
}

/// impl CurveArithmetic trait for BLS381 curve
impl CurveArithmetic for BlsCurve {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

/// impl CurveParams trait for BLS381 curve
impl CurveParams for BlsCurve {
    type FieldElement = FieldElement;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = Fp::ZERO;
    const EQUATION_B: Self::FieldElement = B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = (
        Fp(GENERATOR.0).to_montgomery(),
        Fp(GENERATOR.1).to_montgomery(),
    );
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp::ZERO, Fp::ZERO);
}

impl<C: CurveParams> OnCurve<C> for BlsCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }

    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

}
