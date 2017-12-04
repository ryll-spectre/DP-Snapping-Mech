# The Snapping Mechanism: A Differentially Private and Secure Noise Generator

## Background

Differential Privacy is a property of algorithms which aim to provide reasonable accuracy on queries from databases while preserving the privacy of the users in these databases from various attacks.  A wealth of information on this important field can be found [here](https://www.cis.upenn.edu/~aaroth/Papers/privacybook.pdf), and the formal definiton of Differential Privacy taken from Dwork and Roth can be found below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/images/diff_priv.PNG)

Unfortunately, as thoroughly outlined by Mironov in his paper, [*On significance of the least significant bits for differential privacy*](https://www.microsoft.com/en-us/research/publication/on-significance-of-the-least-significant-bits-for-differential-privacy/), real-life, floating point implementations of several noise generation methods for differentially private algorithms such as the Laplace mechanism result in differential privacy breaches due to the finite precision and rounding effects of floating point operations.

To overcome this vulnerability, Mironov developed the *Snapping Mechanism*, which can serve as a replacement for the Laplace Mechanism where noise is added to some query **f(D)** on some database **D**.  A mathematical overview of the snapping mechanism is shown below.

![alt text](https://github.com/jjgccg/jjgccg.github.io/blob/master/images/sm.PNG)

The snapping mechanism, and thus our implementation of the snapping mechanism, has been proven to satisfy (1/λ + )

## Implementation
<img src="http://latex.codecogs.com/gif.latex? 1/ \lambda" border="0"/>
