use num256::Uint256;

use crate::uint256_ops;
use crate::prime_field;
use crate::stark_params;
use crate::memory_map as mem_map;

pub fn oods_check(ctx: &mut Vec<Uint256>) {
	oods_check_layout_1(ctx);
}

/* ------------------------------------------
		LAYOUT 1 OODS
 ------------------------------------------- */

// For each query point we want to invert (2 + N_ROWS_IN_MASK) items:
//  The query point itself (x).
//  The denominator for the constraint polynomial (x-z^constraintDegree)
//  [(x-(g^rowNumber)z) for rowNumber in mask].
 static BATCH_INVERSE_CHUNK: usize = 2 + stark_params::N_ROWS_IN_MASK;
 static BATCH_INVERSE_SIZE: usize = BATCH_INVERSE_CHUNK + mem_map::MAX_N_QUERIES;


/*
	Builds and sums boundary constraints that check that the prover provided the proper evaluations
	out of domain evaluations for the trace and composition columns.
	The inputs to this function are:
		The verifier context.
	The boundary constraints for the trace enforce claims of the form f(g^k*z) = c by
	requiring the quotient (f(x) - c)/(x-g^k*z) to be a low degree polynomial.
	The boundary constraints for the composition enforce claims of the form h(z^d) = c by
	requiring the quotient (h(x) - c)/(x-z^d) to be a low degree polynomial.
	Where:
		f is a trace column.
		h is a composition column.
		z is the out of domain sampling point.
		g is the trace generator
		k is the offset in the mask.
		d is the degree of the composition polynomial.
		c is the evaluation sent by the prover.
*/
pub fn oods_check_layout_1(ctx: &mut Vec<Uint256>) -> (usize, usize) {

	let mut batch_inverse_vec: Vec<Uint256> = vec![ uint256_ops::get_uint256("0"); 2 * BATCH_INVERSE_SIZE ];

	oods_prepare_inverses_layout_1(ctx, &mut batch_inverse_vec);

	let mut fri_queue = 110;
	let fri_queue_end = fri_queue + 3* uint256_ops::to_usize(&ctx[10]);
	let mut trace_query_response = 1160;
	let mut composition_query_responses = 2216;

	//Points to first actual value in batch_inverse_vec
	// The content of batchInverseOut is described in oodsPrepareInverses.
	let mut denominators_idx = 1;

	 while fri_queue < fri_queue_end {
		 // res accumulates numbers modulo PRIME. Since 31*PRIME < 2**256, we may add up to
		// 31 numbers without fear of overflow, and use addmod modulo PRIME only every
		// 31 iterations, and once more at the very end.
		let mut res = uint256_ops::get_uint256("0");


		// Mask items for column #0.
		// Read the next element.
		let mut column_val = prime_field::fmul(
			ctx[trace_query_response + 0].clone(), prime_field::get_k_montgomery_r_inv()
		);

		/* Trace constraints */
		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1].clone(), ctx[958 + 0].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 0].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2].clone(), ctx[958 + 1].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 1].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[3].clone(), ctx[958 + 2].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 2].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[4].clone(), ctx[958 + 3].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 3].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[5].clone(), ctx[958 + 4].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 4].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[6].clone(), ctx[958 + 5].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 5].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[7].clone(), ctx[958 + 6].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 6].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[8].clone(), ctx[958 + 7].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 7].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[9].clone(), ctx[958 + 8].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 8].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[10].clone(), ctx[958 + 9].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 9].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[11].clone(), ctx[958 + 10].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 10].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[12].clone(), ctx[958 + 11].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 11].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[13].clone(), ctx[958 + 12].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 12].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[14].clone(), ctx[958 + 13].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 13].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[15].clone(), ctx[958 + 14].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 14].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[16].clone(), ctx[958 + 15].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 15].clone() 
		);

		// Mask items for column #1.
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 1].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+0].clone(), ctx[958 + 16].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 16].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 17].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 17].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+55].clone(), ctx[958 + 18].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 18].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+56].clone(), ctx[958 + 19].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 19].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+63].clone(), ctx[958 + 20].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 20].clone() 
		);


		// Mask items for column #2.
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 2].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+0].clone(), ctx[958 + 21].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 21].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 22].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 22].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 23].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 23].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 24].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 24].clone() 
		);

		// Mask items for column #3.
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 3].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 25].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 25].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 26].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 26].clone() 
		);

		// Mask items for column #4
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 4].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 27].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 27].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 28].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 28].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[46+1].clone(), ctx[958 + 29].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 29].clone() 
		);
		

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[47+1].clone(), ctx[958 + 30].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 30].clone() 
			)
		);
		

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[48+1].clone(), ctx[958 + 31].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 31].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[49+1].clone(), ctx[958 + 32].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 32].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[53+1].clone(), ctx[958 + 33].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 33].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[54+1].clone(), ctx[958 + 34].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 34].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 35].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 35].clone() 
		);

		// Mask items for column #5.
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 5].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 36].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 36].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 37].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 37].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 38].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 38].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 39].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 39].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[63+1].clone(), ctx[958 + 40].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 40].clone() 
		);

		// Mask items for column #6
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 6].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 41].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 41].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 42].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 42].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 43].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 43].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 44].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 44].clone() 
		);

		// Mask items for column #7.
		// Read the next element.
		column_val = prime_field::fmul(
			ctx[trace_query_response + 7].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 45].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 45].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 46].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 46].clone() 
		);

		// Mask items for column #8
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 8].clone(), prime_field::get_k_montgomery_r_inv()
		);


		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 47].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 47].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 48].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 48].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[46+1].clone(), ctx[958 + 49].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 49].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[47+1].clone(), ctx[958 + 50].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 50].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[48+1].clone(), ctx[958 + 51].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 51].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[49+1].clone(), ctx[958 + 52].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 52].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[53+1].clone(), ctx[958 + 53].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 53].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[54+1].clone(), ctx[958 + 54].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 54].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 55].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 55].clone() 
		);

		// Mask items for column #9
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 9].clone(), prime_field::get_k_montgomery_r_inv()
		);


		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 56].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 56].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 57].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 57].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 58].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 58].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 59].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 59].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[63+1].clone(), ctx[958 + 60].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 60].clone() 
		);

		// Mask items for column #10
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 10].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[0+1].clone(), ctx[958 + 61].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 61].clone() 
			)		
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 62].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 62].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 63].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 63].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 64].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 64].clone() 
		);

		// Mask items for column #11
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 11].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 65].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 65].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 66].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 66].clone() 
		);

		// Mask items for column #12
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 12].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 67].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 67].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 68].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 68].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[46+1].clone(), ctx[958 + 69].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 69].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[47+1].clone(), ctx[958 + 70].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 70].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[48+1].clone(), ctx[958 + 71].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 71].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[49+1].clone(), ctx[958 + 72].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 72].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[53+1].clone(), ctx[958 + 73].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 73].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[54+1].clone(), ctx[958 + 74].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 74].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 75].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 75].clone() 
		);

		// Mask items for column #13
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 13].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 76].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 76].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 77].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 77].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 78].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 78].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 79].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 79].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[63+1].clone(), ctx[958 + 80].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 80].clone() 
		);

		// Mask items for column #14
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 14].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 81].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 81].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 82].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 82].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 83].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 83].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 84].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 84].clone() 
		);

		// Mask items for column #15
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 15].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 85].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 85].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[55+1].clone(), ctx[958 + 86].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 86].clone() 
		);

		// Mask items for column #16
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 16].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 87].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 87].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 88].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 88].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[46+1].clone(), ctx[958 + 89].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 89].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[47+1].clone(), ctx[958 + 90].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 90].clone() 
		);

		res = res.clone() +prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[48+1].clone(), ctx[958 + 91].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 91].clone() 
		);

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[49+1].clone(), ctx[958 + 92].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 92].clone() 
			)
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[53+1].clone(), ctx[958 + 93].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 93].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[54+1].clone(), ctx[958 + 94].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 94].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[56+1].clone(), ctx[958 + 95].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 95].clone() 
		);

		// Mask items for column #17
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 17].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 96].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 96].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 97].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 97].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2+1].clone(), ctx[958 + 98].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 98].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[3+1].clone(), ctx[958 + 99].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 99].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[4+1].clone(), ctx[958 + 100].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 100].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[5+1].clone(), ctx[958 + 101].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 101].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[6+1].clone(), ctx[958 + 102].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 102].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[7+1].clone(), ctx[958 + 103].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 103].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[8+1].clone(), ctx[958 + 104].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 104].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[9+1].clone(), ctx[958 + 105].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 105].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[12+1].clone(), ctx[958 + 106].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 106].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[13+1].clone(), ctx[958 + 107].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 107].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[16+1].clone(), ctx[958 + 108].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 108].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[20+1].clone(), ctx[958 + 109].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 109].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[21+1].clone(), ctx[958 + 110].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 110].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[27+1].clone(), ctx[958 + 111].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 111].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[28+1].clone(), ctx[958 + 112].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 112].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[33+1].clone(), ctx[958 + 113].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 113].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[34+1].clone(), ctx[958 + 114].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 114].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[38+1].clone(), ctx[958 + 115].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 115].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[39+1].clone(), ctx[958 + 116].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 116].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[42+1].clone(), ctx[958 + 117].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 117].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[43+1].clone(), ctx[958 + 118].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 118].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[45+1].clone(), ctx[958 + 119].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 119].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[50+1].clone(), ctx[958 + 120].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 120].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[52+1].clone(), ctx[958 + 121].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 121].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[57+1].clone(), ctx[958 + 122].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 122].clone() 
		);

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[58+1].clone(), ctx[958 + 123].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 123].clone() 
			)
		);
		

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[59+1].clone(), ctx[958 + 124].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 124].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[60+1].clone(), ctx[958 + 125].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 125].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[61+1].clone(), ctx[958 + 126].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 126].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[62+1].clone(), ctx[958 + 127].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 127].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[71+1].clone(), ctx[958 + 128].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 128].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[72+1].clone(), ctx[958 + 129].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 129].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[81+1].clone(), ctx[958 + 130].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 130].clone() 
		);

		// Mask items for column #18
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 18].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 131].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 131].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 132].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 132].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2+1].clone(), ctx[958 + 133].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 133].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[3+1].clone(), ctx[958 + 134].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 134].clone() 
		);

		// Mask items for column #19
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 19].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 135].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 135].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 136].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 136].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2+1].clone(), ctx[958 + 137].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 137].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[3+1].clone(), ctx[958 + 138].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 138].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[4+1].clone(), ctx[958 + 139].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 139].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[5+1].clone(), ctx[958 + 140].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 140].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[6+1].clone(), ctx[958 + 141].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 141].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[7+1].clone(), ctx[958 + 142].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 142].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[8+1].clone(), ctx[958 + 143].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 143].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[9+1].clone(), ctx[958 + 144].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 144].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[11+1].clone(), ctx[958 + 145].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 145].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[12+1].clone(), ctx[958 + 146].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 146].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[13+1].clone(), ctx[958 + 147].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 147].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[15+1].clone(), ctx[958 + 148].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 148].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[17+1].clone(), ctx[958 + 149].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 149].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[21+1].clone(), ctx[958 + 150].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 150].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[23+1].clone(), ctx[958 + 151].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 151].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[24+1].clone(), ctx[958 + 152].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 152].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[26+1].clone(), ctx[958 + 153].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 153].clone() 
		);	

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[29+1].clone(), ctx[958 + 154].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 154].clone() 
			)
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[31+1].clone(), ctx[958 + 155].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 155].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[35+1].clone(), ctx[958 + 156].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 156].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[37+1].clone(), ctx[958 + 157].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 157].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[40+1].clone(), ctx[958 + 158].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 158].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[41+1].clone(), ctx[958 + 159].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 159].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[69+1].clone(), ctx[958 + 160].clone() //Nice!
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 160].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[70+1].clone(), ctx[958 + 161].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 161].clone() 
		);

		// Mask items for column #20
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 20].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 162].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 162].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 163].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 163].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2+1].clone(), ctx[958 + 164].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 164].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[4+1].clone(), ctx[958 + 165].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 165].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[6+1].clone(), ctx[958 + 166].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 166].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[8+1].clone(), ctx[958 + 167].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 167].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[10+1].clone(), ctx[958 + 168].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 168].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[12+1].clone(), ctx[958 + 169].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 169].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[14+1].clone(), ctx[958 + 170].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 170].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[17+1].clone(), ctx[958 + 171].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 171].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[18+1].clone(), ctx[958 + 172].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 172].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[19+1].clone(), ctx[958 + 173].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 173].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[20+1].clone(), ctx[958 + 174].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 174].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[22+1].clone(), ctx[958 + 175].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 175].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[25+1].clone(), ctx[958 + 176].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 176].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[27+1].clone(), ctx[958 + 177].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 177].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[30+1].clone(), ctx[958 + 178].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 178].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[32+1].clone(), ctx[958 + 179].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 179].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[36+1].clone(), ctx[958 + 180].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 180].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[44+1].clone(), ctx[958 + 181].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 181].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[51+1].clone(), ctx[958 + 182].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 182].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[64+1].clone(), ctx[958 + 183].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 183].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[65+1].clone(), ctx[958 + 184].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 184].clone() 
		);

		res = prime_field::fadd(
			res.clone(),
			prime_field::fmul(
				prime_field::fmul(
					batch_inverse_vec[66+1].clone(), ctx[958 + 185].clone()
				), 
				column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 185].clone() 
			)
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[67+1].clone(), ctx[958 + 186].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 186].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[68+1].clone(), ctx[958 + 187].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 187].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[73+1].clone(), ctx[958 + 188].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 188].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[74+1].clone(), ctx[958 + 189].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 189].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[75+1].clone(), ctx[958 + 190].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 190].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[76+1].clone(), ctx[958 + 191].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 191].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[77+1].clone(), ctx[958 + 192].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 192].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[78+1].clone(), ctx[958 + 193].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 193].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[79+1].clone(), ctx[958 + 194].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 194].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[80+1].clone(), ctx[958 + 195].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 195].clone() 
		);

		// Mask items for column #21
		// Read the next element
		column_val = prime_field::fmul(
			ctx[trace_query_response + 18].clone(), prime_field::get_k_montgomery_r_inv()
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[0+1].clone(), ctx[958 + 196].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 196].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[1+1].clone(), ctx[958 + 197].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 197].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[2+1].clone(), ctx[958 + 198].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 198].clone() 
		);

		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[5+1].clone(), ctx[958 + 199].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[708 + 199].clone() 
		);

		// Advance traceQueryResponses by amount read, nTraceColumns.
		trace_query_response += 22;

		/* Composition constraints */
		column_val = prime_field::fmul(
			ctx[composition_query_responses].clone(), prime_field::get_k_montgomery_r_inv()
		);
		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[82+1].clone(), ctx[1158].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[908].clone() 
		);

		// Read the next element.
		column_val = prime_field::fmul(
			ctx[composition_query_responses].clone(), prime_field::get_k_montgomery_r_inv()
		);
		res = res.clone() + prime_field::fmul(
			prime_field::fmul(
				batch_inverse_vec[82+1].clone(), ctx[1159].clone()
			), 
			column_val.clone() + prime_field::get_k_modulus() - ctx[909].clone() 
		);


		// Advance compositionQueryResponses by amount read,constraintDegree .
		composition_query_responses += 2;

		// Append the friValue, which is the sum of the out-of-domain-sampling boundary
		// constraints for the trace and composition polynomials, to the friQueue array
		ctx[fri_queue + 1] = prime_field::mod_prime(res);

		// Append the friInvPoint of the current query to the friQueue array.
		ctx[fri_queue + 2] = batch_inverse_vec[denominators_idx + 83].clone();

		// Advance denominatorsPtr by chunk size 2+N_ROWS_IN_MASK.
		denominators_idx = denominators_idx + 84;

		fri_queue += 3;
	 }

	 return (110, 144); /* fri_queue_idx */

}


/*
	Computes and performs batch inverse on all the denominators required for the out of domain
	sampling boundary constraints.
	Since the friEvalPoints are calculated during the computation of the denominators
	this function also adds those to the batch inverse in prepartion for the fri that follows.
	After this function returns, the batch_inverse_out array holds #queries
	chunks of size (2 + N_ROWS_IN_MASK) with the following structure:
	0..(N_ROWS_IN_MASK-1):   [(x - g^i * z)^(-1) for i in rowsInMask]
	N_ROWS_IN_MASK:          (x - z^constraintDegree)^-1
	N_ROWS_IN_MASK+1:        friEvalPointInv.
*/
pub fn oods_prepare_inverses_layout_1(ctx: &mut Vec<Uint256>, batch_inverse_vec: &mut Vec<Uint256>) {
	let trace_generator = ctx[345].clone();
	
	// The array expmodsAndPoints stores subexpressions that are needed
	// for the denominators computation.
	// The array is segmented as follows:
	//    expmodsAndPoints[0:19] (.expmods) expmods used during calculations of the points below.
	//    expmodsAndPoints[19:101] (.points) points used during the denominators calculation.
	let mut exp_mods_and_points: Vec<Uint256> = vec![ uint256_ops::get_uint256("0"); 101 ];


	/* Prepare expmods for computations of trace generator powers */
	exp_mods_and_points[0] = prime_field::fmul(
		trace_generator.clone(), trace_generator.clone() 
	);

	exp_mods_and_points[1] = prime_field::fmul(
		exp_mods_and_points[0].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[2] = prime_field::fmul(
		exp_mods_and_points[1].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[3] = prime_field::fmul(
		exp_mods_and_points[2].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[4] = prime_field::fmul(
		exp_mods_and_points[3].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[5] = prime_field::fmul(
		exp_mods_and_points[4].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[6] = prime_field::fmul(
		exp_mods_and_points[5].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[7] = prime_field::fmul(
		exp_mods_and_points[6].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[8] = prime_field::fmul(
		exp_mods_and_points[7].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[9] = prime_field::fmul(
		exp_mods_and_points[8].clone(), exp_mods_and_points[3].clone()
	);

	exp_mods_and_points[10] = prime_field::fmul(
		exp_mods_and_points[9].clone(), exp_mods_and_points[3].clone()
	);

	exp_mods_and_points[11] = prime_field::fmul(
		exp_mods_and_points[10].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[12] = prime_field::fmul(
		exp_mods_and_points[11].clone(), exp_mods_and_points[1].clone()
	);

	exp_mods_and_points[13] = prime_field::fmul(
		exp_mods_and_points[12].clone(), trace_generator.clone() 
	);

	exp_mods_and_points[14] = prime_field::fmul(
		exp_mods_and_points[13].clone(), exp_mods_and_points[4].clone()
	);

	exp_mods_and_points[15] = prime_field::fmul(
		exp_mods_and_points[11].clone(), exp_mods_and_points[0].clone()
	);

	exp_mods_and_points[16] = prime_field::fmul(
		exp_mods_and_points[15].clone(), exp_mods_and_points[6].clone()
	);

	exp_mods_and_points[17] = prime_field::fpow(
		&trace_generator, &uint256_ops::get_uint256("DF1") //3569
	);

	exp_mods_and_points[18] = prime_field::fpow(
		&trace_generator, &uint256_ops::get_uint256("FCA") //4042
	);
	/* -- End precomputing for trace generator powers ---  */


	let oods_point = ctx[346].clone();

	// Compute denominators for rows with nonconst mask expression.
	// We compute those first because for the const rows we modify the point variable.
	// Compute denominators for rows with const mask expression.
	// expmods_and_points.points[0] = -z.

	let mut point = prime_field::get_k_modulus() - oods_point.clone();
	exp_mods_and_points[19] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 0] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 1] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 2] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 3] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 4] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 5] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 6] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 7] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 8] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 9] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 10] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 11] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 12] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 13] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 14] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 15] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 16] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 17] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 18] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 19] = point.clone();

	point = prime_field::fmul( 
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 20] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 21] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 22] = point.clone();

	point = prime_field::fmul(
		point.clone(),  exp_mods_and_points[1].clone()
	);
	exp_mods_and_points[20 + 23] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 24] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 25] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[5].clone()
	);
	exp_mods_and_points[20 + 26] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 27] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[3].clone()
	);
	exp_mods_and_points[20 + 28] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[7].clone()
	);
	exp_mods_and_points[20 + 29] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[4].clone()
	);
	exp_mods_and_points[20 + 30] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 31] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[6].clone()
	);
	exp_mods_and_points[20 + 32] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 33] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[3].clone()
	);
	exp_mods_and_points[20 + 34] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[3].clone()
	);
	exp_mods_and_points[20 + 35] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[8].clone()
	);
	exp_mods_and_points[20 + 36] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[7].clone()
	);
	exp_mods_and_points[20 + 37] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 38] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[3].clone()
	);
	exp_mods_and_points[20 + 39] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[9].clone()
	);
	exp_mods_and_points[20 + 40] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[7].clone()
	);
	exp_mods_and_points[20 + 41] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 42] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[7].clone()
	);
	exp_mods_and_points[20 + 43] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[11].clone()
	);
	exp_mods_and_points[20 + 44] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[12].clone()
	);
	exp_mods_and_points[20 + 45] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 46] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[1].clone()
	);
	exp_mods_and_points[20 + 47] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 48] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 49] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[7].clone()
	);
	exp_mods_and_points[20 + 50] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[10].clone()
	);
	exp_mods_and_points[20 + 51] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[10].clone()
	);
	exp_mods_and_points[20 + 52] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 53] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[1].clone()
	);
	exp_mods_and_points[20 + 54] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 55] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[5].clone()
	);
	exp_mods_and_points[20 + 56] = point.clone();

	point = prime_field::fmul(
		point.clone(),  exp_mods_and_points[14].clone()
	);
	exp_mods_and_points[20 + 57] = point.clone();

	point = prime_field::fmul(
		point.clone(),  exp_mods_and_points[14].clone()
	);
	exp_mods_and_points[20 + 58] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[16].clone()
	);
	exp_mods_and_points[20 + 59] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[14].clone()
	);
	exp_mods_and_points[20 + 60] = point.clone();

	point = prime_field::fmul(
		point.clone(),exp_mods_and_points[14].clone()
	);
	exp_mods_and_points[20 + 61] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[15].clone()
	);
	exp_mods_and_points[20 + 62] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[17].clone()
	);
	exp_mods_and_points[20 + 63] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[2].clone()
	);
	exp_mods_and_points[20 + 64] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[2].clone()
	);
	exp_mods_and_points[20 + 65] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 66] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 67] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[8].clone()
	);
	exp_mods_and_points[20 + 68] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[6].clone()
	);
	exp_mods_and_points[20 + 69] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[5].clone()
	);
	exp_mods_and_points[20 + 70] = point.clone();

	point = prime_field::fmul(
		point.clone(), trace_generator.clone()
	);
	exp_mods_and_points[20 + 71] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[18].clone()
	);
	exp_mods_and_points[20 + 72] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[3].clone()
	);
	exp_mods_and_points[20 + 73] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[6].clone()
	);
	exp_mods_and_points[20 + 74] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 75] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[2].clone()
	);
	exp_mods_and_points[20 + 76] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 77] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[0].clone()
	);
	exp_mods_and_points[20 + 78] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[2].clone()
	);
	exp_mods_and_points[20 + 79] = point.clone();

	point = prime_field::fmul(
		point.clone(), exp_mods_and_points[13].clone()
	);
	exp_mods_and_points[20 + 80] = point.clone();

	/* --- End of computing denominators for rows with const mask expression.  --- */






	let mut eval_points_idx = 910;
	let eval_points_end_idx = eval_points_idx + uint256_ops::to_usize(&ctx[10]);
	let mut products_idx = 1;
	let mut vals_idx = 1;
	let mut partial_prod = uint256_ops::get_uint256("1");
	let minus_point_pow = prime_field::get_k_modulus() - prime_field::fmul(oods_point.clone(), oods_point.clone());

	while eval_points_idx < eval_points_end_idx {
		let eval_point = ctx[eval_points_idx].clone();

		let shifted_eval_point = prime_field::fmul(
			eval_point.clone(), prime_field::get_generator_val()
		);



		/* Below: Calculating denominator for row i: x - z*g^i */

		let mut denominator = shifted_eval_point.clone() + exp_mods_and_points[19].clone();
		batch_inverse_vec[products_idx + 0] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 0] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[20].clone();
		batch_inverse_vec[products_idx + 1] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 1] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[21].clone();
		batch_inverse_vec[products_idx + 2] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 2] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[22].clone();
		batch_inverse_vec[products_idx + 3] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 3] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[23].clone();
		batch_inverse_vec[products_idx + 4] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 4] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[24].clone();
		batch_inverse_vec[products_idx + 5] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 5] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[25].clone();
		batch_inverse_vec[products_idx + 6] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 6] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[26].clone();
		batch_inverse_vec[products_idx + 7] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 7] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[27].clone();
		batch_inverse_vec[products_idx + 8] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 8] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[28].clone();
		batch_inverse_vec[products_idx + 9] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 9] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[29].clone();
		batch_inverse_vec[products_idx + 10] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 10] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[30].clone();
		batch_inverse_vec[products_idx + 11] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 11] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[31].clone();
		batch_inverse_vec[products_idx + 12] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 12] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[32].clone();
		batch_inverse_vec[products_idx + 13] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 13] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[33].clone();
		batch_inverse_vec[products_idx + 14] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 14] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[34].clone();
		batch_inverse_vec[products_idx + 15] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 15] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[35].clone();
		batch_inverse_vec[products_idx + 16] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 16] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[36].clone();
		batch_inverse_vec[products_idx + 17] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 17] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[37].clone();
		batch_inverse_vec[products_idx + 18] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 18] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[38].clone();
		batch_inverse_vec[products_idx + 19] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 19] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[39].clone();
		batch_inverse_vec[products_idx + 20] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 20] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[40].clone();
		batch_inverse_vec[products_idx + 21] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 21] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[41].clone();
		batch_inverse_vec[products_idx + 22] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 22] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[42].clone();
		batch_inverse_vec[products_idx + 23] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 23] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[43].clone();
		batch_inverse_vec[products_idx + 24] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 24] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[44].clone();
		batch_inverse_vec[products_idx + 25] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 25] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[45].clone();
		batch_inverse_vec[products_idx + 26] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 26] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[46].clone();
		batch_inverse_vec[products_idx + 27] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 27] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[47].clone();
		batch_inverse_vec[products_idx + 28] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 28] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[48].clone();
		batch_inverse_vec[products_idx + 29] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 29] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[49].clone();
		batch_inverse_vec[products_idx + 30] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 30] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[50].clone();
		batch_inverse_vec[products_idx + 31] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 31] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[51].clone();
		batch_inverse_vec[products_idx + 32] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 32] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[52].clone();
		batch_inverse_vec[products_idx + 33] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 33] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[53].clone();
		batch_inverse_vec[products_idx + 34] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 34] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[54].clone();
		batch_inverse_vec[products_idx + 35] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 35] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[55].clone();
		batch_inverse_vec[products_idx + 36] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 36] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[56].clone();
		batch_inverse_vec[products_idx + 37] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 37] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[57].clone();
		batch_inverse_vec[products_idx + 38] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 38] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[58].clone();
		batch_inverse_vec[products_idx + 39] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 39] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[59].clone();
		batch_inverse_vec[products_idx + 40] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 40] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[60].clone();
		batch_inverse_vec[products_idx + 41] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 41] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[61].clone();
		batch_inverse_vec[products_idx + 42] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 42] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[62].clone();
		batch_inverse_vec[products_idx + 43] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 43] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[63].clone();
		batch_inverse_vec[products_idx + 44] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 44] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[64].clone();
		batch_inverse_vec[products_idx + 45] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 45] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[65].clone();
		batch_inverse_vec[products_idx + 46] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 46] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[66].clone();
		batch_inverse_vec[products_idx + 47] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 47] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[67].clone();
		batch_inverse_vec[products_idx + 48] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 48] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[68].clone();
		batch_inverse_vec[products_idx + 49] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 49] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[69].clone();
		batch_inverse_vec[products_idx + 50] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 50] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[70].clone();
		batch_inverse_vec[products_idx + 51] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 51] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[71].clone();
		batch_inverse_vec[products_idx + 52] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 52] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[72].clone();
		batch_inverse_vec[products_idx + 53] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 53] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[73].clone();
		batch_inverse_vec[products_idx + 54] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 54] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[74].clone();
		batch_inverse_vec[products_idx + 55] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 55] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[75].clone();
		batch_inverse_vec[products_idx + 56] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 56] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[76].clone();
		batch_inverse_vec[products_idx + 57] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 57] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[77].clone();
		batch_inverse_vec[products_idx + 58] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 58] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[78].clone();
		batch_inverse_vec[products_idx + 59] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 59] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[79].clone();
		batch_inverse_vec[products_idx + 60] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 60] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[80].clone();
		batch_inverse_vec[products_idx + 61] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 61] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[81].clone();
		batch_inverse_vec[products_idx + 62] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 62] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[82].clone();
		batch_inverse_vec[products_idx + 63] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 63] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[83].clone();
		batch_inverse_vec[products_idx + 64] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 64] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[84].clone();
		batch_inverse_vec[products_idx + 65] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 65] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[85].clone();
		batch_inverse_vec[products_idx + 66] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 66] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[86].clone();
		batch_inverse_vec[products_idx + 67] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 67] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[87].clone();
		batch_inverse_vec[products_idx + 68] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 68] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[88].clone();
		batch_inverse_vec[products_idx + 69] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 69] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[89].clone();
		batch_inverse_vec[products_idx + 70] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 70] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[90].clone();
		batch_inverse_vec[products_idx + 71] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 71] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[91].clone();
		batch_inverse_vec[products_idx + 72] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 72] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[92].clone();
		batch_inverse_vec[products_idx + 73] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 73] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[93].clone();
		batch_inverse_vec[products_idx + 74] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 74] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[94].clone();
		batch_inverse_vec[products_idx + 75] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 75] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[95].clone();
		batch_inverse_vec[products_idx + 76] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 76] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[96].clone();
		batch_inverse_vec[products_idx + 77] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 77] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[97].clone();
		batch_inverse_vec[products_idx + 78] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 78] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[98].clone();
		batch_inverse_vec[products_idx + 79] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 79] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[99].clone();
		batch_inverse_vec[products_idx + 80] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 80] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		denominator = shifted_eval_point.clone() + exp_mods_and_points[100].clone();
		batch_inverse_vec[products_idx + 81] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 81] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);

		

		// Calculate the denominator for the composition polynomial columns: x - z^2.
		denominator = shifted_eval_point.clone() + minus_point_pow.clone();
		batch_inverse_vec[products_idx + 82] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 82] = denominator.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), denominator.clone()
		);
		/* ------- End Calculating denominator for row i: x - z*g^i ------- */




		// Add evalPoint to batch inverse inputs.
		// inverse(evalPoint) is going to be used by FRI.
		batch_inverse_vec[products_idx + 83] = partial_prod.clone();
		batch_inverse_vec[vals_idx + 83] = eval_point.clone();
		partial_prod = prime_field::fmul(
			partial_prod.clone(), eval_point.clone()
		);

		//Advance indxes
		eval_points_idx += 1;
		products_idx += 84;
		vals_idx += 84;
	}




	let prod_to_vals_offset = 4032;
	let first_partial_prod_idx = 1;
	// Compute the inverse of the product.
	let mut prod_inv = prime_field::fpow(
		&partial_prod, &(prime_field::get_k_modulus() - uint256_ops::get_uint256("2"))
	);

	if prod_inv == uint256_ops::get_uint256("0") {
		assert!(false); //Batch inverse product is zero
	}

	// Compute the inverses.
	// Loop over denominator_invs in reverse order.
	// currentPartialProductPtr is initialized to one past the end.
	let mut curr_partial_prod_idx = products_idx.clone();

	// Loop in blocks of size 8 as much as possible: we can loop over a full block as long as
	// currentPartialProductPtr >= firstPartialProductPtr + 8, or equivalently,
	// currentPartialProductPtr > firstPartialProductPtr + 7.
	// We use the latter comparison since there is no >= evm opcode.
	let mid_partial_prod_idx = first_partial_prod_idx + 7;

	while curr_partial_prod_idx > mid_partial_prod_idx {
		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv.clone(), batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);

	}





	// Loop over the remainder.
	while curr_partial_prod_idx > first_partial_prod_idx {
		curr_partial_prod_idx -= 1;
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
		batch_inverse_vec[curr_partial_prod_idx] = prime_field::fmul(
			batch_inverse_vec[curr_partial_prod_idx].clone(), prod_inv.clone()
		);
		// Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv, batch_inverse_vec[curr_partial_prod_idx + prod_to_vals_offset].clone()
		);
	}




}
