use bn254::{fp::Fp, fp12::Fp12, fp2::Fp2, fp6::Fp6, scalar::Scalar};
use crypto_bigint::{consts::U32, subtle::Choice, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};
use traits::traits::Field;

// specifying the path for Affine Point and Projective Point
pub type FP12AffinePoint = curve_traits::affinepoint::AffinePoint<F12BNCurve>;
pub type Fp12ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<F12BNCurve>;

///Elliptic curve struct
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F12BNCurve;

impl Curve for F12BNCurve {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: U256 =
    // order of the scalar field of BN
        U256::from_be_hex("30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001");
}

/// impl CurveArithmetic trait for BN curve
impl CurveArithmetic for F12BNCurve {
    type AffinePoint = FP12AffinePoint;
    type ProjectivePoint = Fp12ProjectivePoint;
    type Scalar = Scalar;
}

/// impl CurveParams trait for BN curve
impl CurveParams for F12BNCurve {
    type FieldElement = Fp12<Fp>;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = Fp12::ZERO;
    //c0 + c1 u
    const EQUATION_B: Self::FieldElement = Fp12 {
        c0: Fp6 {
                c0: Fp2 {
                    c0: Fp(U256::from_u8(3)),
                    c1: Fp::ZERO,
                },
                c1: Fp2::ZERO,
                c2: Fp2::ZERO,
            },
        c1: Fp6::ZERO,
    };

    const GENERATOR: (Self::FieldElement, Self::FieldElement) = unimplemented!();

    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp12::ZERO, Fp12::ONE);
}

impl<C: CurveParams> OnCurve<C> for F12BNCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

}
