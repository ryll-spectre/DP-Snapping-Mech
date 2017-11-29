extern crate rand;
extern crate rug;

use rand::{random, Open01};
use rand::distributions::{IndependentSample, Range};
use std::f64;
use std::io;
use rug::Float;
use rug::float::Round;
use rug::ops::Pow;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Returns a database query answer with added noise, such that the result is differentially
/// private
///
///# Arguments
///
///* 'fD' - result of original database query
///* 'lambda' - sensitivity/privacy parameter epsilon
///* 'B' - a number such that: lambda < B < (2^{46})(lambda)
#[no_mangle]
pub extern "C" fn snapping_mechanism(fD: f64, lambda: f64, B: f64) -> f64
{
	// Check to make sure B is within range
	if (lambda >= B) | (B >= (lambda * 2_f64.pow(46)))
	{
		panic!("Invalid B value: must satisfy λ < B < λ*2^(46)");
	}

	let mut S = 0.0; // 50% chance S is -1 or +1
	let between = Range::new(0u8, 1u8); //Two possible choices - 0 or 1
	let mut rng = rand::thread_rng();
	let num = between.ind_sample(&mut rng);
	if num == 1 {S = 1.0};
	if num == 0 {S = -1.0};

	let clamp_fD = clamp(fD, B); //clamp_B (f(D))
	let uni_dist_num = uniform_dist(); // LN(U*)
	let inner_result = clamp_fD + S * lambda * uni_dist_num;

	//calculate outer clamp by passing in inner result rounded to alpha
	let lambda_sub = lambda_sub(lambda);
	let round = round(inner_result, lambda_sub);

	return clamp(round, B)
}

/// Returns a database query answer with added noise, such that the result is differentially
/// private.  Fixed B Version.
///
///# Arguments
///
///* 'fD' - result of original database query
///* 'lambda' - sensitivity/privacy parameter epsilon
pub fn snapping_mechanism_2(fD: f64, lambda: f64) -> f64
{
	let mut S = 0.0; // 50% chance S is -1 or +1
	let between = Range::new(0u8, 1u8); //Two possible choices - 0 or 1
	let mut rng = rand::thread_rng();
	let num = between.ind_sample(&mut rng);
	if num == 1 {S = 1.0};
	if num == 0 {S = -1.0};

	let B = lambda * 2_f64.pow(45); // DEFAULT: Upper bound of B

	let clamp_fD = clamp(fD, B); 
	let uni_dist_num = uniform_dist(); // LN(U*)
	let inner_result = clamp_fD + S * lambda * uni_dist_num;

	//calculate outer clamp by passing in inner result rounded to alpha
	let lambda_sub = lambda_sub(lambda);
	let round = round(inner_result, lambda_sub);

	return clamp(round, B)
}

/// Runs the snapping mechanism a user specified number of times, storing
/// the results into a text file so that it can be viewed, analyzed, and plotted.
///
///# Arguments
///
///* 'fD' - result of original database query
///* 'lambda' - sensitivity/privacy parameter epsilon
///* 'B' - a number such that: lambda < B < (2^{46})(lambda)
///* 'num_iteration' - numer of times to perform snapping mechanism call
pub fn store_results(fD: f64, lambda: f64, B: f64, num_iteration: i64)
{
	let mut output_vals: Vec<f64> = Vec::new();
	for x in 0..num_iteration
    {
        let current_val = snapping_mechanism(fD, lambda, B);
        output_vals.push(current_val);
    }

    let strings: Vec<String> = output_vals.iter().map(|n| n.to_string()).collect();
    let mut file = match File::create("/home/osboxes/results.txt")
    {
    	Err(e) =>
    	{
    		println!("Couldn't open results.txt");
    		return;
    	},
    	Ok(file) => file,
    };
    writeln!(file, "{}", strings.join(", "));
}

/// Returns a floating point number related to input x
///
///# Arguments
///
///* 'x' - input float of clamp function
///* 'b' - a float to compare to x
fn clamp(x: f64 , b: f64) -> f64{
	//output B is x > B
	//ouput -B if x < -B
	//output x otherwise
	if x > b 
	{
		return b;
	} 
	else if x < -b
	{
		return -b;
	} 
	else 
	{
		return x;
	}
}

/// Returns the logarithm of a 64b floating point number which is sampled
/// from the uniform distribution over (0,1)
fn uniform_dist() -> f64
{
	// Generates floating point numbers uniformly in open interval (0,1)
    let Open01(sample) = random::<Open01<f64>>();

    // log10(sample) with precision 64 is rounded to nearest 64 bit float
    let mut val_log = Float::with_val_round(64, f64::log10(sample), Round::Nearest);

    return f64::log10(sample);
}

/// Returns a floating point number representing Lambda, the smallest power
/// of 2 greater than or equal to lambda
///
///# Arguments
///
///* 'lam' - a floating point number that is the ratio of the sensitivity to privacy
fn lambda_sub(lam: f64) -> f64
{
	//calculates the smallest power of 2 >= lambda
	let mut n = 1_f64;
	let mut m = 0_f64;
	if lam < 1_f64 
	{
		while lam < n 
		{
			n = n / 2_f64;
			m = m+1_f64;
		}
		return m
	} 
	else if lam > 1_f64
	{
		while n < lam 
		{
			n = n*2_f64;
			m=m+1_f64;
		}
		return m
	} 
	else 
	{
		return 0_f64;
	}
}

/// Returns a floating point number which is the closest multiple of
/// Lambda in a uniform distribution
///
///# Arguments
///
///* 'l_sub' - the value returned from the lambda_sub function
///* 'inner_clamp' - the result of the computations on the first clamp output
fn round(l_sub: f64, inner_clamp: f64) -> f64
{
	//rounds the inner clamp function to closest multiple of
	//l_sub with ties resolved towards pos inf
	let mut ans = l_sub;
	if ans >= inner_clamp 
	{
		return ans;
	} 
	else 
	{
		while inner_clamp > ans 
		{
			ans += l_sub;
		}
		if ans - inner_clamp <= inner_clamp - (ans - l_sub)
		{
			return ans;
		} 
		else 
		{
			return ans - l_sub;
		}
	}
}