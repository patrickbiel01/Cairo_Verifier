use num256::uint256::Uint256 as Uint256;

use crate::uint256_ops;
use crate::memory_map as map;
use crate::prime_field;

static OFFSET_LOG_N_STEPS: usize =                      0;


/* -------------------
Note:
	LAYOUT 1
 ----------------------- */

//Calculates: Polynomial Constraints
//RETURNS: compositionFromTraceValue - Uint256
pub fn trace_val_w_polynomial_constraints(ctx: Vec<Uint256>) -> Uint256{
	
	let mut res = uint256_ops::get_uint256("0");

	let point = ctx[map::MM_OODS_POINT].clone();

	//Calculate and store commonly used exponents
	let mut exp_mods: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 21];
	exp_mods[0] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()) );

	exp_mods[1] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("10")) ); // divide by 16

	exp_mods[2] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("2")) );

	exp_mods[3] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("8")) ); 

	exp_mods[4] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("4")) ); // divide by 32

	exp_mods[5] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("100")) ); // divide by 256

	exp_mods[6] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("200")) ); // divide by 512

	exp_mods[7] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("80")) ); // divide by 128

	exp_mods[8] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("1000")) ); // divide by 4096

	exp_mods[9] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("20")) ); // divide by 32

	exp_mods[10] = prime_field::fpow(
		&point, &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("2000")) ); // divide by 8192

	exp_mods[11] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("F") * ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("10"))
	);

	exp_mods[12] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("10") * (ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("10") - uint256_ops::get_uint256("1")) )
	);

	exp_mods[13] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("2") * (ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("2") - uint256_ops::get_uint256("1")) )
	);

	exp_mods[14] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("4") * (ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("4") - uint256_ops::get_uint256("1")) )
	);

	exp_mods[15] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("FF") * ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("100"))
	);

	exp_mods[16] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("3F") * ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("40"))
	);

	exp_mods[17] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("2"))
	);

	exp_mods[18] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("80") * (ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("80") - uint256_ops::get_uint256("1")) )
	);

	exp_mods[19] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("FB") * ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("100"))
	);

	exp_mods[20] = prime_field::fpow(
		&ctx[map::MM_TRACE_GENERATOR], &(uint256_ops::get_uint256("2000") * (ctx[map::MM_TRACE_LENGTH].clone()/uint256_ops::get_uint256("2000") - uint256_ops::get_uint256("1")) )
	);


	

	



	//Calculate Denominators
	//TODO: Maybe change to modular arithmetic if it doesn't work
	let mut denominators: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 22];
	denominators[0] = exp_mods[0].clone()   -   uint256_ops::get_uint256("1");

	denominators[1] = exp_mods[1].clone() - exp_mods[11].clone();

	denominators[2] = exp_mods[1].clone() - uint256_ops::get_uint256("1");

	denominators[3] = point.clone()   -   uint256_ops::get_uint256("1");

	denominators[4] = point.clone()   -   exp_mods[12].clone();

	denominators[5] = exp_mods[2].clone() - uint256_ops::get_uint256("1");

	denominators[6] = point.clone() - exp_mods[13].clone();

	denominators[7] = exp_mods[3].clone() - uint256_ops::get_uint256("1");

	denominators[8] = exp_mods[4].clone() - uint256_ops::get_uint256("1");

	denominators[9] = point.clone() - exp_mods[14].clone();

	denominators[10] = exp_mods[5].clone() - uint256_ops::get_uint256("1");

	denominators[11] = exp_mods[5].clone() - exp_mods[16].clone();

	denominators[12] = exp_mods[5].clone() - exp_mods[15].clone();

	denominators[13] = exp_mods[6].clone() - uint256_ops::get_uint256("1");

	denominators[14] = exp_mods[7].clone() - uint256_ops::get_uint256("1");

	denominators[15] = exp_mods[9].clone() - uint256_ops::get_uint256("1");

	denominators[16] = exp_mods[10].clone() - exp_mods[19].clone();

	denominators[17] = exp_mods[10].clone() - exp_mods[15].clone();

	denominators[18] = exp_mods[8].clone() - exp_mods[19].clone();

	denominators[19] = exp_mods[8].clone() - exp_mods[15].clone();

	denominators[20] = exp_mods[10].clone() - uint256_ops::get_uint256("1");

	denominators[21] = exp_mods[8].clone() - uint256_ops::get_uint256("1");











	/* Compute the inverses of the denominators into denominatorInvs using batch inverse */


	// Start by computing the cumulative product.
	// Let (d_0, d_1, d_2, ..., d_{n-1}) be the values in denominators. After this loop
	// denominatorInvs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
	// d_0 * ... * d_{n-1}.
	// Compute the offset between the partialProducts array and the input values array.
	let mut prod = uint256_ops::get_uint256("1");
	let mut denominator_inv: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 22];
	let mut partial_prod_idx = 0;
	while partial_prod_idx < 22 {
		denominator_inv[partial_prod_idx] = prod.clone();
		// prod *= d_{i}.
		prod = prime_field::fmul(
			prod.clone(), denominators[partial_prod_idx].clone()
		);
		partial_prod_idx += 1;
	}

	// Compute the inverse of the product.
	let mut prod_inv = prime_field::fpow(
		&prod, &prime_field::fsub( prime_field::get_k_modulus(), uint256_ops::get_uint256("2") )
	);

	// if prod_inv == uint256_ops::get_uint256("0") {
	// 	// Solidity generates reverts with reason that look as follows:
	// 	// 1. 4 bytes with the constant 0x08c379a0 (== Keccak256(b'Error(string)')[:4]).
	// 	// 2. 32 bytes offset bytes (always 0x20 as far as i can tell).
	// 	// 3. 32 bytes with the length of the revert reason.
	// 	// 4. Revert reason string.
	// 	assert!(false);//Batch inverse product is zero
	// }
	assert!( prod_inv == uint256_ops::get_uint256("0") ); //Batch inverse product is zero

	// Compute the inverses.
	// Loop over denominator_invs in reverse order.
	// currentPartialProductPtr is initialized to one past the end.
	let mut currentPartialProductPtr = 22;
	while currentPartialProductPtr > 0 {
		// Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i})
		denominator_inv[currentPartialProductPtr] = prime_field::fmul(
			denominator_inv[currentPartialProductPtr].clone(), prod_inv.clone()
		);
		// Update prod_inv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
		prod_inv = prime_field::fmul(
			prod_inv.clone(), denominators[currentPartialProductPtr].clone(),
		);
		currentPartialProductPtr -= 1;
	}







	/* Compute Numerators */
	let mut numerators: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 10];
	numerators[0] = prime_field::fadd(
		exp_mods[1].clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[11].clone() )
	);

	numerators[1] = prime_field::fadd(
		point.clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[12].clone() )
	);

	numerators[2] = prime_field::fadd(
		point.clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[13].clone() )
	);

	numerators[3] = prime_field::fadd(
		point.clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[14].clone() )
	);

	numerators[4] = prime_field::fadd(
		exp_mods[5].clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[15].clone() )
	);

	numerators[5] = prime_field::fadd(
		exp_mods[6].clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[17].clone() )
	);

	numerators[6] = prime_field::fadd(
		point.clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[18].clone() )
	);

	numerators[7] = prime_field::fadd(
		exp_mods[8].clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[15].clone() )
	);

	numerators[8] = prime_field::fadd(
		exp_mods[10].clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[15].clone() )
	);

	numerators[9] = prime_field::fadd(
		point.clone(), prime_field::fsub( prime_field::get_k_modulus(), exp_mods[20].clone() )
	);



	/* Compute useful intermediate values */
	let mut intermediate_vals: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 43];

	{
	// cpu/decode/opcode_rc/bit_0 = column0_row0 - (column0_row1 + column0_row1).
	let val = prime_field::fadd(
	/*column0_row0*/ ctx[map::MM_OODS_VALUES+0].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row1*/ ctx[map::MM_OODS_VALUES+1].clone(), /*column0_row1*/ ctx[map::MM_OODS_VALUES+1].clone() )) );
	intermediate_vals[0] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_2 = column0_row2 - (column0_row3 + column0_row3).
	let val = prime_field::fadd(
	/*column0_row2*/ ctx[map::MM_OODS_VALUES+2].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row3*/ ctx[map::MM_OODS_VALUES+3].clone(), /*column0_row3*/ ctx[map::MM_OODS_VALUES+3].clone() )) );
	intermediate_vals[1] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_4 = column0_row4 - (column0_row5 + column0_row5).
	let val = prime_field::fadd(
	/*column0_row4*/ ctx[map::MM_OODS_VALUES+4].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row5*/ ctx[map::MM_OODS_VALUES+5].clone(), /*column0_row5*/ ctx[map::MM_OODS_VALUES+5].clone() )) );
	intermediate_vals[2] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_3 = column0_row3 - (column0_row4 + column0_row4).
	let val = prime_field::fadd(
	/*column0_row3*/ ctx[map::MM_OODS_VALUES+3].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row4*/ ctx[map::MM_OODS_VALUES+4].clone(), /*column0_row4*/ ctx[map::MM_OODS_VALUES+4].clone() )) );
	intermediate_vals[3] = val.clone();
	}


	{
	// cpu/decode/flag_op1_base_op0_0 = 1 - (cpu__decode__opcode_rc__bit_2 + cpu__decode__opcode_rc__bit_4 + cpu__decode__opcode_rc__bit_3).
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_2*/ intermediate_vals[1].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_4*/ intermediate_vals[2].clone() ),
		/*intermediate_value/cpu/decode/opcode_rc/bit_3*/ intermediate_vals[3].clone() )) );
	intermediate_vals[4] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_5 = column0_row5 - (column0_row6 + column0_row6).
	let val = prime_field::fadd(
	/*column0_row5*/ ctx[map::MM_OODS_VALUES+5].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row6*/ ctx[map::MM_OODS_VALUES+6].clone(), /*column0_row6*/ ctx[map::MM_OODS_VALUES+6].clone() )) );
	intermediate_vals[5] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_6 = column0_row6 - (column0_row7 + column0_row7).
	let val = prime_field::fadd(
	/*column0_row6*/ ctx[map::MM_OODS_VALUES+6].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row7*/ ctx[map::MM_OODS_VALUES+7].clone(), /*column0_row7*/ ctx[map::MM_OODS_VALUES+7].clone() )) );
	intermediate_vals[6] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_9 = column0_row9 - (column0_row10 + column0_row10).
	let val = prime_field::fadd(
	/*column0_row9*/ ctx[map::MM_OODS_VALUES+9].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row10*/ ctx[map::MM_OODS_VALUES+10].clone(), /*column0_row10*/ ctx[map::MM_OODS_VALUES+10].clone() )) );
	intermediate_vals[7] = val.clone();
	}


	{
	// cpu/decode/flag_res_op1_0 = 1 - (cpu__decode__opcode_rc__bit_5 + cpu__decode__opcode_rc__bit_6 + cpu__decode__opcode_rc__bit_9).
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_5*/ intermediate_vals[5].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_6*/ intermediate_vals[6].clone() ),
		/*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone() )) );
	intermediate_vals[8] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_7 = column0_row7 - (column0_row8 + column0_row8).
	let val = prime_field::fadd(
	/*column0_row7*/ ctx[map::MM_OODS_VALUES+7].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row8*/ ctx[map::MM_OODS_VALUES+8].clone(), /*column0_row8*/ ctx[map::MM_OODS_VALUES+8].clone() )) );
	intermediate_vals[9] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_8 = column0_row8 - (column0_row9 + column0_row9).
	let val = prime_field::fadd(
	/*column0_row8*/ ctx[map::MM_OODS_VALUES+8].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row9*/ ctx[map::MM_OODS_VALUES+9].clone(), /*column0_row9*/ ctx[map::MM_OODS_VALUES+9].clone() )) );
	intermediate_vals[10] = val.clone();
	}


	{
	// cpu/decode/flag_pc_update_regular_0 = 1 - (cpu__decode__opcode_rc__bit_7 + cpu__decode__opcode_rc__bit_8 + cpu__decode__opcode_rc__bit_9).
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_7*/ intermediate_vals[9].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_8*/ intermediate_vals[10].clone() ),
		/*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone() )) );
	intermediate_vals[11] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_12 = column0_row12 - (column0_row13 + column0_row13).
	let val = prime_field::fadd(
	/*column0_row12*/ ctx[map::MM_OODS_VALUES+12].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row13*/ ctx[map::MM_OODS_VALUES+13].clone(), /*column0_row13*/ ctx[map::MM_OODS_VALUES+13].clone() )) );
	intermediate_vals[12] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_13 = column0_row13 - (column0_row14 + column0_row14).
	let val = prime_field::fadd(
	/*column0_row13*/ ctx[map::MM_OODS_VALUES+13].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row14*/ ctx[map::MM_OODS_VALUES+14].clone(), /*column0_row14*/ ctx[map::MM_OODS_VALUES+14].clone() )) );
	intermediate_vals[13] = val.clone();
	}


	{
	// cpu/decode/fp_update_regular_0 = 1 - (cpu__decode__opcode_rc__bit_12 + cpu__decode__opcode_rc__bit_13).
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
		/*intermediate_value/cpu/decode/opcode_rc/bit_13*/ intermediate_vals[13].clone() )) );
	intermediate_vals[14] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_1 = column0_row1 - (column0_row2 + column0_row2).
	let val = prime_field::fadd(
	/*column0_row1*/ ctx[map::MM_OODS_VALUES+1].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row2*/ ctx[map::MM_OODS_VALUES+2].clone(), /*column0_row2*/ ctx[map::MM_OODS_VALUES+2].clone() )) );
	intermediate_vals[15] = val.clone();
	}


	{
	// npc_reg_0 = column17_row0 + cpu__decode__opcode_rc__bit_2 + 1.
	let val = prime_field::fadd(
	prime_field::fadd(
		/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(),
		/*intermediate_value/cpu/decode/opcode_rc/bit_2*/ intermediate_vals[1].clone() ),
	uint256_ops::get_uint256("1") );
	intermediate_vals[16] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_10 = column0_row10 - (column0_row11 + column0_row11).
	let val = prime_field::fadd(
	/*column0_row10*/ ctx[map::MM_OODS_VALUES+10].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row11*/ ctx[map::MM_OODS_VALUES+11].clone(), /*column0_row11*/ ctx[map::MM_OODS_VALUES+11].clone() )) );
	intermediate_vals[17] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_11 = column0_row11 - (column0_row12 + column0_row12).
	let val = prime_field::fadd(
	/*column0_row11*/ ctx[map::MM_OODS_VALUES+11].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row12*/ ctx[map::MM_OODS_VALUES+12].clone(), /*column0_row12*/ ctx[map::MM_OODS_VALUES+12].clone() )) );
	intermediate_vals[18] = val.clone();
	}


	{
	// cpu/decode/opcode_rc/bit_14 = column0_row14 - (column0_row15 + column0_row15).
	let val = prime_field::fadd(
	/*column0_row14*/ ctx[map::MM_OODS_VALUES+14].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column0_row15*/ ctx[map::MM_OODS_VALUES+15].clone(), /*column0_row15*/ ctx[map::MM_OODS_VALUES+15].clone() )) );
	intermediate_vals[19] = val.clone();
	}


	{
	// memory/address_diff_0 = column18_row2 - column18_row0.
	let val = prime_field::fadd(/*column18_row2*/ ctx[map::MM_OODS_VALUES+133].clone(), sub(prime_field::get_k_modulus(), /*column18_row0*/ ctx[map::MM_OODS_VALUES+131].clone()) );
	intermediate_vals[20] = val.clone();
	}


	{
	// rc16/diff_0 = column19_row6 - column19_row2.
	let val = prime_field::fadd(/*column19_row6*/ ctx[map::MM_OODS_VALUES+141].clone(), sub(prime_field::get_k_modulus(), /*column19_row2*/ ctx[map::MM_OODS_VALUES+137].clone()) );
	intermediate_vals[21] = val.clone();
	}


	{
	// pedersen/hash0/ec_subset_sum/bit_0 = column4_row0 - (column4_row1 + column4_row1).
	let val = prime_field::fadd(
	/*column4_row0*/ ctx[map::MM_OODS_VALUES+27].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column4_row1*/ ctx[map::MM_OODS_VALUES+28].clone(), /*column4_row1*/ ctx[map::MM_OODS_VALUES+28].clone() )) );
	intermediate_vals[22] = val.clone();
	}


	{
	// pedersen/hash0/ec_subset_sum/bit_neg_0 = 1 - pedersen__hash0__ec_subset_sum__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(prime_field::get_k_modulus(), /*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone()) );
	intermediate_vals[23] = val.clone();
	}


	{
	// pedersen/hash1/ec_subset_sum/bit_0 = column8_row0 - (column8_row1 + column8_row1).
	let val = prime_field::fadd(
	/*column8_row0*/ ctx[map::MM_OODS_VALUES+47].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column8_row1*/ ctx[map::MM_OODS_VALUES+48].clone(), /*column8_row1*/ ctx[map::MM_OODS_VALUES+48].clone() )) );
	intermediate_vals[24] = val.clone();
	}


	{
	// pedersen/hash1/ec_subset_sum/bit_neg_0 = 1 - pedersen__hash1__ec_subset_sum__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(prime_field::get_k_modulus(), /*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone()) );
	intermediate_vals[25] = val.clone();
	}


	{
	// pedersen/hash2/ec_subset_sum/bit_0 = column12_row0 - (column12_row1 + column12_row1).
	let val = prime_field::fadd(
	/*column12_row0*/ ctx[map::MM_OODS_VALUES+67].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column12_row1*/ ctx[map::MM_OODS_VALUES+68].clone(), /*column12_row1*/ ctx[map::MM_OODS_VALUES+68].clone() )) );
	intermediate_vals[26] = val.clone();
	}


	{
	// pedersen/hash2/ec_subset_sum/bit_neg_0 = 1 - pedersen__hash2__ec_subset_sum__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(prime_field::get_k_modulus(), /*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone()) );
	intermediate_vals[27] = val.clone();
	}


	{
	// pedersen/hash3/ec_subset_sum/bit_0 = column16_row0 - (column16_row1 + column16_row1).
	let val = prime_field::fadd(
	/*column16_row0*/ ctx[map::MM_OODS_VALUES+87].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column16_row1*/ ctx[map::MM_OODS_VALUES+88].clone(), /*column16_row1*/ ctx[map::MM_OODS_VALUES+88].clone() )) );
	intermediate_vals[28] = val.clone();
	}


	{
	// pedersen/hash3/ec_subset_sum/bit_neg_0 = 1 - pedersen__hash3__ec_subset_sum__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(prime_field::get_k_modulus(), /*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone()) );
	intermediate_vals[29] = val.clone();
	}


	{
	// rc_builtin/value0_0 = column19_row12.
	let val = /*column19_row12*/ ctx[map::MM_OODS_VALUES+146].clone();
	intermediate_vals[30] = val.clone();
	}


	{
	// rc_builtin/value1_0 = rc_builtin__value0_0 * offset_size + column19_row28.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value0_0*/ intermediate_vals[30].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row28*/ ctx[map::MM_OODS_VALUES+152].clone() );
	intermediate_vals[31] = val.clone();
	}


	{
	// rc_builtin/value2_0 = rc_builtin__value1_0 * offset_size + column19_row44.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value1_0*/ intermediate_vals[31].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row44*/ ctx[map::MM_OODS_VALUES+154].clone() );
	intermediate_vals[32] = val.clone();
	}


	{
	// rc_builtin/value3_0 = rc_builtin__value2_0 * offset_size + column19_row60.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value2_0*/ intermediate_vals[32].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row60*/ ctx[map::MM_OODS_VALUES+155].clone() );
	intermediate_vals[33] = val.clone();
	}


	{
	// rc_builtin/value4_0 = rc_builtin__value3_0 * offset_size + column19_row76.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value3_0*/ intermediate_vals[33].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row76*/ ctx[map::MM_OODS_VALUES+156].clone() );
	intermediate_vals[34] = val.clone();
	}


	{
	// rc_builtin/value5_0 = rc_builtin__value4_0 * offset_size + column19_row92.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value4_0*/ intermediate_vals[34].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row92*/ ctx[map::MM_OODS_VALUES+157].clone() );
	intermediate_vals[35] = val.clone();
	}


	{
	// rc_builtin/value6_0 = rc_builtin__value5_0 * offset_size + column19_row108.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value5_0*/ intermediate_vals[35].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row108*/ ctx[map::MM_OODS_VALUES+158].clone() );
	intermediate_vals[36] = val.clone();
	}


	{
	// rc_builtin/value7_0 = rc_builtin__value6_0 * offset_size + column19_row124.
	let val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc_builtin/value6_0*/ intermediate_vals[36].clone(),
		/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
	/*column19_row124*/ ctx[map::MM_OODS_VALUES+159].clone() );
	intermediate_vals[37] = val.clone();
	}


	{
	// ecdsa/signature0/doubling_key/x_squared = column19_row7 * column19_row7.
	let val = prime_field::fmul(/*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone() );
	intermediate_vals[38] = val.clone();
	}


	{
	// ecdsa/signature0/exponentiate_generator/bit_0 = column20_row30 - (column20_row62 + column20_row62).
	let val = prime_field::fadd(
	/*column20_row30*/ ctx[map::MM_OODS_VALUES+176].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column20_row62*/ ctx[map::MM_OODS_VALUES+179].clone(), /*column20_row62*/ ctx[map::MM_OODS_VALUES+179].clone() )) );
	intermediate_vals[39] = val.clone();
	}


	{
	// ecdsa/signature0/exponentiate_generator/bit_neg_0 = 1 - ecdsa__signature0__exponentiate_generator__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone()) );
	intermediate_vals[40] = val.clone();
	}


	{
	// ecdsa/signature0/exponentiate_key/bit_0 = column20_row2 - (column20_row18 + column20_row18).
	let val = prime_field::fadd(
	/*column20_row2*/ ctx[map::MM_OODS_VALUES+164].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column20_row18*/ ctx[map::MM_OODS_VALUES+172].clone(), /*column20_row18*/ ctx[map::MM_OODS_VALUES+172].clone() )) );
	intermediate_vals[41] = val.clone();
	}


	{
	// ecdsa/signature0/exponentiate_key/bit_neg_0 = 1 - ecdsa__signature0__exponentiate_key__bit_0.
	let val = prime_field::fadd(
	uint256_ops::get_uint256("1"),
	sub(
		prime_field::get_k_modulus(),
		/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone()) );
	intermediate_vals[42] = val.clone();
	}









	

	 /* ---- Compute the result of the composition polynomial ---- */

	{
	// Constraint expression for cpu/decode/opcode_rc/bit: cpu__decode__opcode_rc__bit_0 * cpu__decode__opcode_rc__bit_0 - cpu__decode__opcode_rc__bit_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone(),
		/*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone()) );

	// Numerator: point^(trace_length / 16) - trace_generator^(15 * trace_length / 16).
	// val *= numerators[0].
	val = prime_field::fmul(val.clone(), numerators[0].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[0].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[0]*/ ctx[map::MM_COEFFICIENTS+0].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/opcode_rc/zero: column0_row0.
	let mut val = /*column0_row0*/ ctx[map::MM_OODS_VALUES+0].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - trace_generator^(15 * trace_length / 16).
	// val *= denominator_invs[1].
	val = prime_field::fmul(val.clone(), denominator_inv[1].clone() );

	// res += val * coefficients[1].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[1]*/ ctx[map::MM_COEFFICIENTS+1].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/opcode_rc_input: column17_row1 - (((column0_row0 * offset_size + column19_row4) * offset_size + column19_row8) * offset_size + column19_row0).
	let mut val = prime_field::fadd(
	/*column17_row1*/ ctx[map::MM_OODS_VALUES+97].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fmul(
			prime_field::fadd(
			prime_field::fmul(
				prime_field::fadd(
				prime_field::fmul(/*column0_row0*/ ctx[map::MM_OODS_VALUES+0].clone(), /*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
				/*column19_row4*/ ctx[map::MM_OODS_VALUES+139].clone() ),
				/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
			/*column19_row8*/ ctx[map::MM_OODS_VALUES+143].clone() ),
			/*offset_size*/ ctx[map::MM_OFFSET_SIZE].clone() ),
		/*column19_row0*/ ctx[map::MM_OODS_VALUES+135].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[2].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[2]*/ ctx[map::MM_COEFFICIENTS+2].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/flag_op1_base_op0_bit: cpu__decode__flag_op1_base_op0_0 * cpu__decode__flag_op1_base_op0_0 - cpu__decode__flag_op1_base_op0_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/cpu/decode/flag_op1_base_op0_0*/ intermediate_vals[4].clone(),
		/*intermediate_value/cpu/decode/flag_op1_base_op0_0*/ intermediate_vals[4].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/flag_op1_base_op0_0*/ intermediate_vals[4].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[3].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[3]*/ ctx[map::MM_COEFFICIENTS+3].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/flag_res_op1_bit: cpu__decode__flag_res_op1_0 * cpu__decode__flag_res_op1_0 - cpu__decode__flag_res_op1_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/cpu/decode/flag_res_op1_0*/ intermediate_vals[8].clone(),
		/*intermediate_value/cpu/decode/flag_res_op1_0*/ intermediate_vals[8].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/flag_res_op1_0*/ intermediate_vals[8].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[4].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[4]*/ ctx[map::MM_COEFFICIENTS+4].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/flag_pc_update_regular_bit: cpu__decode__flag_pc_update_regular_0 * cpu__decode__flag_pc_update_regular_0 - cpu__decode__flag_pc_update_regular_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/cpu/decode/flag_pc_update_regular_0*/ intermediate_vals[11].clone(),
		/*intermediate_value/cpu/decode/flag_pc_update_regular_0*/ intermediate_vals[11].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/flag_pc_update_regular_0*/ intermediate_vals[11].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[5].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[5]*/ ctx[map::MM_COEFFICIENTS+5].clone() ) );
	}

	{
	// Constraint expression for cpu/decode/fp_update_regular_bit: cpu__decode__fp_update_regular_0 * cpu__decode__fp_update_regular_0 - cpu__decode__fp_update_regular_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/cpu/decode/fp_update_regular_0*/ intermediate_vals[14].clone(),
		/*intermediate_value/cpu/decode/fp_update_regular_0*/ intermediate_vals[14].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/fp_update_regular_0*/ intermediate_vals[14].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[6].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[6]*/ ctx[map::MM_COEFFICIENTS+6].clone() ) );
	}

	{
	// Constraint expression for cpu/operands/mem_dst_addr: column17_row8 + half_offset_size - (cpu__decode__opcode_rc__bit_0 * column19_row9 + (1 - cpu__decode__opcode_rc__bit_0) * column19_row1 + column19_row0).
	let mut val = prime_field::fadd(
	prime_field::fadd(/*column17_row8*/ ctx[map::MM_OODS_VALUES+104].clone(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone(),
			/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone() ),
			prime_field::fmul(
			prime_field::fadd(
				uint256_ops::get_uint256("1"),
				sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone()) ),
			/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone() ) ),
		/*column19_row0*/ ctx[map::MM_OODS_VALUES+135].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[7].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[7]*/ ctx[map::MM_COEFFICIENTS+7].clone() ) );
	}

	{
	// Constraint expression for cpu/operands/mem0_addr: column17_row4 + half_offset_size - (cpu__decode__opcode_rc__bit_1 * column19_row9 + (1 - cpu__decode__opcode_rc__bit_1) * column19_row1 + column19_row8).
	let mut val = prime_field::fadd(
	prime_field::fadd(/*column17_row4*/ ctx[map::MM_OODS_VALUES+100].clone(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_1*/ intermediate_vals[15].clone(),
			/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone() ),
			prime_field::fmul(
			prime_field::fadd(
				uint256_ops::get_uint256("1"),
				sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_1*/ intermediate_vals[15].clone()) ),
			/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone() ) ),
		/*column19_row8*/ ctx[map::MM_OODS_VALUES+143].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[8].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[8]*/ ctx[map::MM_COEFFICIENTS+8].clone() ) );
	}

	{
	// Constraint expression for cpu/operands/mem1_addr: column17_row12 + half_offset_size - (cpu__decode__opcode_rc__bit_2 * column17_row0 + cpu__decode__opcode_rc__bit_4 * column19_row1 + cpu__decode__opcode_rc__bit_3 * column19_row9 + cpu__decode__flag_op1_base_op0_0 * column17_row5 + column19_row4).
	let mut val = prime_field::fadd(
	prime_field::fadd(/*column17_row12*/ ctx[map::MM_OODS_VALUES+106].clone(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fadd(
			prime_field::fadd(
				prime_field::fmul(
				/*intermediate_value/cpu/decode/opcode_rc/bit_2*/ intermediate_vals[1].clone(),
				/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone() ),
				prime_field::fmul(
				/*intermediate_value/cpu/decode/opcode_rc/bit_4*/ intermediate_vals[2].clone(),
				/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone() ) ),
			prime_field::fmul(
				/*intermediate_value/cpu/decode/opcode_rc/bit_3*/ intermediate_vals[3].clone(),
				/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone() ) ),
			prime_field::fmul(
			/*intermediate_value/cpu/decode/flag_op1_base_op0_0*/ intermediate_vals[4].clone(),
			/*column17_row5*/ ctx[map::MM_OODS_VALUES+101].clone() ) ),
		/*column19_row4*/ ctx[map::MM_OODS_VALUES+139].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[9].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[9]*/ ctx[map::MM_COEFFICIENTS+9].clone() ) );
	}

	{
	// Constraint expression for cpu/operands/ops_mul: column19_row5 - column17_row5 * column17_row13.
	let mut val = prime_field::fadd(
	/*column19_row5*/ ctx[map::MM_OODS_VALUES+140].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(/*column17_row5*/ ctx[map::MM_OODS_VALUES+101].clone(), /*column17_row13*/ ctx[map::MM_OODS_VALUES+107].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[10].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[10]*/ ctx[map::MM_COEFFICIENTS+10].clone() ) );
	}

	{
	// Constraint expression for cpu/operands/res: (1 - cpu__decode__opcode_rc__bit_9) * column19_row13 - (cpu__decode__opcode_rc__bit_5 * (column17_row5 + column17_row13) + cpu__decode__opcode_rc__bit_6 * column19_row5 + cpu__decode__flag_res_op1_0 * column17_row13).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		prime_field::fadd(
		uint256_ops::get_uint256("1"),
		sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone()) ),
		/*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_5*/ intermediate_vals[5].clone(),
			prime_field::fadd(/*column17_row5*/ ctx[map::MM_OODS_VALUES+101].clone(), /*column17_row13*/ ctx[map::MM_OODS_VALUES+107].clone() ) ),
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_6*/ intermediate_vals[6].clone(),
			/*column19_row5*/ ctx[map::MM_OODS_VALUES+140].clone() ) ),
		prime_field::fmul(
			/*intermediate_value/cpu/decode/flag_res_op1_0*/ intermediate_vals[8].clone(),
			/*column17_row13*/ ctx[map::MM_OODS_VALUES+107].clone() ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[11].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[11]*/ ctx[map::MM_COEFFICIENTS+11].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_pc/tmp0: column19_row3 - cpu__decode__opcode_rc__bit_9 * column17_row9.
	let mut val = prime_field::fadd(
	/*column19_row3*/ ctx[map::MM_OODS_VALUES+138].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone(),
		/*column17_row9*/ ctx[map::MM_OODS_VALUES+105].clone() )) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[12].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[12]*/ ctx[map::MM_COEFFICIENTS+12].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_pc/tmp1: column19_row11 - column19_row3 * column19_row13.
	let mut val = prime_field::fadd(
	/*column19_row11*/ ctx[map::MM_OODS_VALUES+145].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(/*column19_row3*/ ctx[map::MM_OODS_VALUES+138].clone(), /*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone() )) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[13].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[13]*/ ctx[map::MM_COEFFICIENTS+13].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_pc/pc_cond_negative: (1 - cpu__decode__opcode_rc__bit_9) * column17_row16 + column19_row3 * (column17_row16 - (column17_row0 + column17_row13)) - (cpu__decode__flag_pc_update_regular_0 * npc_reg_0 + cpu__decode__opcode_rc__bit_7 * column19_row13 + cpu__decode__opcode_rc__bit_8 * (column17_row0 + column19_row13)).
	let mut val = prime_field::fadd(
	prime_field::fadd(
		prime_field::fmul(
		prime_field::fadd(
			uint256_ops::get_uint256("1"),
			sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone()) ),
		/*column17_row16*/ ctx[map::MM_OODS_VALUES+108].clone() ),
		prime_field::fmul(
		/*column19_row3*/ ctx[map::MM_OODS_VALUES+138].clone(),
		prime_field::fadd(
			/*column17_row16*/ ctx[map::MM_OODS_VALUES+108].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(), /*column17_row13*/ ctx[map::MM_OODS_VALUES+107].clone() )) ) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(
			/*intermediate_value/cpu/decode/flag_pc_update_regular_0*/ intermediate_vals[11].clone(),
			/*intermediate_value/npc_reg_0*/ intermediate_vals[16].clone() ),
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_7*/ intermediate_vals[9].clone(),
			/*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone() ) ),
		prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_8*/ intermediate_vals[10].clone(),
			prime_field::fadd(/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(), /*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone() ) ) )) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[14].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[14]*/ ctx[map::MM_COEFFICIENTS+14].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_pc/pc_cond_positive: (column19_row11 - cpu__decode__opcode_rc__bit_9) * (column17_row16 - npc_reg_0).
	let mut val = prime_field::fmul(
	prime_field::fadd(
		/*column19_row11*/ ctx[map::MM_OODS_VALUES+145].clone(),
		sub(prime_field::get_k_modulus(), /*intermediate_value/cpu/decode/opcode_rc/bit_9*/ intermediate_vals[7].clone()) ),
	prime_field::fadd(
		/*column17_row16*/ ctx[map::MM_OODS_VALUES+108].clone(),
		sub(prime_field::get_k_modulus(), /*intermediate_value/npc_reg_0*/ intermediate_vals[16].clone()) ) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[15].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[15]*/ ctx[map::MM_COEFFICIENTS+15].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_ap/ap_update: column19_row17 - (column19_row1 + cpu__decode__opcode_rc__bit_10 * column19_row13 + cpu__decode__opcode_rc__bit_11 + cpu__decode__opcode_rc__bit_12 * 2).
	let mut val = prime_field::fadd(
	/*column19_row17*/ ctx[map::MM_OODS_VALUES+149].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fadd(
			/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone(),
			prime_field::fmul(
				/*intermediate_value/cpu/decode/opcode_rc/bit_10*/ intermediate_vals[17].clone(),
				/*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone() ) ),
			/*intermediate_value/cpu/decode/opcode_rc/bit_11*/ intermediate_vals[18].clone() ),
		prime_field::fmul(/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone() , uint256_ops::get_uint256("2")  ) )) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[16].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[16]*/ ctx[map::MM_COEFFICIENTS+16].clone() ) );
	}

	{
	// Constraint expression for cpu/update_registers/update_fp/fp_update: column19_row25 - (cpu__decode__fp_update_regular_0 * column19_row9 + cpu__decode__opcode_rc__bit_13 * column17_row9 + cpu__decode__opcode_rc__bit_12 * (column19_row1 + 2)).
	let mut val = prime_field::fadd(
	/*column19_row25*/ ctx[map::MM_OODS_VALUES+151].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(
			/*intermediate_value/cpu/decode/fp_update_regular_0*/ intermediate_vals[14].clone(),
			/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone() ),
			prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_13*/ intermediate_vals[13].clone(),
			/*column17_row9*/ ctx[map::MM_OODS_VALUES+105].clone() ) ),
		prime_field::fmul(
			/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
			prime_field::fadd(/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone() , uint256_ops::get_uint256("2")  ) ) )) );

	// Numerator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= numerators[1].
	val = prime_field::fmul(val.clone(), numerators[1].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[17].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[17]*/ ctx[map::MM_COEFFICIENTS+17].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/call/push_fp: cpu__decode__opcode_rc__bit_12 * (column17_row9 - column19_row9).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
	prime_field::fadd(/*column17_row9*/ ctx[map::MM_OODS_VALUES+105].clone(), sub(prime_field::get_k_modulus(), /*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone()) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[18].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[18]*/ ctx[map::MM_COEFFICIENTS+18].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/call/push_pc: cpu__decode__opcode_rc__bit_12 * (column17_row5 - (column17_row0 + cpu__decode__opcode_rc__bit_2 + 1)).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
	prime_field::fadd(
		/*column17_row5*/ ctx[map::MM_OODS_VALUES+101].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
			prime_field::fadd(
			/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_2*/ intermediate_vals[1].clone() ),
			uint256_ops::get_uint256("1") )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[19].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[19]*/ ctx[map::MM_COEFFICIENTS+19].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/call/off0: cpu__decode__opcode_rc__bit_12 * (column19_row0 - half_offset_size).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
	prime_field::fadd(
		/*column19_row0*/ ctx[map::MM_OODS_VALUES+135].clone(),
		sub(prime_field::get_k_modulus(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone()) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[20].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[20]*/ ctx[map::MM_COEFFICIENTS+20].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/call/off1: cpu__decode__opcode_rc__bit_12 * (column19_row8 - (half_offset_size + 1)).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
	prime_field::fadd(
		/*column19_row8*/ ctx[map::MM_OODS_VALUES+143].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fadd(/*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone() , uint256_ops::get_uint256("1")  )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[21].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[21]*/ ctx[map::MM_COEFFICIENTS+21].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/call/flags: cpu__decode__opcode_rc__bit_12 * (cpu__decode__opcode_rc__bit_12 + cpu__decode__opcode_rc__bit_12 + 1 + 1 - (cpu__decode__opcode_rc__bit_0 + cpu__decode__opcode_rc__bit_1 + 4)).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
	prime_field::fadd(
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_12*/ intermediate_vals[12].clone() ),
			uint256_ops::get_uint256("1") ),
		uint256_ops::get_uint256("1") ),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
			prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_1*/ intermediate_vals[15].clone() ),
			uint256_ops::get_uint256("4") )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[22].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[22]*/ ctx[map::MM_COEFFICIENTS+22].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/ret/off0: cpu__decode__opcode_rc__bit_13 * (column19_row0 + 2 - half_offset_size).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_13*/ intermediate_vals[13].clone(),
	prime_field::fadd(
		prime_field::fadd(/*column19_row0*/ ctx[map::MM_OODS_VALUES+135].clone() , uint256_ops::get_uint256("2")  ),
		sub(prime_field::get_k_modulus(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone()) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[23].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[23]*/ ctx[map::MM_COEFFICIENTS+23].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/ret/off2: cpu__decode__opcode_rc__bit_13 * (column19_row4 + 1 - half_offset_size).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_13*/ intermediate_vals[13].clone(),
	prime_field::fadd(
		prime_field::fadd(/*column19_row4*/ ctx[map::MM_OODS_VALUES+139].clone() , uint256_ops::get_uint256("1")  ),
		sub(prime_field::get_k_modulus(), /*half_offset_size*/ ctx[map::MM_HALF_OFFSET_SIZE].clone()) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[24].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[24]*/ ctx[map::MM_COEFFICIENTS+24].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/ret/flags: cpu__decode__opcode_rc__bit_13 * (cpu__decode__opcode_rc__bit_7 + cpu__decode__opcode_rc__bit_0 + cpu__decode__opcode_rc__bit_3 + cpu__decode__flag_res_op1_0 - 4).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_13*/ intermediate_vals[13].clone(),
	prime_field::fadd(
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fadd(
			/*intermediate_value/cpu/decode/opcode_rc/bit_7*/ intermediate_vals[9].clone(),
			/*intermediate_value/cpu/decode/opcode_rc/bit_0*/ intermediate_vals[0].clone() ),
			/*intermediate_value/cpu/decode/opcode_rc/bit_3*/ intermediate_vals[3].clone() ),
		/*intermediate_value/cpu/decode/flag_res_op1_0*/ intermediate_vals[8].clone() ),
		sub(prime_field::get_k_modulus(), uint256_ops::get_uint256("4")) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[25].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[25]*/ ctx[map::MM_COEFFICIENTS+25].clone() ) );
	}

	{
	// Constraint expression for cpu/opcodes/assert_eq/assert_eq: cpu__decode__opcode_rc__bit_14 * (column17_row9 - column19_row13).
	let mut val = prime_field::fmul(
	/*intermediate_value/cpu/decode/opcode_rc/bit_14*/ intermediate_vals[19].clone(),
	prime_field::fadd(
		/*column17_row9*/ ctx[map::MM_OODS_VALUES+105].clone(),
		sub(prime_field::get_k_modulus(), /*column19_row13*/ ctx[map::MM_OODS_VALUES+147].clone()) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[26].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[26]*/ ctx[map::MM_COEFFICIENTS+26].clone() ) );
	}

	{
	// Constraint expression for initial_ap: column19_row1 - initial_ap.
	let mut val = prime_field::fadd(/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone(), sub(prime_field::get_k_modulus(), /*initial_ap*/ ctx[map::MM_INITIAL_AP].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[27].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[27]*/ ctx[map::MM_COEFFICIENTS+27].clone() ) );
	}

	{
	// Constraint expression for initial_fp: column19_row9 - initial_ap.
	let mut val = prime_field::fadd(/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone(), sub(prime_field::get_k_modulus(), /*initial_ap*/ ctx[map::MM_INITIAL_AP].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[28].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[28]*/ ctx[map::MM_COEFFICIENTS+28].clone() ) );
	}

	{
	// Constraint expression for initial_pc: column17_row0 - initial_pc.
	let mut val = prime_field::fadd(/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(), sub(prime_field::get_k_modulus(), /*initial_pc*/ ctx[map::MM_INITIAL_PC].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[29].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[29]*/ ctx[map::MM_COEFFICIENTS+29].clone() ) );
	}

	{
	// Constraint expression for final_ap: column19_row1 - final_ap.
	let mut val = prime_field::fadd(/*column19_row1*/ ctx[map::MM_OODS_VALUES+136].clone(), sub(prime_field::get_k_modulus(), /*final_ap*/ ctx[map::MM_FINAL_AP].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= denominator_invs[4].
	val = prime_field::fmul(val.clone(), denominator_inv[4].clone() );

	// res += val * coefficients[30].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[30]*/ ctx[map::MM_COEFFICIENTS+30].clone() ) );
	}

	{
	// Constraint expression for final_fp: column19_row9 - initial_ap.
	let mut val = prime_field::fadd(/*column19_row9*/ ctx[map::MM_OODS_VALUES+144].clone(), sub(prime_field::get_k_modulus(), /*initial_ap*/ ctx[map::MM_INITIAL_AP].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= denominator_invs[4].
	val = prime_field::fmul(val.clone(), denominator_inv[4].clone() );

	// res += val * coefficients[31].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[31]*/ ctx[map::MM_COEFFICIENTS+31].clone() ) );
	}

	{
	// Constraint expression for final_pc: column17_row0 - final_pc.
	let mut val = prime_field::fadd(/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone(), sub(prime_field::get_k_modulus(), /*final_pc*/ ctx[map::MM_FINAL_PC].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(16 * (trace_length / 16 - 1)).
	// val *= denominator_invs[4].
	val = prime_field::fmul(val.clone(), denominator_inv[4].clone() );

	// res += val * coefficients[32].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[32]*/ ctx[map::MM_COEFFICIENTS+32].clone() ) );
	}

	{
	// Constraint expression for memory/multi_column_perm/perm/init0: (memory/multi_column_perm/perm/interaction_elm - (column18_row0 + memory/multi_column_perm/hash_interaction_elm0 * column18_row1)) * column21_inter1_row0 + column17_row0 + memory/multi_column_perm/hash_interaction_elm0 * column17_row1 - memory/multi_column_perm/perm/interaction_elm.
	let mut val = prime_field::fadd(
	prime_field::fadd(
		prime_field::fadd(
		prime_field::fmul(
			prime_field::fadd(
			/*memory/multi_column_perm/perm/interaction_elm*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM].clone(),
			sub(
				prime_field::get_k_modulus(),
				prime_field::fadd(
				/*column18_row0*/ ctx[map::MM_OODS_VALUES+131].clone(),
				prime_field::fmul(
					/*memory/multi_column_perm/hash_interaction_elm0*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0].clone(),
					/*column18_row1*/ ctx[map::MM_OODS_VALUES+132].clone() ) )) ),
			/*column21_inter1_row0*/ ctx[map::MM_OODS_VALUES+196].clone() ),
		/*column17_row0*/ ctx[map::MM_OODS_VALUES+96].clone() ),
		prime_field::fmul(
		/*memory/multi_column_perm/hash_interaction_elm0*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0].clone(),
		/*column17_row1*/ ctx[map::MM_OODS_VALUES+97].clone() ) ),
	sub(prime_field::get_k_modulus(), /*memory/multi_column_perm/perm/interaction_elm*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[33].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[33]*/ ctx[map::MM_COEFFICIENTS+33].clone() ) );
	}

	{
	// Constraint expression for memory/multi_column_perm/perm/step0: (memory/multi_column_perm/perm/interaction_elm - (column18_row2 + memory/multi_column_perm/hash_interaction_elm0 * column18_row3)) * column21_inter1_row2 - (memory/multi_column_perm/perm/interaction_elm - (column17_row2 + memory/multi_column_perm/hash_interaction_elm0 * column17_row3)) * column21_inter1_row0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		prime_field::fadd(
		/*memory/multi_column_perm/perm/interaction_elm*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM].clone(),
		sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(
			/*column18_row2*/ ctx[map::MM_OODS_VALUES+133].clone(),
			prime_field::fmul(
				/*memory/multi_column_perm/hash_interaction_elm0*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0].clone(),
				/*column18_row3*/ ctx[map::MM_OODS_VALUES+134].clone() ) )) ),
		/*column21_inter1_row2*/ ctx[map::MM_OODS_VALUES+198].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*memory/multi_column_perm/perm/interaction_elm*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(
				/*column17_row2*/ ctx[map::MM_OODS_VALUES+98].clone(),
				prime_field::fmul(
				/*memory/multi_column_perm/hash_interaction_elm0*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0].clone(),
				/*column17_row3*/ ctx[map::MM_OODS_VALUES+99].clone() ) )) ),
		/*column21_inter1_row0*/ ctx[map::MM_OODS_VALUES+196].clone() )) );

	// Numerator: point - trace_generator^(2 * (trace_length / 2 - 1)).
	// val *= numerators[2].
	val = prime_field::fmul(val.clone(), numerators[2].clone() );
	// Denominator: point^(trace_length / 2) - 1.
	// val *= denominator_invs[5].
	val = prime_field::fmul(val.clone(), denominator_inv[5].clone() );

	// res += val * coefficients[34].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[34]*/ ctx[map::MM_COEFFICIENTS+34].clone() ) );
	}

	{
	// Constraint expression for memory/multi_column_perm/perm/last: column21_inter1_row0 - memory/multi_column_perm/perm/public_memory_prod.
	let mut val = prime_field::fadd(
	/*column21_inter1_row0*/ ctx[map::MM_OODS_VALUES+196].clone(),
	sub(prime_field::get_k_modulus(), /*memory/multi_column_perm/perm/public_memory_prod*/ ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__PUBLIC_MEMORY_PROD].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(2 * (trace_length / 2 - 1)).
	// val *= denominator_invs[6].
	val = prime_field::fmul(val.clone(), denominator_inv[6].clone() );

	// res += val * coefficients[35].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[35]*/ ctx[map::MM_COEFFICIENTS+35].clone() ) );
	}

	{
	// Constraint expression for memory/diff_is_bit: memory__address_diff_0 * memory__address_diff_0 - memory__address_diff_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/memory/address_diff_0*/ intermediate_vals[20].clone(),
		/*intermediate_value/memory/address_diff_0*/ intermediate_vals[20].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/memory/address_diff_0*/ intermediate_vals[20].clone()) );

	// Numerator: point - trace_generator^(2 * (trace_length / 2 - 1)).
	// val *= numerators[2].
	val = prime_field::fmul(val.clone(), numerators[2].clone() );
	// Denominator: point^(trace_length / 2) - 1.
	// val *= denominator_invs[5].
	val = prime_field::fmul(val.clone(), denominator_inv[5].clone() );

	// res += val * coefficients[36].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[36]*/ ctx[map::MM_COEFFICIENTS+36].clone() ) );
	}

	{
	// Constraint expression for memory/is_func: (memory__address_diff_0 - 1) * (column18_row1 - column18_row3).
	let mut val = prime_field::fmul(
	prime_field::fadd(/*intermediate_value/memory/address_diff_0*/ intermediate_vals[20].clone(), sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ),
	prime_field::fadd(/*column18_row1*/ ctx[map::MM_OODS_VALUES+132].clone(), sub(prime_field::get_k_modulus(), /*column18_row3*/ ctx[map::MM_OODS_VALUES+134].clone()) ) );

	// Numerator: point - trace_generator^(2 * (trace_length / 2 - 1)).
	// val *= numerators[2].
	val = prime_field::fmul(val.clone(), numerators[2].clone() );
	// Denominator: point^(trace_length / 2) - 1.
	// val *= denominator_invs[5].
	val = prime_field::fmul(val.clone(), denominator_inv[5].clone() );

	// res += val * coefficients[37].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[37]*/ ctx[map::MM_COEFFICIENTS+37].clone() ) );
	}

	{
	// Constraint expression for memory/initial_addr: column18_row0 - 1.
	let mut val = prime_field::fadd(/*column18_row0*/ ctx[map::MM_OODS_VALUES+131].clone(), sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[38].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[38]*/ ctx[map::MM_COEFFICIENTS+38].clone() ) );
	}

	{
	// Constraint expression for public_memory_addr_zero: column17_row2.
	let mut val = /*column17_row2*/ ctx[map::MM_OODS_VALUES+98].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8) - 1.
	// val *= denominator_invs[7].
	val = prime_field::fmul(val.clone(), denominator_inv[7].clone() );

	// res += val * coefficients[39].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[39]*/ ctx[map::MM_COEFFICIENTS+39].clone() ) );
	}

	{
	// Constraint expression for public_memory_value_zero: column17_row3.
	let mut val = /*column17_row3*/ ctx[map::MM_OODS_VALUES+99].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8) - 1.
	// val *= denominator_invs[7].
	val = prime_field::fmul(val.clone(), denominator_inv[7].clone() );

	// res += val * coefficients[40].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[40]*/ ctx[map::MM_COEFFICIENTS+40].clone() ) );
	}

	{
	// Constraint expression for rc16/perm/init0: (rc16/perm/interaction_elm - column19_row2) * column21_inter1_row1 + column19_row0 - rc16/perm/interaction_elm.
	let mut val = prime_field::fadd(
	prime_field::fadd(
		prime_field::fmul(
		prime_field::fadd(
			/*rc16/perm/interaction_elm*/ ctx[map::MM_RC16__PERM__INTERACTION_ELM].clone(),
			sub(prime_field::get_k_modulus(), /*column19_row2*/ ctx[map::MM_OODS_VALUES+137].clone()) ),
		/*column21_inter1_row1*/ ctx[map::MM_OODS_VALUES+197].clone() ),
		/*column19_row0*/ ctx[map::MM_OODS_VALUES+135].clone() ),
	sub(prime_field::get_k_modulus(), /*rc16/perm/interaction_elm*/ ctx[map::MM_RC16__PERM__INTERACTION_ELM].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[41].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[41]*/ ctx[map::MM_COEFFICIENTS+41].clone() ) );
	}

	{
	// Constraint expression for rc16/perm/step0: (rc16/perm/interaction_elm - column19_row6) * column21_inter1_row5 - (rc16/perm/interaction_elm - column19_row4) * column21_inter1_row1.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		prime_field::fadd(
		/*rc16/perm/interaction_elm*/ ctx[map::MM_RC16__PERM__INTERACTION_ELM].clone(),
		sub(prime_field::get_k_modulus(), /*column19_row6*/ ctx[map::MM_OODS_VALUES+141].clone()) ),
		/*column21_inter1_row5*/ ctx[map::MM_OODS_VALUES+199].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*rc16/perm/interaction_elm*/ ctx[map::MM_RC16__PERM__INTERACTION_ELM].clone(),
			sub(prime_field::get_k_modulus(), /*column19_row4*/ ctx[map::MM_OODS_VALUES+139].clone()) ),
		/*column21_inter1_row1*/ ctx[map::MM_OODS_VALUES+197].clone() )) );

	// Numerator: point - trace_generator^(4 * (trace_length / 4 - 1)).
	// val *= numerators[3].
	val = prime_field::fmul(val.clone(), numerators[3].clone() );
	// Denominator: point^(trace_length / 4) - 1.
	// val *= denominator_invs[8].
	val = prime_field::fmul(val.clone(), denominator_inv[8].clone() );

	// res += val * coefficients[42].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[42]*/ ctx[map::MM_COEFFICIENTS+42].clone() ) );
	}

	{
	// Constraint expression for rc16/perm/last: column21_inter1_row1 - rc16/perm/public_memory_prod.
	let mut val = prime_field::fadd(
	/*column21_inter1_row1*/ ctx[map::MM_OODS_VALUES+197].clone(),
	sub(prime_field::get_k_modulus(), /*rc16/perm/public_memory_prod*/ ctx[map::MM_RC16__PERM__PUBLIC_MEMORY_PROD].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(4 * (trace_length / 4 - 1)).
	// val *= denominator_invs[9].
	val = prime_field::fmul(val.clone(), denominator_inv[9].clone() );

	// res += val * coefficients[43].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[43]*/ ctx[map::MM_COEFFICIENTS+43].clone() ) );
	}

	{
	// Constraint expression for rc16/diff_is_bit: rc16__diff_0 * rc16__diff_0 - rc16__diff_0.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/rc16/diff_0*/ intermediate_vals[21].clone(),
		/*intermediate_value/rc16/diff_0*/ intermediate_vals[21].clone() ),
	sub(prime_field::get_k_modulus(), /*intermediate_value/rc16/diff_0*/ intermediate_vals[21].clone()) );

	// Numerator: point - trace_generator^(4 * (trace_length / 4 - 1)).
	// val *= numerators[3].
	val = prime_field::fmul(val.clone(), numerators[3].clone() );
	// Denominator: point^(trace_length / 4) - 1.
	// val *= denominator_invs[8].
	val = prime_field::fmul(val.clone(), denominator_inv[8].clone() );

	// res += val * coefficients[44].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[44]*/ ctx[map::MM_COEFFICIENTS+44].clone() ) );
	}

	{
	// Constraint expression for rc16/minimum: column19_row2 - rc_min.
	let mut val = prime_field::fadd(/*column19_row2*/ ctx[map::MM_OODS_VALUES+137].clone(), sub(prime_field::get_k_modulus(), /*rc_min*/ ctx[map::MM_RC_MIN].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[45].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[45]*/ ctx[map::MM_COEFFICIENTS+45].clone() ) );
	}

	{
	// Constraint expression for rc16/maximum: column19_row2 - rc_max.
	let mut val = prime_field::fadd(/*column19_row2*/ ctx[map::MM_OODS_VALUES+137].clone(), sub(prime_field::get_k_modulus(), /*rc_max*/ ctx[map::MM_RC_MAX].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - trace_generator^(4 * (trace_length / 4 - 1)).
	// val *= denominator_invs[9].
	val = prime_field::fmul(val.clone(), denominator_inv[9].clone() );

	// res += val * coefficients[46].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[46]*/ ctx[map::MM_COEFFICIENTS+46].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero: column11_row255 * (column4_row0 - (column4_row1 + column4_row1)).
	let mut val = prime_field::fmul(
	/*column11_row255*/ ctx[map::MM_OODS_VALUES+66].clone(),
	prime_field::fadd(
		/*column4_row0*/ ctx[map::MM_OODS_VALUES+27].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column4_row1*/ ctx[map::MM_OODS_VALUES+28].clone(), /*column4_row1*/ ctx[map::MM_OODS_VALUES+28].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[47].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[47]*/ ctx[map::MM_COEFFICIENTS+47].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0: column11_row255 * (column4_row1 - 3138550867693340381917894711603833208051177722232017256448 * column4_row192).
	let mut val = prime_field::fmul(
	/*column11_row255*/ ctx[map::MM_OODS_VALUES+66].clone(),
	prime_field::fadd(
		/*column4_row1*/ ctx[map::MM_OODS_VALUES+28].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
			uint256_ops::get_uint256("800000000000000000000000000000000000000000000000"),
			/*column4_row192*/ ctx[map::MM_OODS_VALUES+29].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[48].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[48]*/ ctx[map::MM_COEFFICIENTS+48].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192: column11_row255 - column15_row255 * (column4_row192 - (column4_row193 + column4_row193)).
	let mut val = prime_field::fadd(
	/*column11_row255*/ ctx[map::MM_OODS_VALUES+66].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column15_row255*/ ctx[map::MM_OODS_VALUES+86].clone(),
		prime_field::fadd(
			/*column4_row192*/ ctx[map::MM_OODS_VALUES+29].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column4_row193*/ ctx[map::MM_OODS_VALUES+30].clone(), /*column4_row193*/ ctx[map::MM_OODS_VALUES+30].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[49].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[49]*/ ctx[map::MM_COEFFICIENTS+49].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192: column15_row255 * (column4_row193 - 8 * column4_row196).
	let mut val = prime_field::fmul(
	/*column15_row255*/ ctx[map::MM_OODS_VALUES+86].clone(),
	prime_field::fadd(
		/*column4_row193*/ ctx[map::MM_OODS_VALUES+30].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("8"), /*column4_row196*/ ctx[map::MM_OODS_VALUES+31].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[50].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[50]*/ ctx[map::MM_COEFFICIENTS+50].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196: column15_row255 - (column4_row251 - (column4_row252 + column4_row252)) * (column4_row196 - (column4_row197 + column4_row197)).
	let mut val = prime_field::fadd(
	/*column15_row255*/ ctx[map::MM_OODS_VALUES+86].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*column4_row251*/ ctx[map::MM_OODS_VALUES+33].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column4_row252*/ ctx[map::MM_OODS_VALUES+34].clone(), /*column4_row252*/ ctx[map::MM_OODS_VALUES+34].clone() )) ),
		prime_field::fadd(
			/*column4_row196*/ ctx[map::MM_OODS_VALUES+31].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column4_row197*/ ctx[map::MM_OODS_VALUES+32].clone(), /*column4_row197*/ ctx[map::MM_OODS_VALUES+32].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[51].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[51]*/ ctx[map::MM_COEFFICIENTS+51].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196: (column4_row251 - (column4_row252 + column4_row252)) * (column4_row197 - 18014398509481984 * column4_row251).
	let mut val = prime_field::fmul(
	prime_field::fadd(
		/*column4_row251*/ ctx[map::MM_OODS_VALUES+33].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column4_row252*/ ctx[map::MM_OODS_VALUES+34].clone(), /*column4_row252*/ ctx[map::MM_OODS_VALUES+34].clone() )) ),
	prime_field::fadd(
		/*column4_row197*/ ctx[map::MM_OODS_VALUES+32].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("40000000000000"), /*column4_row251*/ ctx[map::MM_OODS_VALUES+33].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[52].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[52]*/ ctx[map::MM_COEFFICIENTS+52].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/booleanity_test: pedersen__hash0__ec_subset_sum__bit_0 * (pedersen__hash0__ec_subset_sum__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone(),
	prime_field::fadd(
		/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[53].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[53]*/ ctx[map::MM_COEFFICIENTS+53].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/bit_extraction_end: column4_row0.
	let mut val = /*column4_row0*/ ctx[map::MM_OODS_VALUES+27].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64).
	// val *= denominator_invs[11].
	val = prime_field::fmul(val.clone(), denominator_inv[11].clone() );

	// res += val * coefficients[54].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[54]*/ ctx[map::MM_COEFFICIENTS+54].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/zeros_tail: column4_row0.
	let mut val = /*column4_row0*/ ctx[map::MM_OODS_VALUES+27].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[12].
	val = prime_field::fmul(val.clone(), denominator_inv[12].clone() );

	// res += val * coefficients[55].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[55]*/ ctx[map::MM_COEFFICIENTS+55].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/add_points/slope: pedersen__hash0__ec_subset_sum__bit_0 * (column2_row0 - pedersen__points__y) - column3_row0 * (column1_row0 - pedersen__points__x).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone(),
		prime_field::fadd(
		/*column2_row0*/ ctx[map::MM_OODS_VALUES+21].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/y*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column3_row0*/ ctx[map::MM_OODS_VALUES+25].clone(),
		prime_field::fadd(
			/*column1_row0*/ ctx[map::MM_OODS_VALUES+16].clone(),
			sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[56].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[56]*/ ctx[map::MM_COEFFICIENTS+56].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/add_points/x: column3_row0 * column3_row0 - pedersen__hash0__ec_subset_sum__bit_0 * (column1_row0 + pedersen__points__x + column1_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column3_row0*/ ctx[map::MM_OODS_VALUES+25].clone(), /*column3_row0*/ ctx[map::MM_OODS_VALUES+25].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone(),
		prime_field::fadd(
			prime_field::fadd(
			/*column1_row0*/ ctx[map::MM_OODS_VALUES+16].clone(),
			/*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone() ),
			/*column1_row1*/ ctx[map::MM_OODS_VALUES+17].clone() ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[57].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[57]*/ ctx[map::MM_COEFFICIENTS+57].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/add_points/y: pedersen__hash0__ec_subset_sum__bit_0 * (column2_row0 + column2_row1) - column3_row0 * (column1_row0 - column1_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_0*/ intermediate_vals[22].clone(),
		prime_field::fadd(/*column2_row0*/ ctx[map::MM_OODS_VALUES+21].clone(), /*column2_row1*/ ctx[map::MM_OODS_VALUES+22].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column3_row0*/ ctx[map::MM_OODS_VALUES+25].clone(),
		prime_field::fadd(/*column1_row0*/ ctx[map::MM_OODS_VALUES+16].clone(), sub(prime_field::get_k_modulus(), /*column1_row1*/ ctx[map::MM_OODS_VALUES+17].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[58].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[58]*/ ctx[map::MM_COEFFICIENTS+58].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/copy_point/x: pedersen__hash0__ec_subset_sum__bit_neg_0 * (column1_row1 - column1_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_neg_0*/ intermediate_vals[23].clone(),
	prime_field::fadd(/*column1_row1*/ ctx[map::MM_OODS_VALUES+17].clone(), sub(prime_field::get_k_modulus(), /*column1_row0*/ ctx[map::MM_OODS_VALUES+16].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[59].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[59]*/ ctx[map::MM_COEFFICIENTS+59].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/ec_subset_sum/copy_point/y: pedersen__hash0__ec_subset_sum__bit_neg_0 * (column2_row1 - column2_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash0/ec_subset_sum/bit_neg_0*/ intermediate_vals[23].clone(),
	prime_field::fadd(/*column2_row1*/ ctx[map::MM_OODS_VALUES+22].clone(), sub(prime_field::get_k_modulus(), /*column2_row0*/ ctx[map::MM_OODS_VALUES+21].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[60].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[60]*/ ctx[map::MM_COEFFICIENTS+60].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/copy_point/x: column1_row256 - column1_row255.
	let mut val = prime_field::fadd(
	/*column1_row256*/ ctx[map::MM_OODS_VALUES+19].clone(),
	sub(prime_field::get_k_modulus(), /*column1_row255*/ ctx[map::MM_OODS_VALUES+18].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[61].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[61]*/ ctx[map::MM_COEFFICIENTS+61].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/copy_point/y: column2_row256 - column2_row255.
	let mut val = prime_field::fadd(
	/*column2_row256*/ ctx[map::MM_OODS_VALUES+24].clone(),
	sub(prime_field::get_k_modulus(), /*column2_row255*/ ctx[map::MM_OODS_VALUES+23].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[62].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[62]*/ ctx[map::MM_COEFFICIENTS+62].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/init/x: column1_row0 - pedersen/shift_point.x.
	let mut val = prime_field::fadd(
	/*column1_row0*/ ctx[map::MM_OODS_VALUES+16].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.x*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[63].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[63]*/ ctx[map::MM_COEFFICIENTS+63].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash0/init/y: column2_row0 - pedersen/shift_point.y.
	let mut val = prime_field::fadd(
	/*column2_row0*/ ctx[map::MM_OODS_VALUES+21].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.y*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_Y].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[64].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[64]*/ ctx[map::MM_COEFFICIENTS+64].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/last_one_is_zero: column3_row255 * (column8_row0 - (column8_row1 + column8_row1)).
	let mut val = prime_field::fmul(
	/*column3_row255*/ ctx[map::MM_OODS_VALUES+26].clone(),
	prime_field::fadd(
		/*column8_row0*/ ctx[map::MM_OODS_VALUES+47].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column8_row1*/ ctx[map::MM_OODS_VALUES+48].clone(), /*column8_row1*/ ctx[map::MM_OODS_VALUES+48].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[65].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[65]*/ ctx[map::MM_COEFFICIENTS+65].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones0: column3_row255 * (column8_row1 - 3138550867693340381917894711603833208051177722232017256448 * column8_row192).
	let mut val = prime_field::fmul(
	/*column3_row255*/ ctx[map::MM_OODS_VALUES+26].clone(),
	prime_field::fadd(
		/*column8_row1*/ ctx[map::MM_OODS_VALUES+48].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
			uint256_ops::get_uint256("800000000000000000000000000000000000000000000000"),
			/*column8_row192*/ ctx[map::MM_OODS_VALUES+49].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[66].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[66]*/ ctx[map::MM_COEFFICIENTS+66].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit192: column3_row255 - column7_row255 * (column8_row192 - (column8_row193 + column8_row193)).
	let mut val = prime_field::fadd(
	/*column3_row255*/ ctx[map::MM_OODS_VALUES+26].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column7_row255*/ ctx[map::MM_OODS_VALUES+46].clone(),
		prime_field::fadd(
			/*column8_row192*/ ctx[map::MM_OODS_VALUES+49].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column8_row193*/ ctx[map::MM_OODS_VALUES+50].clone(), /*column8_row193*/ ctx[map::MM_OODS_VALUES+50].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[67].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[67]*/ ctx[map::MM_COEFFICIENTS+67].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones192: column7_row255 * (column8_row193 - 8 * column8_row196).
	let mut val = prime_field::fmul(
	/*column7_row255*/ ctx[map::MM_OODS_VALUES+46].clone(),
	prime_field::fadd(
		/*column8_row193*/ ctx[map::MM_OODS_VALUES+50].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("8"), /*column8_row196*/ ctx[map::MM_OODS_VALUES+51].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[68].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[68]*/ ctx[map::MM_COEFFICIENTS+68].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit196: column7_row255 - (column8_row251 - (column8_row252 + column8_row252)) * (column8_row196 - (column8_row197 + column8_row197)).
	let mut val = prime_field::fadd(
	/*column7_row255*/ ctx[map::MM_OODS_VALUES+46].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*column8_row251*/ ctx[map::MM_OODS_VALUES+53].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column8_row252*/ ctx[map::MM_OODS_VALUES+54].clone(), /*column8_row252*/ ctx[map::MM_OODS_VALUES+54].clone() )) ),
		prime_field::fadd(
			/*column8_row196*/ ctx[map::MM_OODS_VALUES+51].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column8_row197*/ ctx[map::MM_OODS_VALUES+52].clone(), /*column8_row197*/ ctx[map::MM_OODS_VALUES+52].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[69].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[69]*/ ctx[map::MM_COEFFICIENTS+69].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones196: (column8_row251 - (column8_row252 + column8_row252)) * (column8_row197 - 18014398509481984 * column8_row251).
	let mut val = prime_field::fmul(
	prime_field::fadd(
		/*column8_row251*/ ctx[map::MM_OODS_VALUES+53].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column8_row252*/ ctx[map::MM_OODS_VALUES+54].clone(), /*column8_row252*/ ctx[map::MM_OODS_VALUES+54].clone() )) ),
	prime_field::fadd(
		/*column8_row197*/ ctx[map::MM_OODS_VALUES+52].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("40000000000000"), /*column8_row251*/ ctx[map::MM_OODS_VALUES+53].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[70].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[70]*/ ctx[map::MM_COEFFICIENTS+70].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/booleanity_test: pedersen__hash1__ec_subset_sum__bit_0 * (pedersen__hash1__ec_subset_sum__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone(),
	prime_field::fadd(
		/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[71].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[71]*/ ctx[map::MM_COEFFICIENTS+71].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/bit_extraction_end: column8_row0.
	let mut val = /*column8_row0*/ ctx[map::MM_OODS_VALUES+47].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64).
	// val *= denominator_invs[11].
	val = prime_field::fmul(val.clone(), denominator_inv[11].clone() );

	// res += val * coefficients[72].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[72]*/ ctx[map::MM_COEFFICIENTS+72].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/zeros_tail: column8_row0.
	let mut val = /*column8_row0*/ ctx[map::MM_OODS_VALUES+47].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[12].
	val = prime_field::fmul(val.clone(), denominator_inv[12].clone() );

	// res += val * coefficients[73].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[73]*/ ctx[map::MM_COEFFICIENTS+73].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/add_points/slope: pedersen__hash1__ec_subset_sum__bit_0 * (column6_row0 - pedersen__points__y) - column7_row0 * (column5_row0 - pedersen__points__x).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone(),
		prime_field::fadd(
		/*column6_row0*/ ctx[map::MM_OODS_VALUES+41].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/y*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column7_row0*/ ctx[map::MM_OODS_VALUES+45].clone(),
		prime_field::fadd(
			/*column5_row0*/ ctx[map::MM_OODS_VALUES+36].clone(),
			sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[74].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[74]*/ ctx[map::MM_COEFFICIENTS+74].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/add_points/x: column7_row0 * column7_row0 - pedersen__hash1__ec_subset_sum__bit_0 * (column5_row0 + pedersen__points__x + column5_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column7_row0*/ ctx[map::MM_OODS_VALUES+45].clone(), /*column7_row0*/ ctx[map::MM_OODS_VALUES+45].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone(),
		prime_field::fadd(
			prime_field::fadd(
			/*column5_row0*/ ctx[map::MM_OODS_VALUES+36].clone(),
			/*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone() ),
			/*column5_row1*/ ctx[map::MM_OODS_VALUES+37].clone() ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[75].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[75]*/ ctx[map::MM_COEFFICIENTS+75].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/add_points/y: pedersen__hash1__ec_subset_sum__bit_0 * (column6_row0 + column6_row1) - column7_row0 * (column5_row0 - column5_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_0*/ intermediate_vals[24].clone(),
		prime_field::fadd(/*column6_row0*/ ctx[map::MM_OODS_VALUES+41].clone(), /*column6_row1*/ ctx[map::MM_OODS_VALUES+42].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column7_row0*/ ctx[map::MM_OODS_VALUES+45].clone(),
		prime_field::fadd(/*column5_row0*/ ctx[map::MM_OODS_VALUES+36].clone(), sub(prime_field::get_k_modulus(), /*column5_row1*/ ctx[map::MM_OODS_VALUES+37].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[76].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[76]*/ ctx[map::MM_COEFFICIENTS+76].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/copy_point/x: pedersen__hash1__ec_subset_sum__bit_neg_0 * (column5_row1 - column5_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_neg_0*/ intermediate_vals[25].clone(),
	prime_field::fadd(/*column5_row1*/ ctx[map::MM_OODS_VALUES+37].clone(), sub(prime_field::get_k_modulus(), /*column5_row0*/ ctx[map::MM_OODS_VALUES+36].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[77].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[77]*/ ctx[map::MM_COEFFICIENTS+77].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/ec_subset_sum/copy_point/y: pedersen__hash1__ec_subset_sum__bit_neg_0 * (column6_row1 - column6_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash1/ec_subset_sum/bit_neg_0*/ intermediate_vals[25].clone(),
	prime_field::fadd(/*column6_row1*/ ctx[map::MM_OODS_VALUES+42].clone(), sub(prime_field::get_k_modulus(), /*column6_row0*/ ctx[map::MM_OODS_VALUES+41].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[78].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[78]*/ ctx[map::MM_COEFFICIENTS+78].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/copy_point/x: column5_row256 - column5_row255.
	let mut val = prime_field::fadd(
	/*column5_row256*/ ctx[map::MM_OODS_VALUES+39].clone(),
	sub(prime_field::get_k_modulus(), /*column5_row255*/ ctx[map::MM_OODS_VALUES+38].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[79].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[79]*/ ctx[map::MM_COEFFICIENTS+79].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/copy_point/y: column6_row256 - column6_row255.
	let mut val = prime_field::fadd(
	/*column6_row256*/ ctx[map::MM_OODS_VALUES+44].clone(),
	sub(prime_field::get_k_modulus(), /*column6_row255*/ ctx[map::MM_OODS_VALUES+43].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[80].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[80]*/ ctx[map::MM_COEFFICIENTS+80].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/init/x: column5_row0 - pedersen/shift_point.x.
	let mut val = prime_field::fadd(
	/*column5_row0*/ ctx[map::MM_OODS_VALUES+36].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.x*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[81].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[81]*/ ctx[map::MM_COEFFICIENTS+81].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash1/init/y: column6_row0 - pedersen/shift_point.y.
	let mut val = prime_field::fadd(
	/*column6_row0*/ ctx[map::MM_OODS_VALUES+41].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.y*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_Y].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[82].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[82]*/ ctx[map::MM_COEFFICIENTS+82].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/last_one_is_zero: column20_row145 * (column12_row0 - (column12_row1 + column12_row1)).
	let mut val = prime_field::fmul(
	/*column20_row145*/ ctx[map::MM_OODS_VALUES+181].clone(),
	prime_field::fadd(
		/*column12_row0*/ ctx[map::MM_OODS_VALUES+67].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column12_row1*/ ctx[map::MM_OODS_VALUES+68].clone(), /*column12_row1*/ ctx[map::MM_OODS_VALUES+68].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[83].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[83]*/ ctx[map::MM_COEFFICIENTS+83].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones0: column20_row145 * (column12_row1 - 3138550867693340381917894711603833208051177722232017256448 * column12_row192).
	let mut val = prime_field::fmul(
	/*column20_row145*/ ctx[map::MM_OODS_VALUES+181].clone(),
	prime_field::fadd(
		/*column12_row1*/ ctx[map::MM_OODS_VALUES+68].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
			uint256_ops::get_uint256("800000000000000000000000000000000000000000000000"),
			/*column12_row192*/ ctx[map::MM_OODS_VALUES+69].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[84].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[84]*/ ctx[map::MM_COEFFICIENTS+84].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit192: column20_row145 - column20_row17 * (column12_row192 - (column12_row193 + column12_row193)).
	let mut val = prime_field::fadd(
	/*column20_row145*/ ctx[map::MM_OODS_VALUES+181].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row17*/ ctx[map::MM_OODS_VALUES+171].clone(),
		prime_field::fadd(
			/*column12_row192*/ ctx[map::MM_OODS_VALUES+69].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column12_row193*/ ctx[map::MM_OODS_VALUES+70].clone(), /*column12_row193*/ ctx[map::MM_OODS_VALUES+70].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[85].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[85]*/ ctx[map::MM_COEFFICIENTS+85].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones192: column20_row17 * (column12_row193 - 8 * column12_row196).
	let mut val = prime_field::fmul(
	/*column20_row17*/ ctx[map::MM_OODS_VALUES+171].clone(),
	prime_field::fadd(
		/*column12_row193*/ ctx[map::MM_OODS_VALUES+70].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("8"), /*column12_row196*/ ctx[map::MM_OODS_VALUES+71].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[86].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[86]*/ ctx[map::MM_COEFFICIENTS+86].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit196: column20_row17 - (column12_row251 - (column12_row252 + column12_row252)) * (column12_row196 - (column12_row197 + column12_row197)).
	let mut val = prime_field::fadd(
	/*column20_row17*/ ctx[map::MM_OODS_VALUES+171].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*column12_row251*/ ctx[map::MM_OODS_VALUES+73].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column12_row252*/ ctx[map::MM_OODS_VALUES+74].clone(), /*column12_row252*/ ctx[map::MM_OODS_VALUES+74].clone() )) ),
		prime_field::fadd(
			/*column12_row196*/ ctx[map::MM_OODS_VALUES+71].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column12_row197*/ ctx[map::MM_OODS_VALUES+72].clone(), /*column12_row197*/ ctx[map::MM_OODS_VALUES+72].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[87].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[87]*/ ctx[map::MM_COEFFICIENTS+87].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones196: (column12_row251 - (column12_row252 + column12_row252)) * (column12_row197 - 18014398509481984 * column12_row251).
	let mut val = prime_field::fmul(
	prime_field::fadd(
		/*column12_row251*/ ctx[map::MM_OODS_VALUES+73].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column12_row252*/ ctx[map::MM_OODS_VALUES+74].clone(), /*column12_row252*/ ctx[map::MM_OODS_VALUES+74].clone() )) ),
	prime_field::fadd(
		/*column12_row197*/ ctx[map::MM_OODS_VALUES+72].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("40000000000000"), /*column12_row251*/ ctx[map::MM_OODS_VALUES+73].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[88].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[88]*/ ctx[map::MM_COEFFICIENTS+88].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/booleanity_test: pedersen__hash2__ec_subset_sum__bit_0 * (pedersen__hash2__ec_subset_sum__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone(),
	prime_field::fadd(
		/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[89].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[89]*/ ctx[map::MM_COEFFICIENTS+89].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/bit_extraction_end: column12_row0.
	let mut val = /*column12_row0*/ ctx[map::MM_OODS_VALUES+67].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64).
	// val *= denominator_invs[11].
	val = prime_field::fmul(val.clone(), denominator_inv[11].clone() );

	// res += val * coefficients[90].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[90]*/ ctx[map::MM_COEFFICIENTS+90].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/zeros_tail: column12_row0.
	let mut val = /*column12_row0*/ ctx[map::MM_OODS_VALUES+67].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[12].
	val = prime_field::fmul(val.clone(), denominator_inv[12].clone() );

	// res += val * coefficients[91].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[91]*/ ctx[map::MM_COEFFICIENTS+91].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/add_points/slope: pedersen__hash2__ec_subset_sum__bit_0 * (column10_row0 - pedersen__points__y) - column11_row0 * (column9_row0 - pedersen__points__x).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone(),
		prime_field::fadd(
		/*column10_row0*/ ctx[map::MM_OODS_VALUES+61].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/y*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column11_row0*/ ctx[map::MM_OODS_VALUES+65].clone(),
		prime_field::fadd(
			/*column9_row0*/ ctx[map::MM_OODS_VALUES+56].clone(),
			sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[92].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[92]*/ ctx[map::MM_COEFFICIENTS+92].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/add_points/x: column11_row0 * column11_row0 - pedersen__hash2__ec_subset_sum__bit_0 * (column9_row0 + pedersen__points__x + column9_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column11_row0*/ ctx[map::MM_OODS_VALUES+65].clone(), /*column11_row0*/ ctx[map::MM_OODS_VALUES+65].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone(),
		prime_field::fadd(
			prime_field::fadd(
			/*column9_row0*/ ctx[map::MM_OODS_VALUES+56].clone(),
			/*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone() ),
			/*column9_row1*/ ctx[map::MM_OODS_VALUES+57].clone() ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[93].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[93]*/ ctx[map::MM_COEFFICIENTS+93].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/add_points/y: pedersen__hash2__ec_subset_sum__bit_0 * (column10_row0 + column10_row1) - column11_row0 * (column9_row0 - column9_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_0*/ intermediate_vals[26].clone(),
		prime_field::fadd(/*column10_row0*/ ctx[map::MM_OODS_VALUES+61].clone(), /*column10_row1*/ ctx[map::MM_OODS_VALUES+62].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column11_row0*/ ctx[map::MM_OODS_VALUES+65].clone(),
		prime_field::fadd(/*column9_row0*/ ctx[map::MM_OODS_VALUES+56].clone(), sub(prime_field::get_k_modulus(), /*column9_row1*/ ctx[map::MM_OODS_VALUES+57].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[94].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[94]*/ ctx[map::MM_COEFFICIENTS+94].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/copy_point/x: pedersen__hash2__ec_subset_sum__bit_neg_0 * (column9_row1 - column9_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_neg_0*/ intermediate_vals[27].clone(),
	prime_field::fadd(/*column9_row1*/ ctx[map::MM_OODS_VALUES+57].clone(), sub(prime_field::get_k_modulus(), /*column9_row0*/ ctx[map::MM_OODS_VALUES+56].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[95].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[95]*/ ctx[map::MM_COEFFICIENTS+95].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/ec_subset_sum/copy_point/y: pedersen__hash2__ec_subset_sum__bit_neg_0 * (column10_row1 - column10_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash2/ec_subset_sum/bit_neg_0*/ intermediate_vals[27].clone(),
	prime_field::fadd(/*column10_row1*/ ctx[map::MM_OODS_VALUES+62].clone(), sub(prime_field::get_k_modulus(), /*column10_row0*/ ctx[map::MM_OODS_VALUES+61].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[96].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[96]*/ ctx[map::MM_COEFFICIENTS+96].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/copy_point/x: column9_row256 - column9_row255.
	let mut val = prime_field::fadd(
	/*column9_row256*/ ctx[map::MM_OODS_VALUES+59].clone(),
	sub(prime_field::get_k_modulus(), /*column9_row255*/ ctx[map::MM_OODS_VALUES+58].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[97].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[97]*/ ctx[map::MM_COEFFICIENTS+97].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/copy_point/y: column10_row256 - column10_row255.
	let mut val = prime_field::fadd(
	/*column10_row256*/ ctx[map::MM_OODS_VALUES+64].clone(),
	sub(prime_field::get_k_modulus(), /*column10_row255*/ ctx[map::MM_OODS_VALUES+63].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[98].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[98]*/ ctx[map::MM_COEFFICIENTS+98].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/init/x: column9_row0 - pedersen/shift_point.x.
	let mut val = prime_field::fadd(
	/*column9_row0*/ ctx[map::MM_OODS_VALUES+56].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.x*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[99].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[99]*/ ctx[map::MM_COEFFICIENTS+99].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash2/init/y: column10_row0 - pedersen/shift_point.y.
	let mut val = prime_field::fadd(
	/*column10_row0*/ ctx[map::MM_OODS_VALUES+61].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.y*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_Y].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[100].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[100]*/ ctx[map::MM_COEFFICIENTS+100].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/last_one_is_zero: column20_row209 * (column16_row0 - (column16_row1 + column16_row1)).
	let mut val = prime_field::fmul(
	/*column20_row209*/ ctx[map::MM_OODS_VALUES+182].clone(),
	prime_field::fadd(
		/*column16_row0*/ ctx[map::MM_OODS_VALUES+87].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column16_row1*/ ctx[map::MM_OODS_VALUES+88].clone(), /*column16_row1*/ ctx[map::MM_OODS_VALUES+88].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[101].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[101]*/ ctx[map::MM_COEFFICIENTS+101].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones0: column20_row209 * (column16_row1 - 3138550867693340381917894711603833208051177722232017256448 * column16_row192).
	let mut val = prime_field::fmul(
	/*column20_row209*/ ctx[map::MM_OODS_VALUES+182].clone(),
	prime_field::fadd(
		/*column16_row1*/ ctx[map::MM_OODS_VALUES+88].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
			uint256_ops::get_uint256("800000000000000000000000000000000000000000000000"),
			/*column16_row192*/ ctx[map::MM_OODS_VALUES+89].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[102].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[102]*/ ctx[map::MM_COEFFICIENTS+102].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit192: column20_row209 - column20_row81 * (column16_row192 - (column16_row193 + column16_row193)).
	let mut val = prime_field::fadd(
	/*column20_row209*/ ctx[map::MM_OODS_VALUES+182].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row81*/ ctx[map::MM_OODS_VALUES+180].clone(),
		prime_field::fadd(
			/*column16_row192*/ ctx[map::MM_OODS_VALUES+89].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column16_row193*/ ctx[map::MM_OODS_VALUES+90].clone(), /*column16_row193*/ ctx[map::MM_OODS_VALUES+90].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[103].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[103]*/ ctx[map::MM_COEFFICIENTS+103].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones192: column20_row81 * (column16_row193 - 8 * column16_row196).
	let mut val = prime_field::fmul(
	/*column20_row81*/ ctx[map::MM_OODS_VALUES+180].clone(),
	prime_field::fadd(
		/*column16_row193*/ ctx[map::MM_OODS_VALUES+90].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul(uint256_ops::get_uint256("8"), /*column16_row196*/ ctx[map::MM_OODS_VALUES+91].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[104].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[104]*/ ctx[map::MM_COEFFICIENTS+104].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit196: column20_row81 - (column16_row251 - (column16_row252 + column16_row252)) * (column16_row196 - (column16_row197 + column16_row197)).
	let mut val = prime_field::fadd(
	/*column20_row81*/ ctx[map::MM_OODS_VALUES+180].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(
			/*column16_row251*/ ctx[map::MM_OODS_VALUES+93].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column16_row252*/ ctx[map::MM_OODS_VALUES+94].clone(), /*column16_row252*/ ctx[map::MM_OODS_VALUES+94].clone() )) ),
		prime_field::fadd(
			/*column16_row196*/ ctx[map::MM_OODS_VALUES+91].clone(),
			sub(
			prime_field::get_k_modulus(),
			prime_field::fadd(/*column16_row197*/ ctx[map::MM_OODS_VALUES+92].clone(), /*column16_row197*/ ctx[map::MM_OODS_VALUES+92].clone() )) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[105].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[105]*/ ctx[map::MM_COEFFICIENTS+105].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones196: (column16_row251 - (column16_row252 + column16_row252)) * (column16_row197 - 18014398509481984 * column16_row251).
	let mut val = prime_field::fmul(
	prime_field::fadd(
		/*column16_row251*/ ctx[map::MM_OODS_VALUES+93].clone(),
		sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(/*column16_row252*/ ctx[map::MM_OODS_VALUES+94].clone(), /*column16_row252*/ ctx[map::MM_OODS_VALUES+94].clone() )) ),
	prime_field::fadd(
		/*column16_row197*/ ctx[map::MM_OODS_VALUES+92].clone(),
		sub(prime_field::get_k_modulus(), prime_field::fmul( uint256_ops::get_uint256("40000000000000"), /*column16_row251*/ ctx[map::MM_OODS_VALUES+93].clone() )) ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[106].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[106]*/ ctx[map::MM_COEFFICIENTS+106].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/booleanity_test: pedersen__hash3__ec_subset_sum__bit_0 * (pedersen__hash3__ec_subset_sum__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone(),
	prime_field::fadd(
		/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[107].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[107]*/ ctx[map::MM_COEFFICIENTS+107].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/bit_extraction_end: column16_row0.
	let mut val = /*column16_row0*/ ctx[map::MM_OODS_VALUES+87].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64).
	// val *= denominator_invs[11].
	val = prime_field::fmul(val.clone(), denominator_inv[11].clone() );

	// res += val * coefficients[108].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[108]*/ ctx[map::MM_COEFFICIENTS+108].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/zeros_tail: column16_row0.
	let mut val = /*column16_row0*/ ctx[map::MM_OODS_VALUES+87].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[12].
	val = prime_field::fmul(val.clone(), denominator_inv[12].clone() );

	// res += val * coefficients[109].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[109]*/ ctx[map::MM_COEFFICIENTS+109].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/add_points/slope: pedersen__hash3__ec_subset_sum__bit_0 * (column14_row0 - pedersen__points__y) - column15_row0 * (column13_row0 - pedersen__points__x).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone(),
		prime_field::fadd(
		/*column14_row0*/ ctx[map::MM_OODS_VALUES+81].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/y*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column15_row0*/ ctx[map::MM_OODS_VALUES+85].clone(),
		prime_field::fadd(
			/*column13_row0*/ ctx[map::MM_OODS_VALUES+76].clone(),
			sub(prime_field::get_k_modulus(), /*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[110].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[110]*/ ctx[map::MM_COEFFICIENTS+110].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/add_points/x: column15_row0 * column15_row0 - pedersen__hash3__ec_subset_sum__bit_0 * (column13_row0 + pedersen__points__x + column13_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column15_row0*/ ctx[map::MM_OODS_VALUES+85].clone(), /*column15_row0*/ ctx[map::MM_OODS_VALUES+85].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone(),
		prime_field::fadd(
			prime_field::fadd(
			/*column13_row0*/ ctx[map::MM_OODS_VALUES+76].clone(),
			/*periodic_column/pedersen/points/x*/ ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone() ),
			/*column13_row1*/ ctx[map::MM_OODS_VALUES+77].clone() ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[111].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[111]*/ ctx[map::MM_COEFFICIENTS+111].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/add_points/y: pedersen__hash3__ec_subset_sum__bit_0 * (column14_row0 + column14_row1) - column15_row0 * (column13_row0 - column13_row1).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_0*/ intermediate_vals[28].clone(),
		prime_field::fadd(/*column14_row0*/ ctx[map::MM_OODS_VALUES+81].clone(), /*column14_row1*/ ctx[map::MM_OODS_VALUES+82].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column15_row0*/ ctx[map::MM_OODS_VALUES+85].clone(),
		prime_field::fadd(/*column13_row0*/ ctx[map::MM_OODS_VALUES+76].clone(), sub(prime_field::get_k_modulus(), /*column13_row1*/ ctx[map::MM_OODS_VALUES+77].clone()) ) )) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[112].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[112]*/ ctx[map::MM_COEFFICIENTS+112].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/copy_point/x: pedersen__hash3__ec_subset_sum__bit_neg_0 * (column13_row1 - column13_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_neg_0*/ intermediate_vals[29].clone(),
	prime_field::fadd(/*column13_row1*/ ctx[map::MM_OODS_VALUES+77].clone(), sub(prime_field::get_k_modulus(), /*column13_row0*/ ctx[map::MM_OODS_VALUES+76].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[113].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[113]*/ ctx[map::MM_COEFFICIENTS+113].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/ec_subset_sum/copy_point/y: pedersen__hash3__ec_subset_sum__bit_neg_0 * (column14_row1 - column14_row0).
	let mut val = prime_field::fmul(
	/*intermediate_value/pedersen/hash3/ec_subset_sum/bit_neg_0*/ intermediate_vals[29].clone(),
	prime_field::fadd(/*column14_row1*/ ctx[map::MM_OODS_VALUES+82].clone(), sub(prime_field::get_k_modulus(), /*column14_row0*/ ctx[map::MM_OODS_VALUES+81].clone()) ) );

	// Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[4].
	val = prime_field::fmul(val.clone(), numerators[4].clone() );
	// Denominator: point^trace_length - 1.
	// val *= denominator_invs[0].
	val = prime_field::fmul(val.clone(), denominator_inv[0].clone() );

	// res += val * coefficients[114].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[114]*/ ctx[map::MM_COEFFICIENTS+114].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/copy_point/x: column13_row256 - column13_row255.
	let mut val = prime_field::fadd(
	/*column13_row256*/ ctx[map::MM_OODS_VALUES+79].clone(),
	sub(prime_field::get_k_modulus(), /*column13_row255*/ ctx[map::MM_OODS_VALUES+78].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[115].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[115]*/ ctx[map::MM_COEFFICIENTS+115].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/copy_point/y: column14_row256 - column14_row255.
	let mut val = prime_field::fadd(
	/*column14_row256*/ ctx[map::MM_OODS_VALUES+84].clone(),
	sub(prime_field::get_k_modulus(), /*column14_row255*/ ctx[map::MM_OODS_VALUES+83].clone()) );

	// Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2).
	// val *= numerators[5].
	val = prime_field::fmul(val.clone(), numerators[5].clone() );
	// Denominator: point^(trace_length / 256) - 1.
	// val *= denominator_invs[10].
	val = prime_field::fmul(val.clone(), denominator_inv[10].clone() );

	// res += val * coefficients[116].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[116]*/ ctx[map::MM_COEFFICIENTS+116].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/init/x: column13_row0 - pedersen/shift_point.x.
	let mut val = prime_field::fadd(
	/*column13_row0*/ ctx[map::MM_OODS_VALUES+76].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.x*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[117].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[117]*/ ctx[map::MM_COEFFICIENTS+117].clone() ) );
	}

	{
	// Constraint expression for pedersen/hash3/init/y: column14_row0 - pedersen/shift_point.y.
	let mut val = prime_field::fadd(
	/*column14_row0*/ ctx[map::MM_OODS_VALUES+81].clone(),
	sub(prime_field::get_k_modulus(), /*pedersen/shift_point.y*/ ctx[map::MM_PEDERSEN__SHIFT_POINT_Y].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[118].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[118]*/ ctx[map::MM_COEFFICIENTS+118].clone() ) );
	}

	{
	// Constraint expression for pedersen/input0_value0: column17_row7 - column4_row0.
	let mut val = prime_field::fadd(/*column17_row7*/ ctx[map::MM_OODS_VALUES+103].clone(), sub(prime_field::get_k_modulus(), /*column4_row0*/ ctx[map::MM_OODS_VALUES+27].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[119].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[119]*/ ctx[map::MM_COEFFICIENTS+119].clone() ) );
	}

	{
	// Constraint expression for pedersen/input0_value1: column17_row135 - column8_row0.
	let mut val = prime_field::fadd(
	/*column17_row135*/ ctx[map::MM_OODS_VALUES+118].clone(),
	sub(prime_field::get_k_modulus(), /*column8_row0*/ ctx[map::MM_OODS_VALUES+47].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[120].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[120]*/ ctx[map::MM_COEFFICIENTS+120].clone() ) );
	}

	{
	// Constraint expression for pedersen/input0_value2: column17_row263 - column12_row0.
	let mut val = prime_field::fadd(
	/*column17_row263*/ ctx[map::MM_OODS_VALUES+122].clone(),
	sub(prime_field::get_k_modulus(), /*column12_row0*/ ctx[map::MM_OODS_VALUES+67].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[121].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[121]*/ ctx[map::MM_COEFFICIENTS+121].clone() ) );
	}

	{
	// Constraint expression for pedersen/input0_value3: column17_row391 - column16_row0.
	let mut val = prime_field::fadd(
	/*column17_row391*/ ctx[map::MM_OODS_VALUES+125].clone(),
	sub(prime_field::get_k_modulus(), /*column16_row0*/ ctx[map::MM_OODS_VALUES+87].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[122].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[122]*/ ctx[map::MM_COEFFICIENTS+122].clone() ) );
	}

	{
	// Constraint expression for pedersen/input0_addr: column17_row134 - (column17_row38 + 1).
	let mut val = prime_field::fadd(
	/*column17_row134*/ ctx[map::MM_OODS_VALUES+117].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row38*/ ctx[map::MM_OODS_VALUES+111].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: point - trace_generator^(128 * (trace_length / 128 - 1)).
	// val *= numerators[6].
	val = prime_field::fmul(val.clone(), numerators[6].clone() );
	// Denominator: point^(trace_length / 128) - 1.
	// val *= denominator_invs[14].
	val = prime_field::fmul(val.clone(), denominator_inv[14].clone() );

	// res += val * coefficients[123].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[123]*/ ctx[map::MM_COEFFICIENTS+123].clone() ) );
	}

	{
	// Constraint expression for pedersen/init_addr: column17_row6 - initial_pedersen_addr.
	let mut val = prime_field::fadd(
	/*column17_row6*/ ctx[map::MM_OODS_VALUES+102].clone(),
	sub(prime_field::get_k_modulus(), /*initial_pedersen_addr*/ ctx[map::MM_INITIAL_PEDERSEN_ADDR].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[124].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[124]*/ ctx[map::MM_COEFFICIENTS+124].clone() ) );
	}

	{
	// Constraint expression for pedersen/input1_value0: column17_row71 - column4_row256.
	let mut val = prime_field::fadd(
	/*column17_row71*/ ctx[map::MM_OODS_VALUES+114].clone(),
	sub(prime_field::get_k_modulus(), /*column4_row256*/ ctx[map::MM_OODS_VALUES+35].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[125].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[125]*/ ctx[map::MM_COEFFICIENTS+125].clone() ) );
	}

	{
	// Constraint expression for pedersen/input1_value1: column17_row199 - column8_row256.
	let mut val = prime_field::fadd(
	/*column17_row199*/ ctx[map::MM_OODS_VALUES+120].clone(),
	sub(prime_field::get_k_modulus(), /*column8_row256*/ ctx[map::MM_OODS_VALUES+55].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[126].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[126]*/ ctx[map::MM_COEFFICIENTS+126].clone() ) );
	}

	{
	// Constraint expression for pedersen/input1_value2: column17_row327 - column12_row256.
	let mut val = prime_field::fadd(
	/*column17_row327*/ ctx[map::MM_OODS_VALUES+124].clone(),
	sub(prime_field::get_k_modulus(), /*column12_row256*/ ctx[map::MM_OODS_VALUES+75].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[127].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[127]*/ ctx[map::MM_COEFFICIENTS+127].clone() ) );
	}

	{
	// Constraint expression for pedersen/input1_value3: column17_row455 - column16_row256.
	let mut val = prime_field::fadd(
	/*column17_row455*/ ctx[map::MM_OODS_VALUES+127].clone(),
	sub(prime_field::get_k_modulus(), /*column16_row256*/ ctx[map::MM_OODS_VALUES+95].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[128].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[128]*/ ctx[map::MM_COEFFICIENTS+128].clone() ) );
	}

	{
	// Constraint expression for pedersen/input1_addr: column17_row70 - (column17_row6 + 1).
	let mut val = prime_field::fadd(
	/*column17_row70*/ ctx[map::MM_OODS_VALUES+113].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row6*/ ctx[map::MM_OODS_VALUES+102].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 128) - 1.
	// val *= denominator_invs[14].
	val = prime_field::fmul(val.clone(), denominator_inv[14].clone() );

	// res += val * coefficients[129].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[129]*/ ctx[map::MM_COEFFICIENTS+129].clone() ) );
	}

	{
	// Constraint expression for pedersen/output_value0: column17_row39 - column1_row511.
	let mut val = prime_field::fadd(
	/*column17_row39*/ ctx[map::MM_OODS_VALUES+112].clone(),
	sub(prime_field::get_k_modulus(), /*column1_row511*/ ctx[map::MM_OODS_VALUES+20].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[130].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[130]*/ ctx[map::MM_COEFFICIENTS+130].clone() ) );
	}

	{
	// Constraint expression for pedersen/output_value1: column17_row167 - column5_row511.
	let mut val = prime_field::fadd(
	/*column17_row167*/ ctx[map::MM_OODS_VALUES+119].clone(),
	sub(prime_field::get_k_modulus(), /*column5_row511*/ ctx[map::MM_OODS_VALUES+40].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[131].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[131]*/ ctx[map::MM_COEFFICIENTS+131].clone() ) );
	}

	{
	// Constraint expression for pedersen/output_value2: column17_row295 - column9_row511.
	let mut val = prime_field::fadd(
	/*column17_row295*/ ctx[map::MM_OODS_VALUES+123].clone(),
	sub(prime_field::get_k_modulus(), /*column9_row511*/ ctx[map::MM_OODS_VALUES+60].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[132].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[132]*/ ctx[map::MM_COEFFICIENTS+132].clone() ) );
	}

	{
	// Constraint expression for pedersen/output_value3: column17_row423 - column13_row511.
	let mut val = prime_field::fadd(
	/*column17_row423*/ ctx[map::MM_OODS_VALUES+126].clone(),
	sub(prime_field::get_k_modulus(), /*column13_row511*/ ctx[map::MM_OODS_VALUES+80].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 512) - 1.
	// val *= denominator_invs[13].
	val = prime_field::fmul(val.clone(), denominator_inv[13].clone() );

	// res += val * coefficients[133].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[133]*/ ctx[map::MM_COEFFICIENTS+133].clone() ) );
	}

	{
	// Constraint expression for pedersen/output_addr: column17_row38 - (column17_row70 + 1).
	let mut val = prime_field::fadd(
	/*column17_row38*/ ctx[map::MM_OODS_VALUES+111].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row70*/ ctx[map::MM_OODS_VALUES+113].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 128) - 1.
	// val *= denominator_invs[14].
	val = prime_field::fmul(val.clone(), denominator_inv[14].clone() );

	// res += val * coefficients[134].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[134]*/ ctx[map::MM_COEFFICIENTS+134].clone() ) );
	}

	{
	// Constraint expression for rc_builtin/value: rc_builtin__value7_0 - column17_row103.
	let mut val = prime_field::fadd(
	/*intermediate_value/rc_builtin/value7_0*/ intermediate_vals[37].clone(),
	sub(prime_field::get_k_modulus(), /*column17_row103*/ ctx[map::MM_OODS_VALUES+116].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 128) - 1.
	// val *= denominator_invs[14].
	val = prime_field::fmul(val.clone(), denominator_inv[14].clone() );

	// res += val * coefficients[135].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[135]*/ ctx[map::MM_COEFFICIENTS+135].clone() ) );
	}

	{
	// Constraint expression for rc_builtin/addr_step: column17_row230 - (column17_row102 + 1).
	let mut val = prime_field::fadd(
	/*column17_row230*/ ctx[map::MM_OODS_VALUES+121].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row102*/ ctx[map::MM_OODS_VALUES+115].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: point - trace_generator^(128 * (trace_length / 128 - 1)).
	// val *= numerators[6].
	val = prime_field::fmul(val.clone(), numerators[6].clone() );
	// Denominator: point^(trace_length / 128) - 1.
	// val *= denominator_invs[14].
	val = prime_field::fmul(val.clone(), denominator_inv[14].clone() );

	// res += val * coefficients[136].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[136]*/ ctx[map::MM_COEFFICIENTS+136].clone() ) );
	}

	{
	// Constraint expression for rc_builtin/init_addr: column17_row102 - initial_rc_addr.
	let mut val = prime_field::fadd(
	/*column17_row102*/ ctx[map::MM_OODS_VALUES+115].clone(),
	sub(prime_field::get_k_modulus(), /*initial_rc_addr*/ ctx[map::MM_INITIAL_RC_ADDR].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[137].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[137]*/ ctx[map::MM_COEFFICIENTS+137].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/doubling_key/slope: ecdsa__signature0__doubling_key__x_squared + ecdsa__signature0__doubling_key__x_squared + ecdsa__signature0__doubling_key__x_squared + ecdsa/sig_config.alpha - (column19_row15 + column19_row15) * column20_row0.
	let mut val = prime_field::fadd(
	prime_field::fadd(
		prime_field::fadd(
		prime_field::fadd(
			/*intermediate_value/ecdsa/signature0/doubling_key/x_squared*/ intermediate_vals[38].clone(),
			/*intermediate_value/ecdsa/signature0/doubling_key/x_squared*/ intermediate_vals[38].clone() ),
		/*intermediate_value/ecdsa/signature0/doubling_key/x_squared*/ intermediate_vals[38].clone() ),
		/*ecdsa/sig_config.alpha*/ ctx[map::MM_ECDSA__SIG_CONFIG_ALPHA].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		prime_field::fadd(/*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone(), /*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone() ),
		/*column20_row0*/ ctx[map::MM_OODS_VALUES+162].clone() )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[138].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[138]*/ ctx[map::MM_COEFFICIENTS+138].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/doubling_key/x: column20_row0 * column20_row0 - (column19_row7 + column19_row7 + column19_row23).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row0*/ ctx[map::MM_OODS_VALUES+162].clone(), /*column20_row0*/ ctx[map::MM_OODS_VALUES+162].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(/*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone() ),
		/*column19_row23*/ ctx[map::MM_OODS_VALUES+150].clone() )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[139].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[139]*/ ctx[map::MM_COEFFICIENTS+139].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/doubling_key/y: column19_row15 + column19_row31 - column20_row0 * (column19_row7 - column19_row23).
	let mut val = prime_field::fadd(
	prime_field::fadd(/*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone(), /*column19_row31*/ ctx[map::MM_OODS_VALUES+153].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row0*/ ctx[map::MM_OODS_VALUES+162].clone(),
		prime_field::fadd(
			/*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone(),
			sub(prime_field::get_k_modulus(), /*column19_row23*/ ctx[map::MM_OODS_VALUES+150].clone()) ) )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[140].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[140]*/ ctx[map::MM_COEFFICIENTS+140].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/booleanity_test: ecdsa__signature0__exponentiate_generator__bit_0 * (ecdsa__signature0__exponentiate_generator__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone(),
	prime_field::fadd(
		/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[141].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[141]*/ ctx[map::MM_COEFFICIENTS+141].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/bit_extraction_end: column20_row30.
	let mut val = /*column20_row30*/ ctx[map::MM_OODS_VALUES+176].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - trace_generator^(251 * trace_length / 256).
	// val *= denominator_invs[16].
	val = prime_field::fmul(val.clone(), denominator_inv[16].clone() );

	// res += val * coefficients[142].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[142]*/ ctx[map::MM_COEFFICIENTS+142].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/zeros_tail: column20_row30.
	let mut val = /*column20_row30*/ ctx[map::MM_OODS_VALUES+176].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[17].
	val = prime_field::fmul(val.clone(), denominator_inv[17].clone() );

	// res += val * coefficients[143].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[143]*/ ctx[map::MM_COEFFICIENTS+143].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/add_points/slope: ecdsa__signature0__exponentiate_generator__bit_0 * (column20_row22 - ecdsa__generator_points__y) - column20_row14 * (column20_row6 - ecdsa__generator_points__x).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone(),
		prime_field::fadd(
		/*column20_row22*/ ctx[map::MM_OODS_VALUES+174].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/ecdsa/generator_points/y*/ ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__Y].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row14*/ ctx[map::MM_OODS_VALUES+170].clone(),
		prime_field::fadd(
			/*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone(),
			sub(prime_field::get_k_modulus(), /*periodic_column/ecdsa/generator_points/x*/ ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X].clone()) ) )) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[144].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[144]*/ ctx[map::MM_COEFFICIENTS+144].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/add_points/x: column20_row14 * column20_row14 - ecdsa__signature0__exponentiate_generator__bit_0 * (column20_row6 + ecdsa__generator_points__x + column20_row38).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row14*/ ctx[map::MM_OODS_VALUES+170].clone(), /*column20_row14*/ ctx[map::MM_OODS_VALUES+170].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone(),
		prime_field::fadd(
			prime_field::fadd(
			/*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone(),
			/*periodic_column/ecdsa/generator_points/x*/ ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X].clone() ),
			/*column20_row38*/ ctx[map::MM_OODS_VALUES+177].clone() ) )) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[145].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[145]*/ ctx[map::MM_COEFFICIENTS+145].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/add_points/y: ecdsa__signature0__exponentiate_generator__bit_0 * (column20_row22 + column20_row54) - column20_row14 * (column20_row6 - column20_row38).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_0*/ intermediate_vals[39].clone(),
		prime_field::fadd(/*column20_row22*/ ctx[map::MM_OODS_VALUES+174].clone(), /*column20_row54*/ ctx[map::MM_OODS_VALUES+178].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row14*/ ctx[map::MM_OODS_VALUES+170].clone(),
		prime_field::fadd(
			/*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone(),
			sub(prime_field::get_k_modulus(), /*column20_row38*/ ctx[map::MM_OODS_VALUES+177].clone()) ) )) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[146].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[146]*/ ctx[map::MM_COEFFICIENTS+146].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/add_points/x_diff_inv: column20_row1 * (column20_row6 - ecdsa__generator_points__x) - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*column20_row1*/ ctx[map::MM_OODS_VALUES+163].clone(),
		prime_field::fadd(
		/*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone(),
		sub(prime_field::get_k_modulus(), /*periodic_column/ecdsa/generator_points/x*/ ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X].clone()) ) ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[147].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[147]*/ ctx[map::MM_COEFFICIENTS+147].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/copy_point/x: ecdsa__signature0__exponentiate_generator__bit_neg_0 * (column20_row38 - column20_row6).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_neg_0*/ intermediate_vals[40].clone(),
	prime_field::fadd(
		/*column20_row38*/ ctx[map::MM_OODS_VALUES+177].clone(),
		sub(prime_field::get_k_modulus(), /*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone()) ) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[148].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[148]*/ ctx[map::MM_COEFFICIENTS+148].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_generator/copy_point/y: ecdsa__signature0__exponentiate_generator__bit_neg_0 * (column20_row54 - column20_row22).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_generator/bit_neg_0*/ intermediate_vals[40].clone(),
	prime_field::fadd(
		/*column20_row54*/ ctx[map::MM_OODS_VALUES+178].clone(),
		sub(prime_field::get_k_modulus(), /*column20_row22*/ ctx[map::MM_OODS_VALUES+174].clone()) ) );

	// Numerator: point^(trace_length / 8192) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[8].
	val = prime_field::fmul(val.clone(), numerators[8].clone() );
	// Denominator: point^(trace_length / 32) - 1.
	// val *= denominator_invs[15].
	val = prime_field::fmul(val.clone(), denominator_inv[15].clone() );

	// res += val * coefficients[149].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[149]*/ ctx[map::MM_COEFFICIENTS+149].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/booleanity_test: ecdsa__signature0__exponentiate_key__bit_0 * (ecdsa__signature0__exponentiate_key__bit_0 - 1).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone(),
	prime_field::fadd(
		/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone(),
		sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) ) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[150].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[150]*/ ctx[map::MM_COEFFICIENTS+150].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/bit_extraction_end: column20_row2.
	let mut val = /*column20_row2*/ ctx[map::MM_OODS_VALUES+164].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 4096) - trace_generator^(251 * trace_length / 256).
	// val *= denominator_invs[18].
	val = prime_field::fmul(val.clone(), denominator_inv[18].clone() );

	// res += val * coefficients[151].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[151]*/ ctx[map::MM_COEFFICIENTS+151].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/zeros_tail: column20_row2.
	let mut val = /*column20_row2*/ ctx[map::MM_OODS_VALUES+164].clone();

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= denominator_invs[19].
	val = prime_field::fmul(val.clone(), denominator_inv[19].clone() );

	// res += val * coefficients[152].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[152]*/ ctx[map::MM_COEFFICIENTS+152].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/add_points/slope: ecdsa__signature0__exponentiate_key__bit_0 * (column20_row4 - column19_row15) - column20_row12 * (column20_row8 - column19_row7).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone(),
		prime_field::fadd(
		/*column20_row4*/ ctx[map::MM_OODS_VALUES+165].clone(),
		sub(prime_field::get_k_modulus(), /*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone()) ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row12*/ ctx[map::MM_OODS_VALUES+169].clone(),
		prime_field::fadd(/*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone(), sub(prime_field::get_k_modulus(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone()) ) )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[153].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[153]*/ ctx[map::MM_COEFFICIENTS+153].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/add_points/x: column20_row12 * column20_row12 - ecdsa__signature0__exponentiate_key__bit_0 * (column20_row8 + column19_row7 + column20_row24).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row12*/ ctx[map::MM_OODS_VALUES+169].clone(), /*column20_row12*/ ctx[map::MM_OODS_VALUES+169].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone(),
		prime_field::fadd(
			prime_field::fadd(/*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone() ),
			/*column20_row24*/ ctx[map::MM_OODS_VALUES+175].clone() ) )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[154].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[154]*/ ctx[map::MM_COEFFICIENTS+154].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/add_points/y: ecdsa__signature0__exponentiate_key__bit_0 * (column20_row4 + column20_row20) - column20_row12 * (column20_row8 - column20_row24).
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_0*/ intermediate_vals[41].clone(),
		prime_field::fadd(/*column20_row4*/ ctx[map::MM_OODS_VALUES+165].clone(), /*column20_row20*/ ctx[map::MM_OODS_VALUES+173].clone() ) ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row12*/ ctx[map::MM_OODS_VALUES+169].clone(),
		prime_field::fadd(
			/*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone(),
			sub(prime_field::get_k_modulus(), /*column20_row24*/ ctx[map::MM_OODS_VALUES+175].clone()) ) )) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[155].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[155]*/ ctx[map::MM_COEFFICIENTS+155].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/add_points/x_diff_inv: column20_row10 * (column20_row8 - column19_row7) - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*column20_row10*/ ctx[map::MM_OODS_VALUES+168].clone(),
		prime_field::fadd(/*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone(), sub(prime_field::get_k_modulus(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone()) ) ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[156].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[156]*/ ctx[map::MM_COEFFICIENTS+156].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/copy_point/x: ecdsa__signature0__exponentiate_key__bit_neg_0 * (column20_row24 - column20_row8).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_neg_0*/ intermediate_vals[42].clone(),
	prime_field::fadd(
		/*column20_row24*/ ctx[map::MM_OODS_VALUES+175].clone(),
		sub(prime_field::get_k_modulus(), /*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone()) ) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[157].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[157]*/ ctx[map::MM_COEFFICIENTS+157].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/exponentiate_key/copy_point/y: ecdsa__signature0__exponentiate_key__bit_neg_0 * (column20_row20 - column20_row4).
	let mut val = prime_field::fmul(
	/*intermediate_value/ecdsa/signature0/exponentiate_key/bit_neg_0*/ intermediate_vals[42].clone(),
	prime_field::fadd(
		/*column20_row20*/ ctx[map::MM_OODS_VALUES+173].clone(),
		sub(prime_field::get_k_modulus(), /*column20_row4*/ ctx[map::MM_OODS_VALUES+165].clone()) ) );

	// Numerator: point^(trace_length / 4096) - trace_generator^(255 * trace_length / 256).
	// val *= numerators[7].
	val = prime_field::fmul(val.clone(), numerators[7].clone() );
	// Denominator: point^(trace_length / 16) - 1.
	// val *= denominator_invs[2].
	val = prime_field::fmul(val.clone(), denominator_inv[2].clone() );

	// res += val * coefficients[158].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[158]*/ ctx[map::MM_COEFFICIENTS+158].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/init_gen/x: column20_row6 - ecdsa/sig_config.shift_point.x.
	let mut val = prime_field::fadd(
	/*column20_row6*/ ctx[map::MM_OODS_VALUES+166].clone(),
	sub(prime_field::get_k_modulus(), /*ecdsa/sig_config.shift_point.x*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[159].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[159]*/ ctx[map::MM_COEFFICIENTS+159].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/init_gen/y: column20_row22 + ecdsa/sig_config.shift_point.y.
	let mut val = prime_field::fadd(
	/*column20_row22*/ ctx[map::MM_OODS_VALUES+174].clone(),
	/*ecdsa/sig_config.shift_point.y*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y].clone() );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[160].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[160]*/ ctx[map::MM_COEFFICIENTS+160].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/init_key/x: column20_row8 - ecdsa/sig_config.shift_point.x.
	let mut val = prime_field::fadd(
	/*column20_row8*/ ctx[map::MM_OODS_VALUES+167].clone(),
	sub(prime_field::get_k_modulus(), /*ecdsa/sig_config.shift_point.x*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 4096) - 1.
	// val *= denominator_invs[21].
	val = prime_field::fmul(val.clone(), denominator_inv[21].clone() );

	// res += val * coefficients[161].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[161]*/ ctx[map::MM_COEFFICIENTS+161].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/init_key/y: column20_row4 - ecdsa/sig_config.shift_point.y.
	let mut val = prime_field::fadd(
	/*column20_row4*/ ctx[map::MM_OODS_VALUES+165].clone(),
	sub(prime_field::get_k_modulus(), /*ecdsa/sig_config.shift_point.y*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 4096) - 1.
	// val *= denominator_invs[21].
	val = prime_field::fmul(val.clone(), denominator_inv[21].clone() );

	// res += val * coefficients[162].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[162]*/ ctx[map::MM_COEFFICIENTS+162].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/add_results/slope: column20_row8182 - (column20_row4084 + column20_row8161 * (column20_row8166 - column20_row4088)).
	let mut val = prime_field::fadd(
	/*column20_row8182*/ ctx[map::MM_OODS_VALUES+193].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		/*column20_row4084*/ ctx[map::MM_OODS_VALUES+184].clone(),
		prime_field::fmul(
			/*column20_row8161*/ ctx[map::MM_OODS_VALUES+188].clone(),
			prime_field::fadd(
			/*column20_row8166*/ ctx[map::MM_OODS_VALUES+189].clone(),
			sub(prime_field::get_k_modulus(), /*column20_row4088*/ ctx[map::MM_OODS_VALUES+185].clone()) ) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[163].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[163]*/ ctx[map::MM_COEFFICIENTS+163].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/add_results/x: column20_row8161 * column20_row8161 - (column20_row8166 + column20_row4088 + column19_row4103).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row8161*/ ctx[map::MM_OODS_VALUES+188].clone(), /*column20_row8161*/ ctx[map::MM_OODS_VALUES+188].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(/*column20_row8166*/ ctx[map::MM_OODS_VALUES+189].clone(), /*column20_row4088*/ ctx[map::MM_OODS_VALUES+185].clone() ),
		/*column19_row4103*/ ctx[map::MM_OODS_VALUES+160].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[164].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[164]*/ ctx[map::MM_COEFFICIENTS+164].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/add_results/y: column20_row8182 + column19_row4111 - column20_row8161 * (column20_row8166 - column19_row4103).
	let mut val = prime_field::fadd(
	prime_field::fadd(/*column20_row8182*/ ctx[map::MM_OODS_VALUES+193].clone(), /*column19_row4111*/ ctx[map::MM_OODS_VALUES+161].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row8161*/ ctx[map::MM_OODS_VALUES+188].clone(),
		prime_field::fadd(
			/*column20_row8166*/ ctx[map::MM_OODS_VALUES+189].clone(),
			sub(prime_field::get_k_modulus(), /*column19_row4103*/ ctx[map::MM_OODS_VALUES+160].clone()) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[165].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[165]*/ ctx[map::MM_COEFFICIENTS+165].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/add_results/x_diff_inv: column20_row8174 * (column20_row8166 - column20_row4088) - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*column20_row8174*/ ctx[map::MM_OODS_VALUES+190].clone(),
		prime_field::fadd(
		/*column20_row8166*/ ctx[map::MM_OODS_VALUES+189].clone(),
		sub(prime_field::get_k_modulus(), /*column20_row4088*/ ctx[map::MM_OODS_VALUES+185].clone()) ) ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[166].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[166]*/ ctx[map::MM_COEFFICIENTS+166].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/extract_r/slope: column20_row8180 + ecdsa/sig_config.shift_point.y - column20_row4092 * (column20_row8184 - ecdsa/sig_config.shift_point.x).
	let mut val = prime_field::fadd(
	prime_field::fadd(
		/*column20_row8180*/ ctx[map::MM_OODS_VALUES+192].clone(),
		/*ecdsa/sig_config.shift_point.y*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(
		/*column20_row4092*/ ctx[map::MM_OODS_VALUES+187].clone(),
		prime_field::fadd(
			/*column20_row8184*/ ctx[map::MM_OODS_VALUES+194].clone(),
			sub(prime_field::get_k_modulus(), /*ecdsa/sig_config.shift_point.x*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone()) ) )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[167].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[167]*/ ctx[map::MM_COEFFICIENTS+167].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/extract_r/x: column20_row4092 * column20_row4092 - (column20_row8184 + ecdsa/sig_config.shift_point.x + column20_row2).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row4092*/ ctx[map::MM_OODS_VALUES+187].clone(), /*column20_row4092*/ ctx[map::MM_OODS_VALUES+187].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			/*column20_row8184*/ ctx[map::MM_OODS_VALUES+194].clone(),
			/*ecdsa/sig_config.shift_point.x*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone() ),
		/*column20_row2*/ ctx[map::MM_OODS_VALUES+164].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[168].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[168]*/ ctx[map::MM_COEFFICIENTS+168].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/extract_r/x_diff_inv: column20_row8188 * (column20_row8184 - ecdsa/sig_config.shift_point.x) - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(
		/*column20_row8188*/ ctx[map::MM_OODS_VALUES+195].clone(),
		prime_field::fadd(
		/*column20_row8184*/ ctx[map::MM_OODS_VALUES+194].clone(),
		sub(prime_field::get_k_modulus(), /*ecdsa/sig_config.shift_point.x*/ ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone()) ) ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[169].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[169]*/ ctx[map::MM_COEFFICIENTS+169].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/z_nonzero: column20_row30 * column20_row4080 - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row30*/ ctx[map::MM_OODS_VALUES+176].clone(), /*column20_row4080*/ ctx[map::MM_OODS_VALUES+183].clone() ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[170].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[170]*/ ctx[map::MM_COEFFICIENTS+170].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/r_and_w_nonzero: column20_row2 * column20_row4090 - 1.
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column20_row2*/ ctx[map::MM_OODS_VALUES+164].clone(), /*column20_row4090*/ ctx[map::MM_OODS_VALUES+186].clone() ),
	sub(prime_field::get_k_modulus() , uint256_ops::get_uint256("1")  ) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 4096) - 1.
	// val *= denominator_invs[21].
	val = prime_field::fmul(val.clone(), denominator_inv[21].clone() );

	// res += val * coefficients[171].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[171]*/ ctx[map::MM_COEFFICIENTS+171].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/q_on_curve/x_squared: column20_row8176 - column19_row7 * column19_row7.
	let mut val = prime_field::fadd(
	/*column20_row8176*/ ctx[map::MM_OODS_VALUES+191].clone(),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fmul(/*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[172].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[172]*/ ctx[map::MM_COEFFICIENTS+172].clone() ) );
	}

	{
	// Constraint expression for ecdsa/signature0/q_on_curve/on_curve: column19_row15 * column19_row15 - (column19_row7 * column20_row8176 + ecdsa/sig_config.alpha * column19_row7 + ecdsa/sig_config.beta).
	let mut val = prime_field::fadd(
	prime_field::fmul(/*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone(), /*column19_row15*/ ctx[map::MM_OODS_VALUES+148].clone() ),
	sub(
		prime_field::get_k_modulus(),
		prime_field::fadd(
		prime_field::fadd(
			prime_field::fmul(/*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone(), /*column20_row8176*/ ctx[map::MM_OODS_VALUES+191].clone() ),
			prime_field::fmul(/*ecdsa/sig_config.alpha*/ ctx[map::MM_ECDSA__SIG_CONFIG_ALPHA].clone(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone() ) ),
		/*ecdsa/sig_config.beta*/ ctx[map::MM_ECDSA__SIG_CONFIG_BETA].clone() )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[173].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[173]*/ ctx[map::MM_COEFFICIENTS+173].clone() ) );
	}

	{
	// Constraint expression for ecdsa/init_addr: column17_row22 - initial_ecdsa_addr.
	let mut val = prime_field::fadd(
	/*column17_row22*/ ctx[map::MM_OODS_VALUES+109].clone(),
	sub(prime_field::get_k_modulus(), /*initial_ecdsa_addr*/ ctx[map::MM_INITIAL_ECDSA_ADDR].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point - 1.
	// val *= denominator_invs[3].
	val = prime_field::fmul(val.clone(), denominator_inv[3].clone() );

	// res += val * coefficients[174].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[174]*/ ctx[map::MM_COEFFICIENTS+174].clone() ) );
	}

	{
	// Constraint expression for ecdsa/message_addr: column17_row4118 - (column17_row22 + 1).
	let mut val = prime_field::fadd(
	/*column17_row4118*/ ctx[map::MM_OODS_VALUES+128].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row22*/ ctx[map::MM_OODS_VALUES+109].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[175].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[175]*/ ctx[map::MM_COEFFICIENTS+175].clone() ) );
	}

	{
	// Constraint expression for ecdsa/pubkey_addr: column17_row8214 - (column17_row4118 + 1).
	let mut val = prime_field::fadd(
	/*column17_row8214*/ ctx[map::MM_OODS_VALUES+130].clone(),
	sub(prime_field::get_k_modulus(), prime_field::fadd(/*column17_row4118*/ ctx[map::MM_OODS_VALUES+128].clone() , uint256_ops::get_uint256("1")  )) );

	// Numerator: point - trace_generator^(8192 * (trace_length / 8192 - 1)).
	// val *= numerators[9].
	val = prime_field::fmul(val.clone(), numerators[9].clone() );
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[176].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[176]*/ ctx[map::MM_COEFFICIENTS+176].clone() ) );
	}

	{
	// Constraint expression for ecdsa/message_value0: column17_row4119 - column20_row30.
	let mut val = prime_field::fadd(
	/*column17_row4119*/ ctx[map::MM_OODS_VALUES+129].clone(),
	sub(prime_field::get_k_modulus(), /*column20_row30*/ ctx[map::MM_OODS_VALUES+176].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[177].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[177]*/ ctx[map::MM_COEFFICIENTS+177].clone() ) );
	}

	{
	// Constraint expression for ecdsa/pubkey_value0: column17_row23 - column19_row7.
	let mut val = prime_field::fadd(
	/*column17_row23*/ ctx[map::MM_OODS_VALUES+110].clone(),
	sub(prime_field::get_k_modulus(), /*column19_row7*/ ctx[map::MM_OODS_VALUES+142].clone()) );

	// Numerator: 1.
	// val *= 1.
	// val = prime_field::fmul(val.clone(), 1 ).
	// Denominator: point^(trace_length / 8192) - 1.
	// val *= denominator_invs[20].
	val = prime_field::fmul(val.clone(), denominator_inv[20].clone() );

	// res += val * coefficients[178].
	res = prime_field::fadd(res.clone(),
				prime_field::fmul(val.clone(), /*coefficients[178]*/ ctx[map::MM_COEFFICIENTS+178].clone() ) );
	}












	return res;
}

fn sub(a: Uint256, b: Uint256) -> Uint256 {
	return a-b;
}