use crate::{AffineCoordinates, CurveParams, FieldBytes, ProjectivePoint};
use crypto_bigint::{
    generic_array::GenericArray,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq},
};
use std::ops::{Add, AddAssign};
use traits::traits::{Field, PrimeField};

/// Point on a Weierstrass curve in affine coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AffinePoint<C: CurveParams> {
    /// x-coordinate
    pub x: C::FieldElement,
    /// y-coordinate
    pub y: C::FieldElement,
    /// Is this point the point at infinity? 0 = no, 1 = yes
    /// This is a proxy for [`Choice`], but uses `u8` instead to permit `const`
    /// constructors for `IDENTITY` and `GENERATOR`.
    pub infinity: u8,
}

impl<C> AffinePoint<C>
where
    C: CurveParams,
{
    /// Additive identity of the group a.k.a. the point at infinity. (0,0,1)
    pub const IDENTITY: Self = Self {
        x: C::IDENTITY.0,
        y: C::IDENTITY.1,
        infinity: 1,
    };

    /// Base point of the curve.
    pub const GENERATOR: Self = Self {
        x: C::GENERATOR.0,
        y: C::GENERATOR.1,
        infinity: 0,
    };

    //Is this point the point at infinity?
    pub fn is_identity(&self) -> Choice {
        Choice::from(self.infinity)
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
  
  
    // generate a random value from field element for affine coordinate
    // return an (x,y) coordinate in affine point structure
    // pub fn new() -> Self{
    //     let x: FieldElement = Field::random();
    //     // let y = Field::random();
    //     // if y == C::IDENTITY.1 && x == C::IDENTITY.0 {
    //     //     Self::IDENTITY
    //     // }
    //     // else{
    //     //     // form the structure of the point in form of affine point
    //     //     let point = Self { x: x, y: y, infinity: 0 };
    //     //     // check if the points lie on the curve or not
    //     //     let truth = OnCurve::is_on_curve(point);
    //     //     //
    //     //     // based on the truth value return the point else compute y for which (x,y) lie on curve
    //     //     if truth{
    //     //         point
    //     //     }
    //     //     else{
    //     //         let parity = Choice::from(1u8); // check for this value
    //     //         Self{x: x, y: OnCurve::compute_y(x, parity), infinity:0} // convert to compute y
    //     //     }
    //     // }
    //     let parity = Choice::from(1u8);
    //     let y = compute_y(x, parity);
    //     y
    // }

    pub fn to_projective(&self) -> ProjectivePoint<C> {
        let projective = ProjectivePoint {
            x: self.x,
            y: self.y,
            z: C::FieldElement::ONE,
        };
        ProjectivePoint::conditional_select(
            &projective,
            &ProjectivePoint::IDENTITY,
            self.is_identity(),
        )
    }
}

// trait through imlplementation
// not in impl
/// computes y for the given x Field Element
pub fn compute_y<C: CurveParams>(x: C::FieldElement, parity: Choice) -> C::FieldElement {
    let alpha = x * (x.square() + C::EQUATION_A) + C::EQUATION_B;
    let beta = Field::sqrt(alpha).unwrap();
    let y = C::FieldElement::conditional_select(&-beta, &beta, beta.is_odd().ct_eq(&parity));
    y
}
/// checks for the point on the curve
/// should return true else point not on curve
pub fn is_on_curve<C: CurveParams>(point: AffinePoint<C>) -> bool {
    bool::from((point.y.square() - point.x * (C::EQUATION_A + point.x.square())).ct_eq(&C::EQUATION_B))
}

/// impl coordinates trait for affine point
impl<C: CurveParams> AffineCoordinates for AffinePoint<C> {
    type FieldRepr = FieldBytes<C>; // how to access type alias

    // /// converts x coordinate to byte representation
    fn x(&self) -> Self::FieldRepr {
        GenericArray::clone_from_slice(self.x.to_curve_bytes()) // still there
                                                                //self.x.to_curve_bytes()
    }
    // y_is_odd
    // checks if the y coordinate of affine point is odd or not
    fn y_is_odd(&self) -> Choice {
        self.y.is_odd()
    }
}

/// function impl on affine point
impl<C: CurveParams> AddAssign<AffinePoint<C>> for ProjectivePoint<C> {
    fn add_assign(&mut self, rhs: AffinePoint<C>) {
        *self = ProjectivePoint::add_mixed(self, &rhs);
    }
}
// referenced operator
impl<C: CurveParams> AddAssign<&AffinePoint<C>> for ProjectivePoint<C> {
    fn add_assign(&mut self, rhs: &AffinePoint<C>) {
        *self = ProjectivePoint::add_mixed(self, rhs);
    }
}

// impl add over projective pointx
impl<C: CurveParams> Add<AffinePoint<C>> for &ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;
    fn add(self, other: AffinePoint<C>) -> ProjectivePoint<C> {
        ProjectivePoint::add_mixed(&self, &other)
    }
}

// impl from projective point
impl<C: CurveParams> From<ProjectivePoint<C>> for AffinePoint<C> {
    fn from(value: ProjectivePoint<C>) -> Self {
        value.to_affine()
    }
}

impl<C> ConditionallySelectable for AffinePoint<C>
where
    C: CurveParams,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            x: C::FieldElement::conditional_select(&a.x, &b.x, choice),
            y: C::FieldElement::conditional_select(&a.y, &b.y, choice),
            infinity: u8::conditional_select(&a.infinity, &b.infinity, choice),
        }
    }
}
