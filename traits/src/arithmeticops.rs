use crate::{affinepoint::AffinePoint, projectivepoint::ProjectivePoint, CurveParams};
use crypto_bigint::subtle::ConditionallySelectable;
use traits::traits::Field;

///point arithmetic trait mod sealed
pub mod sealed {
    use crate::{AffinePoint, CurveParams, ProjectivePoint};
    pub trait PointArithmetic<C: CurveParams> {
        /// Returns `lhs + rhs`
        fn add(lhs: &ProjectivePoint<C>, rhs: &ProjectivePoint<C>) -> ProjectivePoint<C>;
        /// Returns `lhs + rhs`
        fn add_mixed(lhs: &ProjectivePoint<C>, rhs: &AffinePoint<C>) -> ProjectivePoint<C>;
        /// Returns `point + point`
        fn double(point: &ProjectivePoint<C>) -> ProjectivePoint<C>;
    }
}

/// Allow crate-local visibility
pub use sealed::PointArithmetic;

/// Equation is different for different curves
/// depending on the equation the implementation is defined
//Equation is One
pub struct EquationIsONE {}
//equation is zero
pub struct EquationIsZero {}
//equation is generic
pub struct EquationIsGeneric {}

/// impl point arithmetic for equation is one
impl<C: CurveParams> PointArithmetic<C> for EquationIsONE {
    fn add(lhs: &ProjectivePoint<C>, other: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        let equation_3b = C::EQUATION_B * C::FieldElement::from(3 as u8);
        let l0 = lhs.x * other.x; // 1
        let l1 = lhs.y * other.y; // 2
        let l2 = lhs.z * other.z; // 3
        let l3 = ((lhs.x + lhs.y) * (other.x + other.y)) - (l0 + l1); // 4, 5, 6, 7, 8
        let l4 = ((lhs.x + lhs.z) * (other.x + other.z)) - (l0 + l2); // 9,10,11,12,13
        let l5 = ((lhs.y + lhs.z) * (other.y + other.z)) - (l1 + l2); // 14,15,16,17,18
        let z3 = (equation_3b * l2) + l4; //19,20,21
        let x3 = l1 - z3; //22
        let zz3 = l1 + z3; //23
        let ll1 = l0.double() + l0; // 25, 26
        let ll4 = (equation_3b * l4) + (l0 - l2); //27,29,30
        let ul1 = ll1 + l2; //28
        let ul0 = ul1 * ll4; //28,31

        ProjectivePoint {
            x: (l3 * x3) - (l5 * ll4), //33,34,35
            y: (x3 * zz3) + ul0,       // 24,32
            z: (l5 * zz3) + (l3 * ul1),
        }
    }
    ///[Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
    /// the above algorithm is being used here
    fn add_mixed(lhs: &ProjectivePoint<C>, other: &AffinePoint<C>) -> ProjectivePoint<C> {
        let equation_3b = C::EQUATION_B * C::FieldElement::from(3 as u8);
        let t0 = lhs.x * other.x; //1
        let t1 = lhs.y * other.y; //2
        let t3 = ((other.x + other.y) * (lhs.x + lhs.y)) - (t0 + t1); // 3,4,5,6,7
        let t4 = (other.x * lhs.z) + lhs.x; // 8,9
        let t5 = (other.y * lhs.z) + lhs.y; //10,11
        let z3 = (equation_3b * lhs.z) + t4; // 12,13,14
        let x3 = t1 - z3; //15
        let zz3 = t1 + z3; //16
        let y3 = x3 * zz3; //17
        let tt1 = (t0.double() + t0) + lhs.z; //18,19,20,22
        let tt2 = t0 - lhs.z; //23,24
        let tt4 = (equation_3b * t4) + tt2; //21,25
        let tt0 = tt1 * tt4; //26
        let y = y3 + tt0; //27
        let x = (t3 * x3) - (t5 * tt4); // 28,29,30
        let z = (t5 * zz3) + (t3 * tt1); //31,32,33
        let mut result = ProjectivePoint { x, y, z };
        result.conditional_assign(lhs, other.is_identity());
        result
    }
    /// double function
    fn double(point: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        let equation_3b = C::EQUATION_B * C::FieldElement::from(3 as u8);

        let l0 = point.x.square(); // 1
        let l1 = point.y.square(); // 2
        let l2 = point.z.square(); // 3
        let l3 = (point.x * point.y).double(); // 4, 5
        let z3 = (point.x * point.z).double(); // 6, 7
        let y3 = z3 + (equation_3b * l2); // 8,9,10
        let x3 = l1 - y3; //11
        let yy3 = l1 + y3; //12
        let y_frag = x3 * yy3; // 13
        let x_frag = x3 * l3; // 14
        let ll3 = (l0 - l2) + (equation_3b * z3); // 15,16,17
        let ll0 = l0.double() + l0 + l2; // 18,19,20

        let y = y_frag + (ll0 * ll3); // 21,22
        let ll2 = (point.y * point.z).double(); //23,24
        let x = x_frag - (ll2 * ll3); //25,26
        let m = (ll2 * l1).double();
        let z = m.double();
        ProjectivePoint { x, y, z }
    }
}

///
/// point arithemtic for equation is generic
/// referred to [Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
/// algorithm 1
impl<C: CurveParams> PointArithmetic<C> for EquationIsGeneric {
    fn add(lhs: &ProjectivePoint<C>, rhs: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        // 3 * b
        let equation_3b = C::FieldElement::from(3 as u8) * C::EQUATION_B;
        //
        let g0 = lhs.x * rhs.x; // 1
        let g1 = lhs.y * rhs.y; // 2
        let g2 = lhs.z * rhs.z; // 3
        let g3 = lhs.x + lhs.y; // 4
        let g4 = rhs.x + rhs.y; // 5
        let g3 = g3 * g4; // 6
        let g4 = g0 + g1; // 7
        let g3 = g3 - g4; // 8
        let g4 = lhs.x + lhs.z; // 9
        let g5 = rhs.x + rhs.z; // 10
        let g4 = g4 * g5; // 11
        let g5 = g0 + g2; // 12
        let g4 = g4 - g5; // 13
        let g5 = lhs.y + lhs.z; // 14
        let x3 = rhs.y + rhs.z; // 15
        let g5 = g5 * x3; // 16
        let x3 = g1 + g2; // 17
        let g5 = g5 - x3; // 18
        let z3 = C::EQUATION_A * g4; // 19
        let x3 = equation_3b * g2; // 20
        let z3 = x3 + z3; // 21
        let x3 = g1 - z3; // 22
        let z3 = g1 + z3; // 23
        let y3 = x3 * z3; // 24
        let g1 = g0.double(); // 25
        let g1 = g1 + g0; // 26
        let g2 = C::EQUATION_A * g2; // 27
        let g4 = equation_3b * g4; // 28
        let g1 = g1 + g2; // 29
        let g2 = g0 - g2; // 30
        let g2 = C::EQUATION_A * g2; // 31
        let g4 = g4 + g2; // 32
        let g0 = g1 * g4; // 33
        let y3 = y3 + g0; // 34
        let g0 = g5 * g4; // 35
        let x3 = g3 * x3; // 36
        let x3 = x3 - g0; // 37
        let g0 = g3 * g1; // 38
        let z3 = g5 * z3; // 39
        let z3 = z3 + g0; // 40

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }
    ///
    /// used algorithm 2 implemented here from the same paper
    fn add_mixed(lhs: &ProjectivePoint<C>, rhs: &AffinePoint<C>) -> ProjectivePoint<C> {
        let b3 = C::EQUATION_B * C::FieldElement::from(3 as u8);
        let g0 = lhs.x * rhs.x; // 1
        let g1 = lhs.y * rhs.y; // 2
        let g3 = rhs.x + rhs.y; // 3
        let g4 = lhs.x + lhs.y; // 4
        let g3 = g3 * g4; // 5
        let g4 = g0 + g1; // 6
        let g3 = g3 - g4; // 7
        let g4 = rhs.x * lhs.z; // 8
        let g4 = g4 + lhs.x; // 9
        let g5 = rhs.y * lhs.z; // 10
        let g5 = g5 + lhs.y; // 11
        let z3 = C::EQUATION_A * g4; // 12
        let x3 = b3 * lhs.z; // 13
        let z3 = x3 + z3; // 14
        let x3 = g1 - z3; // 15
        let z3 = g1 + z3; // 16
        let y3 = x3 * z3; // 17
        let g1 = g0.double(); // 18
        let g1 = g1 + g0; // 19
        let g2 = C::EQUATION_A * lhs.z; // 20
        let g4 = b3 * g4; // 21
        let g1 = g1 + g2; // 22
        let g2 = g0 - g2; // 23
        let g2 = C::EQUATION_A * g2; // 24
        let g4 = g4 + g2; // 25
        let g0 = g1 * g4; // 26
        let y3 = y3 + g0; // 27
        let g0 = g5 * g4; // 28
        let x3 = g3 * x3; // 29
        let x3 = x3 - g0; // 30
        let g0 = g3 * g1; // 31
        let z3 = g5 * z3; // 32
        let z3 = z3 + g0; // 33

        let mut ret = ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        };
        ret.conditional_assign(lhs, rhs.is_identity());
        ret
    }

    fn double(point: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        let b3 = C::EQUATION_B * C::FieldElement::from(3 as u8);
        let g0 = point.x.square(); //1
        let g1 = point.y.square(); //2
        let g2 = point.z.square(); //3
        let g3 = point.x * point.y; //4
        let g3 = g3.double(); //5
        let z3 = point.x * point.z; //6
        let z3 = z3.double(); //7
        let x3 = C::EQUATION_A * z3; //8
        let y3 = b3 * g2; //9
        let y3 = x3 + y3; // 10
        let x3 = g1 - y3; //11
        let y3 = g1 + y3; //12
        let y3 = x3 * y3; //13
        let x3 = g3 * x3; //14
        let z3 = b3 * z3; // 15
        let g2 = C::EQUATION_A * g2; //16
        let g3 = g0 - g2; //17
        let g3 = C::EQUATION_A * g3; //18
        let g3 = g3 + z3; //19
        let z3 = g0.double(); //20
        let g0 = z3 + g0; //21
        let g0 = g0 + g2; //22
        let g0 = g0 * g3; //23
        let y3 = y3 + g0; //24
        let g2 = point.y * point.z; //25
        let g2 = g2.double(); //26
        let g0 = g2 * g3; //27
        let x3 = x3 - g0; //28
        let z3 = g2 * g1; //29
        let z3 = z3.double().double(); //30, 31

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

/// for equation zero
/// impl for equation is zero i.e y^2 = x^3 + b
///[Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
/// the above algorithm is being used here
impl<C: CurveParams> PointArithmetic<C> for EquationIsZero {
    fn add(lhs: &ProjectivePoint<C>, rhs: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        // 3 * b
        let equation_3b = C::FieldElement::from([3, 0, 0, 0, 0, 0]) * C::EQUATION_B;
        let g0 = lhs.x * rhs.x; // 1
        let g1 = lhs.y * rhs.y; // 2
        let g2 = lhs.z * rhs.z; // 3
        let g3 = lhs.x + lhs.y; // 4

        let g4 = rhs.x + rhs.y; // 5
        let g3 = g3 * g4; // 6

        let g4 = g0 + g1; // 7

        let g3 = g3 - g4; // 8

        let g4 = lhs.y + lhs.z; // 9

        let x3 = rhs.y + rhs.z; // 10

        let g4 = g4 * x3; //11

        let x3 = g1 + g2; // 12

        let g4 = g4 - x3; //13

        let x3 = lhs.x + lhs.z; // 14

        let y3 = rhs.x + rhs.z; // 15

        let x3 = x3 * y3; //16

        let y3 = g0 + g2; //17

        let y3 = x3 - y3; //18

        let x3 = g0.double(); // 19

        let g0 = x3 + g0; // 20

        let g2 = equation_3b * g2; //21

        let z3 = g1 + g2; //22
        let g1 = g1 - g2; // 2
        let y3 = equation_3b * y3; // 24
        let x3 = g4 * y3; //25

        let g2 = g3 * g1; //26
        let x3 = g2 - x3; //27

        let y3 = y3 * g0; //28
        let g1 = g1 * z3; //29
        let y3 = g1 + y3; //30
        let g0 = g0 * g3; //31
        let z3 = z3 * g4; //32
        let z3 = z3 + g0; //33

        // result
        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    ///
    ///
    ///
    /// Algorithm 8 is being implemented here
    fn add_mixed(lhs: &ProjectivePoint<C>, rhs: &AffinePoint<C>) -> ProjectivePoint<C> {
        // 3b
        let equation_3b = C::FieldElement::from([3, 0, 0, 0, 0, 0]) * C::EQUATION_B;
        //
        let g0 = lhs.x * rhs.x; // 1
        let g1 = lhs.y * rhs.y; // 2
        let g3 = rhs.x + rhs.y; //3
        let g4 = lhs.x + lhs.y; //4
        let g3 = g3 * g4; //5
        let g4 = g0 + g1; //6
        let g3 = g3 - g4; //7
        let g4 = rhs.y * lhs.z; //8
        let g4 = g4 + lhs.y; //9
        let y3 = rhs.x * lhs.z; //10
        let y3 = y3 + lhs.x; // 11
        let x3 = g0.double(); //12

        let g0 = x3 + g0; //13
        let g2 = equation_3b * lhs.z; //14
        let z3 = g1 + g2; // 15
        let g1 = g1 - g2; //16
        let y3 = equation_3b * y3; //17
        let x3 = g4 * y3; //18
        let g2 = g3 * g1; //19
        let x3 = g2 - x3; //20
        let y3 = y3 * g0; //21
        let g1 = g1 * z3; //22
        let y3 = g1 + y3; //23
        let g0 = g0 * g3; //24
        let z3 = z3 * g4; //25
        let z3 = z3 + g0; //26
                          // result
        let mut point = ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        };
        point.conditional_assign(lhs, rhs.is_identity());
        point
    }

    fn double(point: &ProjectivePoint<C>) -> ProjectivePoint<C> {
        let equation_3b = C::EQUATION_B * C::FieldElement::from([3, 0, 0, 0, 0, 0]);

        let g0 = point.y.square(); //1
        let z3 = g0.double(); //2
        let z3 = (z3.double()).double(); //3,4
        let g1 = point.y * point.z; //5
        let g2 = point.z.square(); //6
        let g2 = equation_3b * g2; //7
        let x3 = g2 * z3; // 8
        let y3 = g0 + g2; // 9
        let z3 = g1 * z3; //10
        let g1 = g2.double(); //11
        let g2 = g1 + g2; //12
        let g0 = g0 - g2; // 13
        let y3 = g0 * y3; //14
        let y3 = x3 + y3; // 15
        let g1 = point.x * point.y; //16
        let x3 = g0 * g1; //17
        let x3 = x3.double(); // 18

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}
