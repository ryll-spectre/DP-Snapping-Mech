use std::io; 

fn main
{
	// calculate snapping mechanism in here with components from
	// sunroutine calls
}

/// Uniform distribution calculator 
/// Returns a vector containing the logarithm of a floating point distribution (0,1)
/// # Arguments
/// * 'p' - precision with which the uniform distribution over (0,1) is sampled
fn U(p: f32) -> Vec<f64>
{

}


/// Clamping function calculator
/// Returns the result of a clamp
/// # Arguments
/// * 'B' - value such that λ < B < 2^(46)λ 
/// * 'x' - the value to be passed into the clamp function
fn clamp(B: f64, x: f64) -> f64
{
	
}

/// Alpha calculator
/// Returns smallest power of 2 greater than or equal to lambda
/// * 'lambda' - value which is sensitivity/epsilon privacy
fn alpha(lambda: f64) -> f64
{
	
}

/// asdf
/// Returns result of rounding the value with the closest multiple of Alpha 
/// in the uniform distribution
/// #Arguments
/// * 'value' - clamp(f(D) + S + λ + LN(U*))
/// * 'alpha' - alpha parameter
fn lrFloor(value: f64, alpha: f64) -> f64
{
	
}