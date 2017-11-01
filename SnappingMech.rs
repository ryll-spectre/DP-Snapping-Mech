fn main(){
	/*takes outputs from clamp funtion on f(D), then computes the value of 
	the output + S * lambda * the output from the uniform function. Then,
	this value is rounded before having the clamp function called again
	*/
	let num = alpha(6.0_f64);
	let num2 = alpha(0.4_f64);
	let num3 = round(4.0_f64,6.0_f64);
	let num4 = round(0.1_f64, 6.75_f64);
	let num5 = clamp(5.0_f64, 6.0_f64);
	println!("{}", num);
	println!("{}", num2);
	println!("{}", num3);
	println!("{}", num4);
	println!("{}", num5);
	
}

///
/// Returns a floating point number related to input x
///
///# Arguments
///
///* 'x' - a float ....?
///* 'b' - a float to compare to x
///

fn clamp(x: f64 , b: f64) -> f64{
	//takes a float and outputs a value for B
	//if x> B, output B if x < -B output -B, otherwise output x
	if x > b {
		return b;
	} else if x < -b{
		return -b;
	} else {
		return x;
	}
}

///
/// Returns a vector of floating point numbers
///
///# Arguments
///
///* 'p' - an integer to act as a precision parameter
///
//fn uniformDist(p: i32) -> Vec<f64>{
	//takes a precision value and creates a vector of values
	//that is the natural log of the values from a uniform dist


//}

///
///Returns a floating point number representing alpha, the smallest power
///of 2 greater than or equal to lambda
///
///# Arguments
///
///* 'lam' - a floating point number that is the ratio of the sensitivity to privacy
///
fn alpha(lam: f64) -> f64{
	//calculates the smallest power of 2 >= lambda
	let mut n = 1_f64;
	if lam < 1_f64 {
		while lam < n {
			n = n / 2_f64;
		}
		return n*2_f64
	} else {
		while n < lam {
			n = n*2_f64;
		}
		return n
	}

}

///
///Returns a floating point number equal to some multiple of alpha
///
///# Arguments
///
///* 'alpha' - the value returned from the alpha function
///* 'answer' - the result of the computations on the first clamp output
///
fn round(alpha: f64, answer: f64) -> f64{
	//rounds the inner clamp function to closest multiple of
	//alpha with ties resolved towards pos inf
	let mut ans = alpha;
	if ans >= answer {
		return ans;
	} else {
		while answer > ans {
			ans += alpha;
		}
		if ans - answer <= answer - (ans - alpha){
		
			return ans;
		} else {
	
			return ans -alpha;
		}
	}
}



