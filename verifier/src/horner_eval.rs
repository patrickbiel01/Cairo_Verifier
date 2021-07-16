use num256::uint256::Uint256 as Uint256;

use crate::prime_field;
use crate::uint256_ops::{make_copy, get_uint256};


 /*
	TLDR: Compute f(a) quickly, where f is a polynomial and 'a' is a real value

	Computes the evaluation of a polynomial f(x) = sum(a_i * x^i) on the given point.
	The coefficients of the polynomial are given in
	a_0 = coefs_start[0], ..., a_{n-1} = coefs_start[n - 1]
	where n = nCoefs = fri_last_layer_deg_bound. Note that coefs_starts is not actually an array but
	a direct index
	The function requires that n is divisible by 8.
*/
pub fn horner_eval(
	coefs_start: usize, point: Uint256, n_coefs: usize, ctx: & Vec<Uint256>
) -> Uint256 {
	let mut result = get_uint256("0");

	assert!( n_coefs % 8 == 0 ); // Number of polynomial coefficients must be divisible by 8
	assert!( n_coefs < 4096 ); // No more than 4096 coefficients are supported

	let mut coefs_idx = coefs_start + n_coefs;

	while coefs_idx > coefs_start {
		// Reduce coefs_idx by 8 field elements.
		coefs_idx -= 8;

		// Apply 4 Horner steps (result := result * point + coef).
		result = make_copy(&ctx[coefs_idx + 4]) + prime_field::fmul(

				make_copy(&ctx[coefs_idx + 5]) + prime_field::fmul(
					make_copy(&ctx[coefs_idx + 6]) + prime_field::fmul(
						make_copy(&ctx[coefs_idx + 7]) + prime_field::fmul(
							result, make_copy(&point)
						),
						make_copy(&point)
					),
					make_copy(&point)
				),

				make_copy(&point)

			);

		// Apply 4 additional Horner steps
		result = make_copy( &ctx[coefs_idx] ) + prime_field::fmul(

				make_copy(&ctx[coefs_idx + 1]) + prime_field::fmul(
					make_copy(&ctx[coefs_idx + 2]) + prime_field::fmul(
						make_copy(&ctx[coefs_idx + 3]) + prime_field::fmul(
							result, make_copy(&point)
						),
						make_copy(&point)
					),
					make_copy(&point)
				),

				make_copy(&point)

			);

	}
	
	// Since the last operation was "add" (instead of "addmod"), we need to take result % prime.
	return prime_field::mod_prime( result );


}