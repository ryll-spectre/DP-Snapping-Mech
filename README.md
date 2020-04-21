# The Snapping Mechanism: A Differentially Private and Secure Noise Generator

## Background

Differential Privacy is a property of algorithms which aims to provide reasonable accuracy on queries from databases while preserving the privacy of the users in these databases from various attacks.  A wealth of information on this important field can be found [here](https://www.cis.upenn.edu/~aaroth/Papers/privacybook.pdf), and the formal definiton of Differential Privacy taken from Dwork and Roth can be found below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/assets/img/diff_priv.PNG)

Unfortunately, as thoroughly outlined by Mironov in his paper, [*On significance of the least significant bits for differential privacy*](https://www.microsoft.com/en-us/research/publication/on-significance-of-the-least-significant-bits-for-differential-privacy/), real-life, floating point implementations of several noise generation methods for differentially private algorithms such as the Laplace mechanism result in differential privacy breaches due to the finite precision and rounding effects of floating point operations.

To overcome this vulnerability, Mironov developed the *Snapping Mechanism*, which can serve as a replacement for the Laplace Mechanism where noise is added to some query **f(D)** on some database **D**.  A mathematical overview of the snapping mechanism is shown below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/assets/img/sm.PNG)

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

Our program was developed and run on a Linux environment, with the latest versions of Rust and Cargo installed.  Since the snapping mechanism functions are declared as ```pub```, they can be called from another Rust file which may require the use of secure and differentially private noise.

Download the repository and navigate to the source code folder and ```cargo run```.

```main.rs``` is currently set up as a demo so the user can modify the parameters for the snapping mechanism call, and then see the result printed out from calling the function.  Additionally, a function which stores the result in a text file of calling the snapping mechanism with fixed parameters **n** times is currently called inside of main for the purpose of plotting the distribution of output values.  Of course, this can be removed if need be.

## Python FFI

Not everyone who develops a differentially private algorithm may do so in Rust.  Languages such as Python are more widely used and popular.  Rust was used to implement the snapping mechanism simply because of its precision, security, and performance.

Keeping this in mind, we have developed an FFI call to our Rust snapping mechanism implementation in a Python program to model a practical use case.  The Python program is an implementation of the *Subsample and Aggregate* algorithm, which can be found in section 7.1 of [The Algorithmic Foundations of Differential Privacy](https://www.cis.upenn.edu/~aaroth/Papers/privacybook.pdf).

Inside of the ```python_rust_ffi``` directory, you can find the subsample and aggregate algorithm, which creates a foreign function call on the Rust program in the following way, using ```FFI``` from ```cffi```

```python
ffi = FFI()

#C function signature matching Rust "snapping_mech"
ffi.cdef( """
    double snapping_mechanism(double, double, double);
""")

C = ffi.dlopen("/home/osboxes/Desktop/DP/target/debug/libsnapping_mech.so")
```

This code creates a file called ```libsnapping_mech.so``` inside of your debug folder in your Rust code directory which we originally created through using the ```cargo run``` command.  Note that if you are using windows, this file should instead be ```libsnapping_mech.dll```, and also note the user of this python file must change the path in ```ffi.dlopen``` to match their own machine directory structure.

Once this FFI object is created, it is later called in place of Laplace noise as such

```python
noisy_stability = C.snapping_mechanism(d, delta/eps, 1000.0) # FFI call to rust snapping_mechanism
```

Using this method of a Python to Rust function call (from which we express our gratitude to [this](https://bheisler.github.io/post/calling-rust-in-python/) tutorial), a programmer can create an algorithm in Python which requires the use of Laplace noise and instead call the snapping mechanism implemented in Rust.

### Project Authors

Joseph George | jge9@protonmail.ch

Marissa Sisco | marissasisco97@gmail.com
