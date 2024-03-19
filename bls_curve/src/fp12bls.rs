use bls381::{fp::Fp, fp12::Fp12, fp2::Fp2, fp6::Fp6, scalar::Scalar};
use crypto_bigint::{consts::U48, subtle::Choice, Limb, Uint, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};
use traits::traits::Field;
///Elliptic curve struct
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct F12BlsCurve;
// specifying the path for Affine Point and Projective Point
pub type FP12AffinePoint = curve_traits::affinepoint::AffinePoint<F12BlsCurve>;
pub type Fp12ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<F12BlsCurve>;
impl Curve for F12BlsCurve {
    type FieldBytesSize = U48;
    type Uint = U256;
    const ORDER: U256 =
        U256::from_be_hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");
}

/// impl CurveArithmetic trait for BLS381 curve
impl CurveArithmetic for F12BlsCurve {
    type AffinePoint = FP12AffinePoint;
    type ProjectivePoint = Fp12ProjectivePoint;
    type Scalar = Scalar;
}

/// impl CurveParams trait for BLS381 curve
impl CurveParams for F12BlsCurve {
    type FieldElement = Fp12<Fp>;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = Fp12::ZERO;
    const EQUATION_B: Self::FieldElement = Fp12 {
        c0: Fp6 {
            c0: Fp2 {
                c0: Fp(Uint {
                    limbs: [
                        Limb(12260768510540316659),
                        Limb(6038201419376623626),
                        Limb(5156596810353639551),
                        Limb(12813724723179037911),
                        Limb(10288881524157229871),
                        Limb(708830206584151678),
                    ],
                }),
                c1: Fp::ZERO,
            },
            c1: Fp2::ZERO,
            c2: Fp2::ZERO,
        },
        c1: Fp6::ZERO,
    };
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = (
        Fp12 {
            c0: Fp6 {
                c0: Fp2::ZERO,
                c1: Fp2::ZERO,
                c2: Fp2 {
                    c0: Fp(Uint {
                        limbs: [
                            Limb(5606462617328287755),
                            Limb(12631295554488433519),
                            Limb(7975040303647731395),
                            Limb(6473827934145556776),
                            Limb(11029124417308089817),
                            Limb(831439482444633445),
                        ],
                    }),
                    c1: Fp(Uint {
                        limbs: [
                            Limb(6330821281390786043),
                            Limb(18110492783210685780),
                            Limb(14773061534522132802),
                            Limb(13925990517507785278),
                            Limb(3001537920258090861),
                            Limb(434681182878701710),
                        ],
                    }),
                },
            },
            c1: Fp6::ZERO,
        },
        Fp12 {
            c0: Fp6::ZERO,
            c1: Fp6 {
                c0: Fp2::ZERO,
                c1: Fp2 {
                    c0: Fp(Uint {
                        limbs: [
                            Limb(6492365222644732841),
                            Limb(5217860609106738971),
                            Limb(16925233543524655759),
                            Limb(14760599312231038892),
                            Limb(2090607828866420403),
                            Limb(1357950792441491129),
                        ],
                    }),
                    c1: Fp(Uint {
                        limbs: [
                            Limb(983606391556900703),
                            Limb(17216300903697015489),
                            Limb(215043374364081972),
                            Limb(1218356693526296141),
                            Limb(1526627126496504081),
                            Limb(1320798782042837972),
                        ],
                    }),
                },
                c2: Fp2::ZERO,
            },
        },
    );

    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp12::ZERO, Fp12::ZERO);
}

impl<C: CurveParams> OnCurve<C> for F12BlsCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }
}
