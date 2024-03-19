# BLS12-381 curve

BLS12-381  is a pairing-friendly elliptic curve, designed by Sean Bowe as an upgrade to the Zcash protocol. The name is a part of the family of curves described by Barreto, Lynn and Scott. 
The equation that describes the curve is defined by $\ E(F_p) : y^2 = x^3 +4$ defined over 381-bit prime field  $\ F_p$ [https://github.com/arithmic/field_trait/tree/main/bls381/src] with p = "1A0111EA397FE69A4B1BA7B6434BACD764774B84F38512BF6730D2A0F6B0F6241EABFFFEB153FFFFB9FEFFFFFFFFAAAB".

The key features of our implementation of the BLS curve,
* The order of the curve is "73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001".
* Montgomery form is used in the arithmetic implementation.
* The implemented structure is easy to parse.
* The generator of the curve is G = ("17F1D3A73197D7942695638C4FA9AC0FC3688C4F9774B905A14E3A3F171BAC586C55E83FF97A1AEFFB3AF00ADB22C6BB", <br>
  "08B3F481E3AAA0F1A09E30ED741D8AE4FCF5E095D5D00AF600DB18CB2C04B3EDD03CC744A2888AE40CAA232946C5E7E1").

It also contains the implementation of the curve over the extension field $\ F_{p^2}$ and $\ F_{p^{12}}$ with curve equation $\ E(F_{p^2}) : y^2 = x^3 + (4+i)$ and $\ E(F_{p^{12}}) : y^2 = x^3 + 4 respectively.