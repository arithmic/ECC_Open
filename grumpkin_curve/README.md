# GrumpkinCurve  

The Grumpkincurve is based on the scalar field of BN-254 curve[https://github.com/arithmic/field_trait/tree/main/bn254/src]. The Grumpkincurve is defined by the equation $\ E(F_p) : y^2 = x^3 + ax + b$ (mod p), where a and b are constants.The parameters of the curve are: <br>
a = "0", <br>
b = "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593EFFFFFF0", <br>
p = "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001".


The key features of our implementation of the Grumpkincurve : 
* The order of the curve is "0000000000000002CF135E7506A45D632D270D45F1181294833FC48D823F272C" which is the base field of the bn-254 curve.
* The generator of the curve is G =  ("1",<br>
"0000000000000002CF135E7506A45D632D270D45F1181294833FC48D823F272C").
