# Traits 

This crate includes the traits which are defined for the implementation of the elliptic curves. These traits include :
* **Curve trait** : which defines the base field bytes, integer type used to represent field elements of elliptic curve and order of the elliptic curve.
* **CurveArithmetic trait** :  which is used to define the  affine and projective coordinates of the elliptic curve along with specifying the scalar field.
* **PointArithmetic trait** : this trait defines the  arithmetic function over the curve like addition of the projective points, doubling of the projective points and mixed addition i.e addition of projective pointwith affine points.
* **CurveParams trait** : which defines the type of the base field on which curve is built, parameters used in the weierstrass equation of the elliptic curve, generator of the curve along with the point arithmetic of the curve.
* **AffineCoordinates  trait** : defines the function to represent the x coordinate of the affine point to as vector of 8-bit unsigned integers along with the function which defines the choice for the y-coordinates is odd or not.
* **OnCurve  trait** : defines the function to check whether the point lie on the curve or not along with the function to compute y-coordinate of the affine point from the x-coordinate.


**AffinePoint** and **ProjectivePoint** structs are defined depending upon the CurveParams trait to represent the point on a weierstrass curve in affine and projective coordinates. 