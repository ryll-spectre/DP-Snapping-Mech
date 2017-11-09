# Differential Privacy: The Snapping Mechanism

## Overview

This Rust program serves as an implementation of Mironov's *Snapping Mechanism* described in section 5.2 of his paper, 
*On significance of the least significant bits for differential privacy*.  Note that this it is still a work in progress, as a few technical details need to be cleared up potentially in the uniform distribution function and the addition of more precise rounding for logarithmic calculations.

## Running the Program

Assuming you're using a UNIX based system with the latest version of `Rust` and `Cargo` installed, download the repository, `cd` to the directory, and `cargo run`

## I/O Structure

The program, in its current state, simply asks the user for a number of parameters to simulate the results of a query result fed through the snapping mechanism.  These parameters include:

- **f(D)** - the result of a query on some database D
- **Δ** - sensitivity 
- **ε** - differential privacy parameter
- **B** - a number such that λ < B < 2^{46} λ where λ = Δ/ε

The resulting output is, in theory, a differentially private and secure version (in terms of floating point implementation) of the original query on database D with added noise using the snapping mechanism.
