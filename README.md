# ECC CURVE
Elliptic curve are the set of solutions over K(field) to the following equation <br>
$\ E(K) : y^2 = ax^3 + bx^2 + cx + d$ <br>
where $\ a,b,c,d$ are in  $\ K$,  $\ a$ must not be equal to zero and the polynomial in $\ x$ does not have a multiple root.
ECC curve repository contains the implementation of different elliptic curves. The implemented curves are: 
* BLS12-381 curve
* Stark-252 curve
* Bandersnatch curve
* Jubjub curve
* Cheetah curve
* BN254 curve
* Baby Jubjub curve
* Grumpkin curve


### How to use the ECC curve in your project
Add the ECC curve crates into your project by adding the specific crate name(as in Cargo.toml file) <br>
 "crate_name" = {git = "ssh://git@github.com/arithmic/ECC_Open.git" , branch = "main"} <br>
in your Cargo.toml file

#### Dependencies :
1. crypto_bigint [https://github.com/arithmic/crypto_bigint].
2. field traits repository [https://github.com/arithmic/Field_Open.git].

