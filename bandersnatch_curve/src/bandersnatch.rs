///Imports
use bls381::bandersnatch_scalar::BandScalar as Scalar;
use bls381::scalar::Scalar as Fp;
use crypto_bigint::{consts::U48, subtle::Choice, Limb, Uint, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsGeneric, Curve, CurveArithmetic, CurveParams, OnCurve,
};

//Generator
pub const GENERATOR: (Fp, Fp) = (
    //
    Fp(U256::from_be_hex(
        "0a76451786f95a802c0982bbd0abd68e41b92adc86c8859b4f44679b21658710",
    )),
    Fp(U256::from_be_hex(
        "44d150c8b4bd14f79720d021a839e7b7eb4ee43844b30243126a72ac2375490a",
    )),
);

//Identity of affine point
pub const IDENTITY: (Fp, Fp) = (Fp::ZERO, Fp::ZERO);

//Equation A
/// a = -3763200000
pub const EQUATION_A: Fp = Fp(Uint {
    limbs: [
        Limb(18446744065651384321),
        Limb(6034159408538082302),
        Limb(3691218898639771653),
        Limb(8353516859464449352),
    ],
});

//equation B
// B = - 78675968000000
pub const EQUATION_B: Fp = Fp(Uint {
    limbs: [
        Limb(18446665393446584321),
        Limb(6034159408538082302),
        Limb(3691218898639771653),
        Limb(8353516859464449352),
    ],
});

///Elliptic curve struct
/// having equation y^2 = x^3 + ax + b
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bandersnatch;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<Bandersnatch>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<Bandersnatch>;

/// impl curve, curve params, curve arithmetic on Bandersnatch curve
/// impl Curve trait for Bandersnatch curve
impl Curve for Bandersnatch {
    type FieldBytesSize = U48;
    type Uint = U256;
    const ORDER: Self::Uint =
        U256::from_be_hex("1CFB69D4CA675F520CCE760202687600FF8F87007419047174FD06B52876E7E1");
}

//impl CurveParams trait for bandersnatch
impl CurveParams for Bandersnatch {
    type FieldElement = Fp;
    type PointArithmetic = EquationIsGeneric;
    const EQUATION_A: Self::FieldElement = EQUATION_A;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = IDENTITY;
}

//impl CurveArithmetic for bandersnatch
impl CurveArithmetic for Bandersnatch {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

impl<C: CurveParams> OnCurve<C> for Bandersnatch {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }
}
