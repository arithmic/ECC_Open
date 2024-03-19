# BN-254 curve

Barreto-Naehrig (BN) curve  is a pairing-friendly elliptic curve described by the equation $\ E(F_p) : y^2 = x^3 + 3$ defined over a over 254-bit prime field $\ F_p$[https://github.com/arithmic/field_trait/tree/main/bn254/src].

The key features of our implementation of the BN-254 curve :
* The order of the curve is "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001".
* The implemented structure is easy to parse.
* The generator of the curve is G =  (1,2).

It also contains the implementation of the curve over the extension field $\ F_{p^2}$ and $\ F_{p^{12}}$ with curve equation $\ E(F_{p^2}) : y^2 = x^3 + 3/(9+u)$ and $\ E(F_{p^{12}}) : y^2 = x^3 + 3 respectively.