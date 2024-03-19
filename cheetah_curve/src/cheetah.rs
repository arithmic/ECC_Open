///Imports
use cheetah64::{fp::Fp, fp6::Fp6, scalar::Scalar};
use crypto_bigint::{U256, consts::U48};
use curve_traits::{CurveParams, CurveArithmetic, Curve, arithmeticops::EquationIsONE, OnCurve, affinepoint};

// const B = 395 + u
pub const B: Fp6<Fp> = Fp6 {
    c0: Fp(395),
    c1: Fp::one(),
    c2: Fp::zero(),
    c3: Fp::zero(),
    c4: Fp::zero(),
    c5: Fp::zero(),
};

//B3= 3*B  
pub const B3: Fp6<Fp> = Fp6 {
    c0: Fp(1185),
    c1: Fp(3),
    c2: Fp::zero(),
    c3: Fp::zero(),
    c4: Fp::zero(),
    c5: Fp::zero(),
};

///Elliptic curve struct
/// having equation y^2 = x^3 + x + (395 + u)
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cheetah64;

// specifying the path for Affine Point and Projective Point
pub type AffinePoint = curve_traits::affinepoint::AffinePoint<Cheetah64>;
pub type ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<Cheetah64>;

/// impl curve, curve params, curve arithmetic on Cheetah curve
/// impl Curve trait for Cheetah curve
impl Curve for Cheetah64{
    type FieldBytesSize= U48;

    type Uint = U256;

    const ORDER: U256 = U256::from_be_hex("0x7AF2599B3B3F22D0563FBF0F990A37B5327AA72330157722D443623EAED4ACCF");
}

/// impl curve arithmetic trait on Cheetah curve
impl CurveArithmetic for Cheetah64{
    type AffinePoint= AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar= Scalar;
}

///impl CurveParams for for Cheetah curve
impl CurveParams for Cheetah64{
    type FieldElement= Fp6<Fp>;
    type PointArithmetic = EquationIsONE;
    const EQUATION_A: Self::FieldElement= Fp6::one();
    const EQUATION_B: Self::FieldElement= B;
    const GENERATOR: (Self::FieldElement, Self::FieldElement)= (Fp6 {
        c0: Fp(0x263a588f4b0118a1),
        c1: Fp(0x7757a0bcb26a142d),
        c2: Fp(0x9215adfc1e925890),
        c3: Fp(0x430aad2ce14759a4),
        c4: Fp(0x534ece54de4b2c8),
        c5: Fp(0xb39050f01f7b1f33),
    },
    
    Fp6 {
        c0: Fp(0xd57f0d0d47482534),
        c1: Fp(0x26821d894fa8ea0f),
        c2: Fp(0xc77f564783ef13a1),
        c3: Fp(0x949c360784284ec2),
        c4: Fp(0xb7040bd639ef3cc4),
        c5: Fp(0x8aa635f2719d255f),
    });

    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp6::zero(), Fp6::one());

}

impl <C:CurveParams> OnCurve<C> for Cheetah64{
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve::<C>(point)
    }

    fn compute_y(x: <C as CurveParams>::FieldElement, parity: crypto_bigint::subtle::Choice) -> <C as CurveParams>::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }
}
