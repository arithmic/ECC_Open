///Imports
use bn254::babyjub_scalar::BabyjubScalar as Scalar;
use bn254::scalar::Scalar as Fp;
use crypto_bigint::{consts::U32, subtle::Choice, U256};
use curve_traits::{
    affinepoint, arithmeticops::EquationIsGeneric, Curve, CurveArithmetic, CurveParams, OnCurve,
};

///a = 7296080957279758407415468581752425029516121466805344781232734728849116493472
pub const EQUATION_A: Fp = Fp(U256::from_be_hex(
    "10216F7BA065E00DE81AC1E7808072C9B8114D6D7DE87ADB16A0A72F1A91F6A0",
));

///b = 16213513238399463127589930181672055621146936592900766180517188641980520820846
pub const EQUATION_B: Fp = Fp(U256::from_be_hex(
    "23D885F647FED5743CAD3D1EE4ABA9C043B4AC0FC2766658A410EFDEB21F706E",
));

pub const GENERATOR: (Fp, Fp) = (
    Fp(U256::from_be_hex(
        "0EC9255DF7CFDD67538B81BF7CB0B2208BBEB7F939E912DFF8FC0325ECCA8FDF",
    )),
    Fp(U256::from_be_hex(
        "289541D597CBEEC836CBC1F5CE4706EF64AB797FD4A9316D34485A22FC417CE4",
    )),
);

///Elliptic curve struct
/// having equation y^2 = x^3 + ax + b
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BabyJubjub;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<BabyJubjub>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<BabyJubjub>;

/// impl curve, curve params, curve arithmetic on Jubjub curve
/// impl Curve trait for Jubjub curve
impl Curve for BabyJubjub {
    type FieldBytesSize = U32;
    type Uint = U256;
    const ORDER: Self::Uint =
        U256::from_be_hex("060C89CE5C263405370A08B6D0302B0BAB3EEDB83920EE0A677297DC392126F1");
}

/// impl curveArithmetic trait on Jubjub curve
impl CurveArithmetic for BabyJubjub {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

///impl CurveParams for for Jubjub curve
impl CurveParams for BabyJubjub {
    type FieldElement = Fp;
    type PointArithmetic = EquationIsGeneric;
    const EQUATION_A: Self::FieldElement = EQUATION_A;
    const EQUATION_B: Self::FieldElement = EQUATION_B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = GENERATOR;
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp::ZERO, Fp::ONE);
}

impl<C: CurveParams> OnCurve<C> for BabyJubjub {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }
}
