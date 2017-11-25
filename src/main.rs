mod snapping_mech; // contains public snapping mechanism

extern crate dataplotlib;
extern crate rand;
extern crate rug;

use rand::{random, Open01};
use rand::distributions::{IndependentSample, Range};
use std::f64;
use std::io;
use std::iter::repeat;
use dataplotlib::util::{zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;

// Implementation of laplacian noise generator which uses snapping mechanism in its place
fn main()
{
	// call the snapping mechanism using specified parameter.

	let dp_noisy_output = snapping_mech::snapping_mechanism(12.0, 2.0, 10000.0);
	println!("Result of adding noise to query result f(D): ");
    println!("{}", dp_noisy_output);

    // Run the noisy output to generate a plot of distribution of outputs
    // Will generate a different output each time because of sample from uniform dist
    let mut output_vals: Vec<f64> = Vec::new();
    let mut iterations: Vec<f64> = Vec::new();
    let mut iter = 0.0;
    for x in 0..1000000
    {
    	output_vals.push(snapping_mech::snapping_mechanism(12.0, 2.0, 10000.0));
    	iterations.push(iter);
    	iter = iter + 1.0;
    }
    // Pass results to plotting function
    // TODO: need to plot value vs. probability of that value
    plot_output_dist(&iterations, &output_vals);
}

// Plot output values of snapping mechanism with parameters held constant
fn plot_output_dist(output_vals: & Vec<f64>, iterations: & Vec<f64>)
{
	let plot_vals = zip2(iterations, output_vals);
	let mut pb = PlotBuilder2D::new();
	pb.add_color_xy(plot_vals, [1.0, 0.0, 0.0, 1.0]);

	let mut plt = Plotter::new();
    plt.plot2d(pb);
    plt.join();
}