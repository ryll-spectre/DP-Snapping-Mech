extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::f64;
use std::io;

fn main(){
	/*takes outputs from clamp funtion on f(D), then computes the value of 
	the output + S * lambda * the output from the uniform function. Then,
	this value is rounded before having the clamp function called again
	*/

    println!("\n");
    println!("PROGRAM START:");

    // For each required parameter:
    // 1) Request user input and read into a string buffer
    // 2) Parse the string buffer into a 64b floating point value

    let mut f_D = String::new();
    println!("Enter f(D) for database D: ");
    io::stdin().read_line(&mut f_D)
    	.ok()
    	.expect("Couldn't read line!");
    let f_D_float = f_D.trim().parse::<f64>().unwrap();
    println!("{}", f_D_float);

    let mut delta = String::new();
    println!("Enter sensitivity (Δ): ");
        io::stdin().read_line(&mut delta)
    	.ok()
    	.expect("Couldn't read line!");
    let delta_float = delta.trim().parse::<f64>().unwrap();

    let mut epsilon = String::new();
    println!("Enter privacy parameter (ε): ");
        io::stdin().read_line(&mut epsilon)
    	.ok()
    	.expect("Couldn't read line!");
    let epsilon_float = epsilon.trim().parse::<f64>().unwrap();

    let mut B = String::new();
    println!("Enter B: ");
    io::stdin().read_line(&mut B)
    	.ok()
    	.expect("Couldn't read line!");
    let B_float = B.trim().parse::<f64>().unwrap();

    let lambda = delta_float/epsilon_float;
    let DP_noisy_output = snapping_mechanism(f_D_float, lambda, B_float); //final result
    println!("Result of adding noise to query result f(D): ");
    println!("{}", DP_noisy_output);
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
fn uniformDist() -> f64
{
	let mut rng = rand::thread_rng(); 
    let range = Range::new(0.0,1.0);
    let sample = range.ind_sample(&mut rng);

    // TODO: may need to account for rounding errors in logarithm
    return f64::log10(sample)
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

/// Returns a database query answer with noise, such that the result is differentially
/// private
///
///# Arguments
///
///* 'fD' - result of original database query
///* 'lambda' - sensitivity/privacy parameter epsilon
///* 'B' - a number such that: lambda < B < (2^{46})(lambda)
fn snapping_mechanism(fD: f64, lambda: f64, B: f64) -> f64
{
	// 50% chance S is -1 or +1
	let mut S = 0.0;
	let between = Range::new(0u8, 1u8); //Two possible choices - 0 or 1
	let mut rng = rand::thread_rng();
	let num = between.ind_sample(&mut rng);
	if num == 1 {S = 1.0};
	if num == 0 {S = -1.0};
	println!("The value of S is:");
	println!("{}", S);

	let clampfD = clamp(fD, B); //clamp_B (f(D))
	let uni_dist_num = uniformDist(); // LN(U*)
	println!("The uniform dist num is: ");
	println!("{}", uni_dist_num);
	let inner_result = clampfD + S * lambda * uni_dist_num;

	//calculate outer clamp by passing in inner result rounded to alpha
	let lambda_sub = lambda_sub(lambda);
	println!("Lambda is: ");
	println!("{}", lambda_sub);
	let round = round(inner_result, lambda_sub);

	return clamp(round, B)
}