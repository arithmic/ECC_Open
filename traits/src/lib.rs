// imports
pub use affinepoint::AffinePoint;
pub use arithmeticops::PointArithmetic;
use crypto_bigint::{
    generic_array::{ArrayLength, GenericArray},
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq},
};
pub use projectivepoint::ProjectivePoint;
use std::marker::Copy;
use traits::traits::{PrimeField, Field};
pub mod affinepoint;
pub mod arithmeticops;
pub mod projectivepoint;

//Size of serialized field element in bytes similar to FieldByteSize in Curve
pub type FieldBytesSize<C> = <C as Curve>::FieldBytesSize;

//Byte representation of a base/scalar field element of the curve
pub type FieldBytes<C> = GenericArray<u8, FieldBytesSize<C>>;

//===curve trait
pub trait Curve: 'static + Copy + Clone + Default + Send + Sync + PartialEq + Eq {
    ///field(base field) bytes
    type FieldBytesSize: ArrayLength<u8>;
    /// Integer type used to represent field elements of this elliptic curve.
    type Uint; //scalar or
    /// Order of this elliptic curve, i.e. number of elements in the scalar
    /// field.
    const ORDER: Self::Uint;
}
//===curve params trait
pub trait CurveParams:
    Curve
    + CurveArithmetic
    + CurveArithmetic<AffinePoint = AffinePoint<Self>>
    + CurveArithmetic<ProjectivePoint = ProjectivePoint<Self>>
    + OnCurve<Self>
{
    /// Base field element type.
    type FieldElement: PrimeField + ConditionallySelectable + ConstantTimeEq;
    type PointArithmetic: PointArithmetic<Self>;
    const EQUATION_A: Self::FieldElement; // A
    const EQUATION_B: Self::FieldElement; // B
    const GENERATOR: (Self::FieldElement, Self::FieldElement); // Generator
    const IDENTITY: (Self::FieldElement, Self::FieldElement);
}

//===Cruve arithmetic trait
pub trait CurveArithmetic: Curve {
    /// Elliptic curve point in affine coordinates.
    type AffinePoint: 'static + Copy + Sized + Send + Sync;
    /// Elliptic curve point in projective coordinates.
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`Debug`]
    /// - [`Eq`]
    /// - [`Sized`]
    /// - [`Send`]
    /// - [`Sync`]
    type ProjectivePoint: Copy;
    /// Scalar field modulo this curve's order.
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`ConditionallySelectable`]
    /// - [`ConstantTimeEq`]
    /// - [`Debug`]
    /// - [`Default`]
    /// - [`Send`]
    /// - [`Sync`]
    type Scalar: Copy + PrimeField + Field;
    // + PrimeField
    // + PrimeField<Repr = FieldBytes>;
}

//affine coordinate
pub trait AffineCoordinates {
    type FieldRepr: AsRef<[u8]>;
    fn x(&self) -> Self::FieldRepr;
    fn y_is_odd(&self) -> Choice;
}

// trait for point y
pub trait OnCurve<C: CurveParams> {
    fn is_on_curve(point: AffinePoint<C>) -> bool;
    /// compute y for the given x value, to lie on curve
    fn compute_y(x: C::FieldElement, parity: Choice) -> C::FieldElement;
}

// double trait
pub trait Double {
    type Output;
    fn double(self) -> Self::Output;
}