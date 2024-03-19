# Stark-252 Curve 

The Stark-friendly elliptic curve is defined by the equation 
$\ E(F_p) : y^2 = x^3 + ax + b$ (mod p) over the  basefield  $\ F_p$ [https://github.com/arithmic/field_trait/tree/main/stark252/src] , where a and b are constants. The parameters of the curve are:<br>
a = 1,<br>
b = "06F21413EFBE40DE150E596D72F7A8C5609AD26C15C915C1F4CDFCB99CEE9E89",<br>
p = "0800000000000011000000000000000000000000000000000000000000000001".<br>


The key features of our implementation of the stark curve :
* The order of the curve is "0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2F".
* The implemented structure is easy to parse.
* The generator of the curve is G = ("01EF15C18599971B7BECED415A40F0C7DEACFD9B0D1819E03D723D8BC943CFCA",<br>
"005668060AA49730B7BE4801DF46EC62DE53ECD11ABE43A32873000C36E8DC1F").

