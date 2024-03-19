# Cheetah Curve 

The cheetah curve is a STARK-friendly elliptic curve defined over a sextic extension of $\ F_p$[https://github.com/arithmic/field_trait/tree/main/cheetah64/src]. The cheetah curve is described by the equation 
$\ E(F_p) : y^2 = x^3 + ax + b$ (mod p) , where a and b are constants. The parameters of the curve are:<br>
a = 1, <br>
b = 395 + u  with $\ u^6 = 7$, <br>
p = $\ 2^{64} − 2^{32}+ 1 $. <br>

The key features of our implementation of the cheetah curve :
* The order of the curve is "7AF2599B3B3F22D0563FBF0F990A37B5327AA72330157722D443623EAED4ACCF".
* Montgomery form is used in the arithmetic implementation.
* The generator of the curve is G = ("B39050F01F7B1F33534ECE54DE4B2C8534ECE54DE4B2C89215ADFC1E9258907757A0BCB26A142D263A588F4B0118A1","8AA635F2719D255FB7040BD639EF3CC4949C360784284EC2C77F564783EF13A126821D894FA8EA0FD57F0D0D47482534").

