mod snapping_mech; // contains public snapping mechanism

extern crate rand;
extern crate rug;

use rand::{random, Open01};
use rand::distributions::{IndependentSample, Range};
use std::f64;
use std::io;
use rug::ops::Pow;

// Implementation of laplacian noise generator which uses snapping mechanism in its place
fn main()
{
	// Snapping Mechanism call: f(D), Lambda, B
	let dp_noisy_output = snapping_mech::snapping_mechanism(12.0, 2.0, 10000.0);
	println!("Result of adding noise to query result f(D): ");
    println!("{}", dp_noisy_output);

    // Snapping Mechanism call: f(D), Lambda.  B IS FIXED.
    let dp_noisy_output_2 = snapping_mech::snapping_mechanism_2(12.0, 2.0);
    println!("Result of adding noise to query result f(D): ");
    println!("{}", dp_noisy_output_2);

    // Run snapping mechanism 'x' number of times to store results in file for plotting
    snapping_mech::store_results(12.0, 2.0, 10000.0, 1000000);
}