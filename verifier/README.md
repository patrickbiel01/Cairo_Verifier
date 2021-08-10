# STARK Verifier
Below is a brief explamnation about the steps in verification. For more information look at the resources section on the main page.

## Arithmetization
The verifier ensures consistency between the compositional polynomial and the Trace Low Domain Extension using Out of Domain Sampling (OODS).

## Fast Reed-Solomon Interactive Oracle Proof (FRI)
Assuming the prover is honest, the compositional polynomial should be of low degree. To prove so succictly, the prover sends over polynomials in the form: f(x) = g(x^2) + xh(x^2) until a constant is reached. The FRI verifier can randomly sample the commitments and interpolate the next layer of commitments and verify the layers.
<br><br>
f(x)  = g(x^2) + xh(x^2)
<br>
f(-x) = g((-x)^2) - xh((-x)^2) = g(x^2) - xh(x^2)
<br>
<br>
=>
    2g(x^2) = f(x) + f(-x)
    <br>
    2h(x^2) = (f(x) - f(-x))/x
   <br>
   <br>
=> The 2 times interpolation at evalPoint is:<br>
      2*(g(x^2) + evalPoint*h(x^2)) = f(x) + f(-x) + evalPoint*(f(x) - f(-x))*xInv.
<br>
