use bn254::{fp::Fp, fp2::Fp2, scalar::Scalar};
use crypto_bigint::{consts::U32, subtle::Choice, Limb, Uint, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsZero, Curve, CurveArithmetic, CurveParams, OnCurve,
};

pub type G2AffinePoint = curve_traits::affinepoint::AffinePoint<G2BNCurve>;
pub type G2ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<G2BNCurve>;

//c0: 19485874751759354771024239261021720505790618469301721065564631296452457478373
//c1: 266929791119991161246907387137283842545076965332900288569378510910307636690
pub const EQUATION_B: Fp2<Fp> = Fp2 {
    c0: Fp(Uint {
        limbs: [
            Limb(3632125457679333605),
            Limb(13093307605518643107),
            Limb(9348936922344483523),
            Limb(3104278944836790958),
        ],
    }),
    c1: Fp(Uint {
        limbs: [
            Limb(16474938222586303954),
            Limb(12056031220135172178),
            Limb(14784384838321896948),
            Limb(42524369107353300),
        ],
    }),
};

pub const GENERATOR: (Fp2<Fp>, Fp2<Fp>) = (
    Fp2 {
        c0: Fp(U256::from_be_hex(
            "1800DEEF121F1E76426A00665E5C4479674322D4F75EDADD46DEBD5CD992F6ED",
        )),
        c1: Fp(U256::from_be_hex(
            "198E9393920D483A7260BFB731FB5D25F1AA493335A9E71297E485B7AEF312C2",
        )),
    },
    Fp2 {
        c0: Fp(U256::from_be_hex(
            "12C85EA5DB8C6DEB4AAB71808DCB408FE3D1E7690C43D37B4CE6CC0166FA7DAA",
        )),
        c1: Fp(U256::from_be_hex(
            "090689D0585FF075EC9E99AD690C3395BC4B313370B38EF355ACDADCD122975B",
        )),
    },
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct G2BNCurve;
//
impl Curve for G2BNCurve {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: U256 =
        //order of the scalar field
        U256::from_be_hex("30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001");
}

/// impl CurveArithmetic trait for BLS381 curve
impl CurveArithmetic for G2BNCurve {
    type AffinePoint = G2AffinePoint;
    type ProjectivePoint = G2ProjectivePoint;
    type Scalar = Scalar;
}

/// impl CurveParams trait for BLS381 curve
impl CurveParams for G2BNCurve {
    type FieldElement = Fp2<Fp>;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = Fp2::ZERO;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp2::ZERO, Fp2::ONE);
}

impl<C: CurveParams> OnCurve<C> for G2BNCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }

    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

}
