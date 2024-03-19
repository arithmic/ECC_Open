use std::ops::Neg;

use bls381::{fp2::Fp2, fp::Fp, scalar::Scalar};
use crypto_bigint::{consts::U48, U256, Uint, Limb, U384, subtle::Choice};
use curve_traits::{Curve, CurveArithmetic, CurveParams, arithmeticops::EquationIsZero, OnCurve, affinepoint};

pub type G2AffinePoint = curve_traits::affinepoint::AffinePoint<G2BlsCurve>;
pub type G2ProjectivePoint = curve_traits::projectivepoint::ProjectivePoint<G2BlsCurve>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct G2BlsCurve ;
impl Curve for G2BlsCurve{
    type FieldBytesSize = U48;
    type Uint = U256;
    const ORDER: U256 =
        U256::from_be_hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");
}

/// impl CurveArithmetic trait for BLS381 curve
impl CurveArithmetic for G2BlsCurve{
    type AffinePoint = G2AffinePoint;
    type ProjectivePoint = G2ProjectivePoint;
    type Scalar =Scalar;
}


/// impl CurveParams trait for BLS381 curve
impl CurveParams for G2BlsCurve {
    type FieldElement = Fp2<Fp>;
    type PointArithmetic = EquationIsZero;
    const EQUATION_A: Self::FieldElement = Fp2::ZERO;

    const EQUATION_B: Self::FieldElement = Fp2 { 
        c0: Fp(Uint { limbs: [Limb(12260768510540316659), Limb(6038201419376623626), Limb(5156596810353639551), Limb(12813724723179037911), Limb(10288881524157229871), Limb(708830206584151678)] }), 
        c1: Fp(Uint { limbs: [Limb(12260768510540316659), Limb(6038201419376623626), Limb(5156596810353639551), Limb(12813724723179037911), Limb(10288881524157229871), Limb(708830206584151678)] }) };
    const GENERATOR: (Self::FieldElement, Self::FieldElement) = (Fp2 {
    
        c0: Fp(U384::from_be_hex("024AA2B2F08F0A91260805272DC51051C6E47AD4FA403B02B4510B647AE3D1770BAC0326A805BBEFD48056C8C121BDB8")).to_montgomery(),
        c1: Fp(U384::from_be_hex("13E02B6052719F607DACD3A088274F65596BD0D09920B61AB5DA61BBDC7F5049334CF11213945D57E5AC7D055D042B7E")).to_montgomery(), },
        
        Fp2{ c0: Fp(U384::from_be_hex("0CE5D527727D6E118CC9CDC6DA2E351AADFD9BAA8CBDD3A76D429A695160D12C923AC9CC3BACA289E193548608B82801")).to_montgomery(), 
        c1:  Fp(U384::from_be_hex("0606C4A02EA734CC32ACD2B02BC28B99CB3E287E85A763AF267492AB572E99AB3F370D275CEC1DA1AAA9075FF05F79BE")).to_montgomery() } 
        );
    const IDENTITY: (Self::FieldElement, Self::FieldElement) = (Fp2::ZERO, Fp2::ZERO);
}

impl<C: CurveParams> OnCurve<C> for G2BlsCurve {
    fn is_on_curve(point: curve_traits::AffinePoint<C>) -> bool {
        affinepoint::is_on_curve(point)
    }

    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement {
        affinepoint::compute_y::<C>(x, parity)
    }

  

}


// 1 / ((u+1) ^ (p-1)/3)
pub const PSI_COEFF_X : Fp2<Fp> = Fp2 :: new(Fp::ZERO, 
    Fp(U384::from_be_hex("1A0111EA397FE699EC02408663D4DE85AA0D857D89759AD4897D29650FB85F9B409427EB4F49FFFD8BFD00000000AAAD")).to_montgomery());

// 1 / ((u+1) ^ (p-1)/2)
pub const PSI_COEFF_Y : Fp2<Fp> = Fp2::new(
    Fp(U384::from_be_hex("135203E60180A68EE2E9C448D77A2CD91C3DEDD930B1CF60EF396489F61EB45E304466CF3E67FA0AF1EE7B04121BDEA2")).to_montgomery(),
    Fp(U384::from_be_hex("06AF0E0437FF400B6831E36D6BD17FFE48395DABC2D3435E77F76E17009241C5EE67992F72EC05F4C81084FBEDE3CC09")).to_montgomery());
 pub const DOUBLE_P_POWER_ENDOMORPHISM : Fp2<Fp> = Fp2::new(Fp(U384::from_be_hex("1A0111EA397FE699EC02408663D4DE85AA0D857D89759AD4897D29650FB85F9B409427EB4F49FFFD8BFD00000000AAAC")).to_montgomery(), Fp::ZERO);



impl G2BlsCurve{
    // Computation of psi(P)
    pub fn endomorphism( p :G2AffinePoint ) -> G2AffinePoint {
        // The p-power endomorphism for G2 is defined as follows:
        // 1. Note that G2 is defined on curve E': y^2 = x^3 + 4(u+1). To map a point
        // (x, y) in E' to (s, t) in E,    one set s = x / ((u+1) ^ (1/3)), t = y /
        // ((u+1) ^ (1/2)), because E: y^2 = x^3 + 4. 
        // 2. Apply the Frobenius
        // endomorphism (s, t) => (s', t'), another point on curve E,    where s' =
        // s^p, t' = t^p. 
        // 3. Map the point from E back to E'; that is,
        //    one set x' = s' * ((u+1) ^ (1/3)), y' = t' * ((u+1) ^ (1/2)).
        //
        // To sum up, it maps
        // (x,y) -> (x^p / ((u+1)^((p-1)/3)), y^p / ((u+1)^((p-1)/2)))
        // as implemented in the code as follows.
    
        let mut res = p;
        
        res.x.frobenius_map(1);
        res.y.frobenius_map(1);
    
        let tmp_x = res.x;
        res.x.c0 = -PSI_COEFF_X.c1 * tmp_x.c1;
        res.x.c1 = PSI_COEFF_X.c1 * tmp_x.c0;
        res.y *= PSI_COEFF_Y;
    
        res
        
    }
    
    /// For a p-power endomorphism psi(P), compute psi(psi(P))
    pub fn double_p_power_endomorphism(p: G2AffinePoint ) -> G2AffinePoint{
        let mut res = p;
        res.x *= DOUBLE_P_POWER_ENDOMORPHISM;
        res.y = res.y.neg();
        res
    
    }
       
    }