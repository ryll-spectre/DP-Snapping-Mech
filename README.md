# The Snapping Mechanism: A Differentially Private and Secure Noise Generator

## Background

Differential Privacy is a property of algorithms which aim to provide reasonable accuracy on queries from databases while preserving the privacy of the users in these databases from various attacks.  A wealth of information on this important field can be found [here](https://www.cis.upenn.edu/~aaroth/Papers/privacybook.pdf), and the formal definiton of Differential Privacy taken from Dwork and Roth can be found below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/images/diff_priv.PNG)

Unfortunately, as thoroughly outlined by Mironov in his paper, [*On significance of the least significant bits for differential privacy*](https://www.microsoft.com/en-us/research/publication/on-significance-of-the-least-significant-bits-for-differential-privacy/), real-life, floating point implementations of several noise generation methods for differentially private algorithms such as the Laplace mechanism result in differential privacy breaches due to the finite precision and rounding effects of floating point operations.

To overcome this vulnerability, Mironov developed the *Snapping Mechanism*, which can serve as a replacement for the Laplace Mechanism where noise is added to some query **f(D)** on some database **D**.  A mathematical overview of the snapping mechanism is shown below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/images/sm.PNG)

The snapping mechanism, and thus our implementation of the snapping mechanism, has been proven to satisfy <img src="http://latex.codecogs.com/svg.latex?1/\lambda+2^{-49}\mathrm{B}/\lambda" border="0"/> - *differential privacy* when <img src="http://latex.codecogs.com/svg.latex?\lambda<\mathrm{B}<2^{46}\lambda" border="0"/>, where <img src="http://latex.codecogs.com/svg.latex?\lambda=\Delta/\epsilon}" border="0"/>

## Implementation

We've modeled the snapping mechanism as accurately as possible through the creation of a Rust program which can be called as a function.  The uniform distribution **U** over <img src="http://latex.codecogs.com/svg.latex?\mathrm{D}\cap(0,1)" border="0"/> was implemented by randomly sampling a number from the open interval (0.0,1.0), which contains all possible 64-bit floating point values.

The snapping mechanism function is implemented in two versions:

```rust
fn snapping_mechanism(fD: f64, lambda: f64, B: f64) -> f64
```
```rust
fn snapping_mechanism_2(fD: f64, lambda: f64) -> f64
```

In the first version, the user can specify their own value of **B** in addition to their privacy parameter, **λ**.  In the second version, **B** is fixed to be <img src="http://latex.codecogs.com/svg.latex?\lambda\cdot2^{45}" border="0"/>, allowing for a large range of values to be output.

In both cases, the snapping mechanism function returns a 64-bit floating point number which is the result of adding noise to some query **f(D)** in a secure and differentially private way.

## Running the Rust Program

## Python FFI Call to snapping_mechanism
