use crate::arithmeticops::sealed::PointArithmetic;
use crate::{affinepoint, CurveArithmetic, Double};
use crate::{affinepoint::AffinePoint, CurveParams};
use crypto_bigint::subtle::{Choice, ConditionallySelectable};
use crypto_bigint::U256;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use table::{self, Table};
use traits::traits::Field;
//pub scalar
pub type Scalar<C> = <C as CurveArithmetic>::Scalar;

/// Point on a Weierstrass curve in projective coordinates.
/// struct definition
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectivePoint<C: CurveParams> {
    pub x: C::FieldElement,
    pub y: C::FieldElement,
    pub z: C::FieldElement,
}
/// impl of Projective Point
impl<C: CurveParams> ProjectivePoint<C> {
    /// Additive identity of the group a.k.a. the point at infinity.
    pub const IDENTITY: Self = Self {
        x: C::FieldElement::ZERO,
        y: C::FieldElement::ONE,
        z: C::FieldElement::ZERO,
    };

    /// Base point of the curve.
    pub const GENERATOR: Self = Self {
        x: C::GENERATOR.0,
        y: C::GENERATOR.1,
        z: C::FieldElement::ONE,
    };

    // Returns the affine representation of this point, or `None` if it is the identity.
    pub fn to_affine(&self) -> AffinePoint<C> {
        self.z
            .invert()
            .map(|zinv| AffinePoint {
                x: self.x * zinv,
                y: self.y * zinv,
                infinity: 0,
            })
            .unwrap_or(AffinePoint::IDENTITY)
    }

    /// Returns `-self`.
    pub fn neg(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }

    pub fn is_identity(&self) -> bool {
        (self.x == C::FieldElement::ZERO) && (self.z == C::FieldElement::ZERO)
    }

    /// Returns `self + other`.
    pub fn add(&self, other: &Self) -> Self {
        C::PointArithmetic::add(self, other)
    }

    // /// Returns `self + other`.
    pub fn add_mixed(&self, other: &AffinePoint<C>) -> Self {
        C::PointArithmetic::add_mixed(self, other)
    }

    /// Returns `self - other`.
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// returns double of the point
    pub fn double(&self) -> Self {
        C::PointArithmetic::double(self)
    }

    /// returns 'self * scalar'
    pub fn mul(self, other: Scalar<C>) -> Self {
        let exp = other.to_words();
        let mut a = Self::IDENTITY;
        for e in exp.iter().rev() {
            for i in (0..64).rev() {
                a = a.double(); // curve double
                if ((*e >> i) & 1) == 1 {
                    a = a.add(self) // curve add function
                }
            }
        }
        a
    }

    /// returns 'self * field'
    pub fn mul_field(self, other: C::FieldElement) -> Self {
        let exp = other.to_words();
        let mut a = Self::IDENTITY;
        for e in exp.iter().rev() {
            for i in (0..64).rev() {
                a = a.double(); // curve double
                if ((*e >> i) & 1) == 1 {
                    a = a.add(self) // curve add function
                }
            }
        }
        a
    }

    // returns 'self * scalar'
    // powers_vec stores the precomputed value like g, g^2,g^4,g^8......
    pub fn exp(powers_vec: &Vec<ProjectivePoint<C>>, other: Scalar<C>) -> ProjectivePoint<C> {
        let mut exp_bits = vec![0; 256];
        for idx in 0..256 {
            let a = other.to_words();
            exp_bits[idx] = U256::from_words([a[0], a[1], a[2], a[3]]).bit(idx);
        }
        let mut r = ProjectivePoint::IDENTITY;
        if exp_bits[0] == 0 {
            r = r;
        } else {
            r = powers_vec[0];
        }
        for outer_idx in 1..exp_bits.len() {
            if exp_bits[outer_idx] == 1 {
                r = r.add(&powers_vec[outer_idx]);
            }
        }
        r
    }

    // returns 'self * field element'
    // powers_vec stores the precomputed value like g, g^2,g^4,g^8......
    pub fn exp_field(
        powers_vec: &Vec<ProjectivePoint<C>>,
        other: C::FieldElement,
    ) -> ProjectivePoint<C> {
        let mut exp_bits = vec![0; 256];
        for idx in 0..256 {
            let a = other.to_words();
            exp_bits[idx] = U256::from_words([a[0], a[1], a[2], a[3]]).bit(idx);
        }
        let mut r = ProjectivePoint::IDENTITY;
        if exp_bits[0] == 0 {
            r = r;
        } else {
            r = powers_vec[0];
        }
        for outer_idx in 1..exp_bits.len() {
            if exp_bits[outer_idx] == 1 {
                r = r.add(&powers_vec[outer_idx]);
            }
        }
        r
    }

    ///....................
    /// .......................
    /// ............................
    /// Shamir's trick
    // pub fn multi_exponentiation(point: Vec<AffinePoint<C>>, exponent: Vec<Scalar<C>>) -> Self {
    //     // Convert Vec<Scalar> into Vec<[u64; fieldbits/64]>
    //     let mut exp = Vec::new();
    //     for value in exponent {
    //         exp.push(value.to_words())
    //     }
    //     let mut result = Self::IDENTITY;
    //     for outer_idx in (0..exp[0].len()).rev() {
    //         for inner_idx in (0..64).rev() {
    //             result = result.double();
    //             //println!("{}",affinepoint::is_on_curve(result.to_affine()));
    //             for (idx, value) in exp.iter().enumerate() {
    //                 if ((value[outer_idx] >> inner_idx) & 1) == 1 {
    //                     //convert the part to projective point
    //                     //println!("{}",affinepoint::is_on_curve(point[idx]));
    //                     result = result + point[idx].to_projective();
    //                     //println!("{}",affinepoint::is_on_curve(result.to_affine()));
    //                 }
    //             }
    //         }
    //     }
    //     result
    // }
    ///.................
    /// ................Pippenger MSM...............
    /// .......................
    pub fn pippenger_msm(
        bases: &[ProjectivePoint<C>],
        exponents: &[Scalar<C>],
    ) -> ProjectivePoint<C> {
        let bits = bases.len().next_power_of_two().trailing_zeros();
        assert_eq!(
            bases.len(),
            exponents.len(),
            "bases and exponents are of different length"
        );
        let window_bits: usize = match bits {
            0..=5 => 4,
            6..=9 => 5,
            10 => 6,
            11 | 12 => 8,
            13 | 14 => 10,
            15 => 11,
            16..=18 => 12,
            19 | 20 => 16,
            21..=u32::MAX => 17,
        };

        //Stores the byte expression of all the exponents.
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = (256f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .par_iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }
    pub fn pippenger_msm_field(
        bases: &[ProjectivePoint<C>],
        exponents: &[C::FieldElement],
    ) -> ProjectivePoint<C> {
        let bits = bases.len().next_power_of_two().trailing_zeros();
        assert_eq!(
            bases.len(),
            exponents.len(),
            "bases and exponents are of different length"
        );
        let window_bits: usize = match bits {
            0..=5 => 4,
            6..=9 => 5,
            10 => 6,
            11 | 12 => 8,
            13 | 14 => 10,
            15 => 11,
            16..=18 => 12,
            19 | 20 => 16,
            21..=u32::MAX => 17,
        };

        //Stores the byte expression of all the exponents.
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = (256f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .par_iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }

    pub fn pippenger_seq(
        bases: &[ProjectivePoint<C>],
        exponents: &[Scalar<C>],
        max_bits: usize,
    ) -> ProjectivePoint<C> {
        let bits = bases.len().next_power_of_two().trailing_zeros();
        assert_eq!(
            bases.len(),
            exponents.len(),
            "bases and exponents are of different length"
        );
        let window_bits: usize = match bits {
            0..=5 => 4,
            6..=9 => 5,
            10 => 6,
            11 | 12 => 8,
            13 | 14 => 10,
            15 => 11,
            16..=18 => 12,
            19 | 20 => 16,
            21..=u32::MAX => 17,
        };

        //Stores the byte expression of all the exponents.
        let windows: Vec<Vec<usize>> = exponents
            .iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = ((max_bits as f32) / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    if windows[i][n_window] != 0 {
                        buckets[windows[i][n_window] as usize] += bases[i];
                    }
                }
            });

        S.iter_mut()
            .zip(window_sum.iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }

    pub fn affine_pippenger(bases: &[AffinePoint<C>], exponents: &[Scalar<C>]) -> AffinePoint<C> {
        let bits = bases.len().next_power_of_two().trailing_zeros();

        let window_bits: usize = match bits {
            0..=5 => 4,
            6..=9 => 5,
            10 => 7,
            11 | 12 => 8,
            13 | 14 => 10,
            15 => 12,
            16..=18 => 13,
            19 | 20 => 16,
            21..=u32::MAX => 17,
        };

        //Stores the byte expression of all the exponents.
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = (256f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::<C>::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::<C>::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::<C>::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum.to_affine()
    }

    pub fn pippenger_onebyte(
        bases: &[ProjectivePoint<C>],
        exponents: &[Scalar<C>],
    ) -> ProjectivePoint<C> {
        let window_bits = 10;
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = (64f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .par_iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }

    pub fn pippenger_onelimb(
        bases: &[ProjectivePoint<C>],
        exponents: &[Scalar<C>],
    ) -> ProjectivePoint<C> {
        let window_bits = 10;
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = (64f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .par_iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }

    pub fn pippenger_nbytes(
        bases: &[ProjectivePoint<C>],
        exponents: &[Scalar<C>],
        bytes: usize,
    ) -> ProjectivePoint<C> {
        //TODO: window size to be set with a match to bit length of input size.
        let window_bits = 10;
        let windows: Vec<Vec<usize>> = exponents
            .par_iter()
            .map(|e| e.get_windows(window_bits))
            .collect();

        let n_windows = ((bytes * 8) as f32 / (window_bits as f32)).ceil() as usize;

        let mut sum = ProjectivePoint::IDENTITY;

        let mut window_sum = vec![ProjectivePoint::IDENTITY; n_windows];
        let mut S = vec![ProjectivePoint::IDENTITY; n_windows];

        let mut buckets: Vec<Vec<ProjectivePoint<C>>> =
            vec![vec![ProjectivePoint::IDENTITY; 1 << window_bits]; n_windows];

        //Sorting bases into buckets, parallelised over the windows.
        buckets
            .par_iter_mut()
            .enumerate()
            .for_each(|(n_window, buckets)| {
                for i in 0..bases.len() {
                    buckets[windows[i][n_window] as usize] += bases[i];
                }
            });

        S.par_iter_mut()
            .zip(window_sum.par_iter_mut())
            .enumerate()
            .for_each(|(k, (S_k, W_k))| {
                for i in (1..1 << window_bits).rev() {
                    *S_k += buckets[k][i];
                    *W_k += *S_k
                }
            });

        for k in (0..n_windows).rev() {
            sum = window_sum[k] + exp_by_2(sum, window_bits)
        }

        sum
    }

    pub fn mult_out_pippenger(
        bases: &[ProjectivePoint<C>],
        exponents: &Table<Scalar<C>>,
        max_bits: usize,
    ) -> Vec<ProjectivePoint<C>> {
        (0..exponents.num_of_columns())
            .into_par_iter()
            .map(|column_idx| {
                Self::pippenger_seq(bases, &exponents.column_at(column_idx), max_bits)
            })
            .collect()
    }

    //Exponentiates self by exp(Scalar) and stores the intermediate values used in trace_table
    pub fn mul_verification_scalar(
        p: C::ProjectivePoint,
        exp: &C::Scalar,
    ) -> (C::ProjectivePoint, Vec<C::ProjectivePoint>) {
        let mut computation = Vec::new();
        let mut exp_bits = Vec::new();
        for idx in 0..256 {
            let a = exp.to_words();
            exp_bits.push(U256::from_words([a[0], a[1], a[2], a[3]]).bit(idx));
        }
        let mut r = C::ProjectivePoint::IDENTITY;
        let mut a = p;
        if exp_bits[0] == 0 {
            r = r;
        } else {
            r = a;
        }
        computation.push(r);
        a = a.add(&a);
        for outer_idx in 1..exp_bits.len() {
            if exp_bits[outer_idx] == 1 {
                r = r.add(&a);
            }
            computation.push(r);
            a = a.add(&a);
        }
        (r, computation)
    }

    // Exponentiates self by exp(FieldElement) and stores the intermediate values used in trace_table
    pub fn mul_verification_field(
        p: C::ProjectivePoint,
        exp: &C::FieldElement,
    ) -> (C::ProjectivePoint, Vec<C::ProjectivePoint>) {
        let mut computation = Vec::new();
        let mut exp_bits = Vec::new();
        for idx in 0..256 {
            let a = exp.to_words();
            exp_bits.push(U256::from_words([a[0], a[1], a[2], a[3]]).bit(idx));
        }
        let mut r = C::ProjectivePoint::IDENTITY;
        let mut a = p;
        if exp_bits[0] == 0 {
            r = r;
        } else {
            r = a;
        }
        computation.push(r);
        a = a.add(&a);
        for outer_idx in 1..exp_bits.len() {
            if exp_bits[outer_idx] == 1 {
                r = r.add(&a);
            }
            computation.push(r);
            a = a.add(&a);
        }
        (r, computation)
    }

    pub fn from_string(str: &str) -> Self {
        let mut split = str.split(" ");
        let x = C::FieldElement::from(U256::from_be_hex(split.next().unwrap()));
        let y = C::FieldElement::from(U256::from_be_hex(split.next().unwrap()));
        let z = C::FieldElement::from(U256::from_be_hex(split.next().unwrap()));
        ProjectivePoint { x, y, z }
    }
}

impl<C: CurveParams> Display for ProjectivePoint<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z,)
    }
}

// conditionally selectable
impl<C> ConditionallySelectable for ProjectivePoint<C>
where
    C: CurveParams,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            x: C::FieldElement::conditional_select(&a.x, &b.x, choice),
            y: C::FieldElement::conditional_select(&a.y, &b.y, choice),
            z: C::FieldElement::conditional_select(&a.z, &b.z, choice),
        }
    }
}

// impl add over projective point
impl<C: CurveParams> Add<ProjectivePoint<C>> for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;
    fn add(self, rhs: Self) -> Self::Output {
        ProjectivePoint::add(&self, &rhs)
    }
}

// impl add over projective point
impl<C: CurveParams> Add<&ProjectivePoint<C>> for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;
    fn add(self, rhs: &Self) -> Self::Output {
        ProjectivePoint::add(&self, &rhs)
    }
}

// impl addAssign over projective point
impl<C: CurveParams> AddAssign<ProjectivePoint<C>> for ProjectivePoint<C> {
    fn add_assign(&mut self, rhs: ProjectivePoint<C>) {
        *self = ProjectivePoint::add(self, &rhs);
    }
}

// impl addAssign over projective point
impl<C: CurveParams> AddAssign<&ProjectivePoint<C>> for ProjectivePoint<C> {
    fn add_assign(&mut self, rhs: &ProjectivePoint<C>) {
        *self = ProjectivePoint::add(self, &rhs);
    }
}

// impl Sub over projective point
impl<C: CurveParams> Sub<ProjectivePoint<C>> for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;
    fn sub(self, rhs: ProjectivePoint<C>) -> Self::Output {
        Self::sub(&self, &rhs)
    }
}

/// assignment operator for the subtraction
impl<C: CurveParams> SubAssign<ProjectivePoint<C>> for ProjectivePoint<C> {
    fn sub_assign(&mut self, rhs: ProjectivePoint<C>) {
        *self = ProjectivePoint::add(&self, &rhs.neg())
    }
}

///Neg for Projective Point
impl<C> Neg for ProjectivePoint<C>
where
    C: CurveParams,
{
    type Output = ProjectivePoint<C>;

    fn neg(self) -> ProjectivePoint<C> {
        ProjectivePoint::neg(&self)
    }
}
/// impl double trait
impl<C: CurveParams> Double for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;

    fn double(self) -> Self::Output {
        ProjectivePoint::double(&self)
    }
}

pub fn is_on_curve<C: CurveParams>(point: ProjectivePoint<C>) -> bool {
    affinepoint::is_on_curve(point.to_affine())
}

pub fn recursive_squaring<C: CurveArithmetic + CurveParams>(
    g: ProjectivePoint<C>,
) -> [ProjectivePoint<C>; 8] {
    let mut squares = [ProjectivePoint::IDENTITY; 8];
    squares[0] = g;
    for i in 1..8usize {
        squares[i] = squares[i - 1].double()
    }
    squares
}
pub fn exp_by_2<C: CurveArithmetic + CurveParams>(
    a: ProjectivePoint<C>,
    j: usize,
) -> ProjectivePoint<C> {
    let mut temp = a;
    for _ in 0..j {
        temp = temp.double();
    }
    temp
}
