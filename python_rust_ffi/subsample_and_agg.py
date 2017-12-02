from cffi import FFI
import math
from numpy import random

"""
Differential Privacy Subsample and Aggreate algorithm implementation.
Makes a foreign function call on the Snapping Mechanism written in Rust.

Paradigm: procedural

Author: Joseph George
"""

############################################################### FFI
ffi = FFI()

#C function signature matching Rust "snapping_mech"
ffi.cdef( """
    double snapping_mechanism(double, double, double);
""")

#open SO file
#this will be a DLL file if on windows
C = ffi.dlopen("/home/osboxes/Desktop/DP/target/debug/libsnapping_mech.so")
############################################################### FFI

"""
Performs the primary subsample and aggregate algorithm.  Raises an
exception if an entry of the database occurs in too many subsamples.

Args:
    x: database of doubles (list)
    eps: privacy parameter epsilon (double)
    delta: privacy parameter delta (double)
Returns: 
    mode
"""
def subsample_and_aggregate(x, eps, delta):
	n = len(x)
	q = eps / float((64 * math.log10(1 / delta)))
	m = int(math.ceil(math.log(n / delta) / (q * q))) # subsample number

	samples = [] # will be list of double lists
	frequencies	= [] # list of integer values
	for i in range(0, n):
		frequencies.append(0)

	# get m subsamples
	for i in range(0, m):
		curr_sample = sample(frequencies, x, q) # returns list
		samples.append(curr_sample)

	threshold = 2 * q * m
	for f in range(0, len(frequencies)): # abort if element is picked more than 2qm times
		if float(f) > threshold:
			raise ValueError('An element was picked too many times.  Aborting.')

	sample_variances = [] # list of integers
	for i in range(0, len(samples)):
		sample_variances = sample_var(samples[i])

	var_frequencies = {} # {integer, integer} dictionary
	for num in range(0, len(sample_variances)):
		key = sample_variances[num]
		if key in var_frequencies:
			var_frequencies[key] = var_frequencies[key] + 1
		else:
			var_frequencies[key] = 1

	num_modes, num_submodes, mode, submode = 0, 0, 0, 0
	for key in var_frequencies:
		f = var_frequencies[key]
		if f > num_modes:
			num_submodes = num_modes
			submode = mode
			num_modes = f
			mode = key
		elif f > num_submodes:
			num_submodes = f
			submode = key

	d = (num_modes - num_submodes) / (4 * m * q) - 1
	noisy_stability = C.snapping_mechanism(d, delta/eps, 1000.0) # FFI call to rust snapping_mechanism
	if noisy_stability > (math.log10(1 / delta) / eps):
		return mode
	else:
		raise ValueError('Noisy stability is too low.  Aborting.')

"""
Sample a subset of database x, where each entry is picked with a
probability of q.

Args:
    frequencies: list containing the number of times element is picked
    x: database
    q: probability of picking entry in database
Returns: 
    res: subsamples from database
"""
def sample(frequencies, x, q):
	res = []
	for i in range(0, len(x)):
		p = random.uniform(0, 1)
		if p <= q:
			frequencies[i] = frequencies[i] + 1
			res.append(x[i])

	return res

"""
Returns a sample from the guassian distribution.

Args:
    mu: mean
    var: variance
Returns: 
    sample from gaussian
"""
def gaussian(mu, var):
	x = random.normal()
	return (math.sqrt(var) * x) + mu

"""
Returns the rounded down sample variance.

Args:
    sample: a list of samples, each of which is a floating point number
Returns: 
    rounded down sample variance
"""
def sample_var(sample):
	total = 0.0
	for i in range(0, len(sample)):
		total += sample[i]

	sample_mean = total/len(sample)
	total_var = 0.0
	for i in range(0, len(sample)):
		total_var += (sample[i] - sample_mean) * (sample[i] - sample_mean)

	return int(total_var/len(sample))

############################################################### MAIN

### USER DEFINED DATA
mu = 5.0    #mean
var = 3.5   #variance
n = 3000    #number of samples
eps = 2.0   #privacy parameter epsilon
delta = 0.1 #sensitivity

db = []
for i in range(0, n): #populate database with gaussian samples
	db.append(gaussian(mu, var))

true_var = sample_var(db)
private_var = 0.0
try:
	private_var = subsample_and_aggregate(db, eps, delta)
	if private_var == true_var:
		print('Private subsampling estimated the correct variance of: ' + private_var)
	else:
		print('Private subsampling estimated a variance of: ' + private_var)
		print('Actual variance is: ' + true_var)
except ValueError as error:
	raise

############################################################### MAIN