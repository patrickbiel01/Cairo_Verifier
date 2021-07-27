use num256::uint256::Uint256 as Uint256;

use crate::uint256_ops;
use crate::prime_field;
use crate::horner_eval::horner_eval;
use crate::memory_map as map;
use crate::verify_merkle::{verify_merkle, get_hash_mask};


static FRI_MAX_FRI_STEP: usize = 4;
static MAX_COSET_SIZE: usize = 2usize.pow(FRI_MAX_FRI_STEP as u32);
static FRI_GROUP_GEN: &str = "5ec467b88826aba4537602d514425f3b0bdf467bbf302458337c45f6021e539";
static FRI_GROUP_SIZE: usize = MAX_COSET_SIZE;
static FRI_CTX_TO_COSET_EVALUATIONS_OFFSET: usize = 0;
static FRI_CTX_TO_FRI_GROUP_OFFSET: usize = FRI_GROUP_SIZE;
static FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET: usize = FRI_CTX_TO_FRI_GROUP_OFFSET + FRI_GROUP_SIZE;
static FRI_CTX_SIZE: usize  = FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET + (FRI_GROUP_SIZE / 2);


/* -------------------------
	FRI Protocol Code
		Verifies that the compositional polynomial sent by the prover 
		is valid in poly-log(degree) time
 --------------------------- */


// Verify the last set of coefficents sent from the prover
pub fn verify_last_layer(ctx: & mut Vec<Uint256>, n_points: usize) {

	let fri_last_deg_bound = uint256_ops::to_usize( &ctx[map::MM_FRI_LAST_LAYER_DEG_BOUND] );
	let group_order_minus_1 = uint256_ops::from_usize(fri_last_deg_bound) * ctx[map::MM_BLOW_UP_FACTOR].clone() - uint256_ops::get_uint256("0");
	let coeff_start = uint256_ops::to_usize( &ctx[map::MM_FRI_LAST_LAYER_PTR] );

	for i in 0..n_points {
		
		let mut point = ctx[map::MM_FRI_QUEUE + 3*i + 2].clone();

		// Invert point using inverse(point) == fpow(point, ord(point) - 1)
		point = prime_field::fpow(&point, &group_order_minus_1); 

		assert!( horner_eval(coeff_start, point, fri_last_deg_bound, ctx) ==  ctx[map::MM_FRI_QUEUE + 3*i + 1] ); //Bad Last layer value

	}
	
}


/*
	Verifies FRI layers.

	Upon entry and every time we pass through the "if (index < layerSize)" condition,
	ctx[mmFriQueue:] holds an array of triplets (query index, FRI value, FRI inversed point), i.e.
		ctx[mmFriQueue::3] holds query indices.
		ctx[mmFriQueue + 1::3] holds the input for the next layer.
		ctx[mmFriQueue + 2::3] holds the inverses of the evaluation points:
		ctx[mmFriQueue + 3*i + 2] = inverse(
			fpow(layerGenerator,  bitReverse(ctx[mmFriQueue + 3*i], logLayerSize)
		)
*/
pub fn fri_verify_layers(ctx: & mut Vec<Uint256>) {

    assert!(map::MAX_SUPPORTED_MAX_FRI_STEP == FRI_MAX_FRI_STEP); //Incosistent MAX_FRI_STEP between MemoryMap.sol and FriLayer.sol

	let fri_ctx = map::MM_FRI_CTX;
	init_fri_groups(fri_ctx, ctx);

	let channel_idx = map::MM_CHANNEL;
	let merkle_queue_idx = map::MM_MERKLE_QUEUE;
	let fri_queue = map::MM_FRI_QUEUE;

	let mut fri_step = 1;
	let mut n_live_queries = uint256_ops::to_usize(&ctx[map::MM_N_UNIQUE_QUERIES]);

	// Add 0 at the end of the queries array to avoid empty array check in readNextElment.
    ctx[map::MM_FRI_QUERIES_DELIMITER] = uint256_ops::get_uint256("0");

	// Rather than converting all the values from Montgomery to standard form,
	// we can just pretend that the values are in standard form but all
	// the committed polynomials are multiplied by MontgomeryR.
	//
	// The values in the proof are already multiplied by MontgomeryR,
	// but the inputs from the OODS oracle need to be fixed.
	 for i in 0..n_live_queries {
        ctx[map::MM_FRI_QUEUE + 3*i + 1] = prime_field::fmul( ctx[map::MM_FRI_QUEUE + 3*i + 1].clone(), prime_field::get_k_montgomery_r() );
    }

	let fri_steps: Vec<Uint256> = get_fri_steps(ctx);
	let n_fri_steps = fri_steps.len();

	while fri_step < n_fri_steps {
		let fri_coset_size = uint256_ops::to_usize( &prime_field::fpow(&uint256_ops::get_uint256("2"), &fri_steps[fri_step]) ); // 2^friSteps[friStep]
		 n_live_queries = compute_next_layer(
                channel_idx, fri_queue, merkle_queue_idx, n_live_queries,
                ctx[map::MM_FRI_EVAL_POINTS + fri_step].clone(), fri_coset_size, fri_ctx, ctx
		);

		// Layer is done, verify the current layer and move to next layer.
		// ctx[mmMerkleQueue: merkleQueueIdx) holds the indices
		// and values of the merkle leaves that need verification.
		verify_merkle(
                channel_idx, ctx, merkle_queue_idx, ctx[map::MM_FRI_COMMITMENTS + fri_step - 1].clone(), n_live_queries
		);
				
		fri_step += 1;
	}

	verify_last_layer(ctx, n_live_queries);

}


/*
	Initializes the FRI group and half inv group in the FRI context.
*/
fn init_fri_groups(fri_ctx: usize, ctx: & mut Vec<Uint256>) {
	let fri_group_idx = fri_ctx + FRI_CTX_TO_FRI_GROUP_OFFSET;
	let fri_half_inv_group_idx = fri_ctx + FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET;

	// FRI_GROUP_GEN is the coset generator.
	// Raising it to the (MAX_COSET_SIZE - 1) power gives us the inverse
	let gen_fri_group = uint256_ops::get_uint256(FRI_GROUP_GEN);

	let gen_fri_group_inv = prime_field::fpow( &gen_fri_group, &uint256_ops::from_usize(MAX_COSET_SIZE - 1) );

	ctx[fri_half_inv_group_idx] = uint256_ops::get_uint256("1");
	ctx[fri_group_idx] = uint256_ops::get_uint256("1");
	ctx[fri_group_idx + 1] = prime_field::get_k_modulus() - uint256_ops::get_uint256("1"); //PRIME - 1s

	let mut last_val = uint256_ops::get_uint256("1");
	let mut last_val_inv = uint256_ops::get_uint256("1"); 

	// To compute [1, -1 (== g^n/2), g^n/4, -g^n/4, ...]
	// we compute half the elements and derive the rest using negation.
	for i in 1..MAX_COSET_SIZE/2 {
		last_val = prime_field::fmul(last_val.clone(), gen_fri_group.clone()); //Should be for first iteration: 2679026602897868112349604024891625875968950767352485125058791696935099163961, 
		last_val_inv = prime_field::fmul(last_val_inv.clone(), gen_fri_group_inv.clone()); //Should be for first iteration: 2607735469685256064975697808597423000021425046638838630471627721324227832437, 
		
		let idx = bit_reverse( uint256_ops::from_usize(i), FRI_MAX_FRI_STEP-1);

		ctx[fri_half_inv_group_idx + idx] = last_val_inv.clone();
		ctx[fri_group_idx + 2*idx] = last_val.clone();
		ctx[fri_group_idx + 2*idx + 1] = prime_field::get_k_modulus() - last_val.clone();

	}
}

/*
	Returns the bit reversal of num assuming it has the given number of bits.
	For example, if we have numberOfBits = 6 and num = (0b)1101 == (0b)001101,
	the function will return (0b)101100.
*/
fn bit_reverse(num: Uint256, num_of_bits: usize) -> usize {
	assert!( num_of_bits == 256 || num < prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::from_usize(num_of_bits) ) ); // Make sure number size is correctly specified

	let mut r = 0;
	let mut n = uint256_ops::to_usize(&num);
	for _ in 0..num_of_bits {
		r = (r * 2) | (n % 2);
		n = n / 2;
	}

	return r;
}

pub fn get_fri_steps(ctx: &mut Vec<Uint256>) -> Vec<Uint256> {
	let mut fri_steps: Vec<Uint256> =  vec![];
	fri_steps.push(ctx[map::MM_FRI_STEPS_PTR].clone());
	return fri_steps; //TODO: This doesn't seem right ...
}


/*
	Computes the FRI step with eta = log2(friCosetSize) for all the live queries.
	The input and output data is given in array of triplets:
		(query index, FRI value, FRI inversed point)
	in the address friQueuePtr (which is &ctx[mmFriQueue:]).
	The function returns the number of live queries remaining after computing the FRI step.
	The number of live queries decreases whenever multiple query points in the same
	coset are reduced to a single query in the next FRI layer.
	As the function computes the next layer it also collects that data from
	the previous layer for Merkle verification.
*/
pub fn compute_next_layer(
	channel_idx: usize, fri_queue_idx: usize, merkle_queue_idx: usize, n_queries: usize,
	fri_eval_point: Uint256, fri_coset_size: usize, fri_ctx: usize, ctx: &mut Vec<Uint256>
) -> usize {
	
	let fri_queue_end = fri_queue_idx + 3 * n_queries;
	let mut fri_queue_tail = fri_queue_idx;
	let mut fri_queue_head = fri_queue_idx;
	let mut merkle_queue_tail = merkle_queue_idx;

	//Do: get coset inputs and do fri steps while still in fri queue

	let (new_queue_head0, index0, coset_offset0) = gather_coset_inputs(
		channel_idx, fri_ctx, fri_queue_head, fri_coset_size, ctx
	);
	do_fri_steps(
        fri_ctx, fri_queue_tail, coset_offset0, fri_eval_point.clone(), fri_coset_size, index0, merkle_queue_tail, ctx
	);

	merkle_queue_tail += 2;
	fri_queue_tail += 3;
	fri_queue_head = new_queue_head0;


	while fri_queue_head < fri_queue_end {
		let (new_queue_head, index, coset_offset) = gather_coset_inputs(
			channel_idx, fri_ctx, fri_queue_head, fri_coset_size, ctx
		);

		do_fri_steps(
			fri_ctx, fri_queue_tail, coset_offset, fri_eval_point.clone(), fri_coset_size, index, merkle_queue_tail, ctx
		);

		merkle_queue_tail += 2;
		fri_queue_tail += 3;
		fri_queue_head = new_queue_head;
	}

	return (fri_queue_tail - fri_queue_idx) / 3;

}

 /*
	Gathers the "cosetSize" elements that belong to the same coset
	as the item at the top of the FRI queue and stores them in ctx[MM_FRI_STEP_VALUES:].
	Returns
		friQueueHead - friQueueHead + 3 * (# elements that were taken from the queue).
		cosetIdx - the start index of the coset that was gathered.
		cosetOffset - the xInv field element that corresponds to cosetIdx.
*/
fn gather_coset_inputs(
	channel_idx: usize, fri_ctx: usize, fri_queue_head_input: usize, coset_size: usize, ctx: & mut Vec<Uint256>
) -> (usize, Uint256, Uint256) {

	let mut evals_on_coset_idx = fri_ctx + FRI_CTX_TO_COSET_EVALUATIONS_OFFSET;
	let fri_group_idx = fri_ctx + FRI_CTX_TO_FRI_GROUP_OFFSET;
	let mut fri_queue_head = fri_queue_head_input; //mutable copy of input

	let mut queue_item_idx = ctx[fri_queue_head].clone();

	// The coset index is represented by the most significant bits of the queue item index.
	let negated: Uint256 = uint256_ops::bitwise_not( uint256_ops::from_usize(coset_size-1)  );
	let coset_idx = uint256_ops::bitwise_and( &queue_item_idx.clone(), &negated );
	let next_coset_idx = coset_idx.clone() + uint256_ops::from_usize(coset_size);
	

	// Get the algebraic coset offset:
	// I.e. given c*g^(-k) compute c, where
	//      g is the generator of the coset group.
	//      k is bitReverse(offsetWithinCoset, log2(cosetSize)).
	//
	// To do this we multiply the algebraic coset offset at the top of the queue (c*g^(-k))
	// by the group element that corresponds to the index inside the coset (g^k)
	let coset_offset = prime_field::fmul(
		ctx[fri_queue_head + 2].clone(),
		ctx[fri_group_idx + uint256_ops::to_usize( &(queue_item_idx.clone() - coset_idx.clone()) ) ].clone()
	);

	let mut proof_idx = uint256_ops::to_usize(&ctx[channel_idx]);

	let mut index = coset_idx.clone();
	while index < next_coset_idx {
		// Inline channel operation:
		// Assume we are going to read the next element from the proof.
		// If this is not the case proof_idx += 1 will be reverted
		let mut field_elem_idx = proof_idx;
		proof_idx += 1;

		// Load the next index from the queue and check if it is our sibling.
		if index == queue_item_idx {
			// Take element from the queue rather than from the proof
            // and convert it back to Montgomery form for Merkle verification.
			field_elem_idx = fri_queue_head + 1;

			// Revert the read from proof.
			proof_idx -= 1;

			// Reading the next index here is safe due to the
			// delimiter after the queries
			fri_queue_head = fri_queue_head + 3;
			queue_item_idx = ctx[fri_queue_head].clone();
		}

		// Note that we apply the modulo operation to convert the field elements we read
		// from the proof to canonical representation (in the range [0, PRIME - 1])
		ctx[evals_on_coset_idx] = prime_field::mod_prime( ctx[field_elem_idx].clone() ); //mod (val, PRIME)
		evals_on_coset_idx += 1;

		index += uint256_ops::get_uint256("1");
	} 

	//Update proof pointer to reflect proof's data read
	ctx[channel_idx] = uint256_ops::from_usize(proof_idx);

	return (fri_queue_head, coset_idx, coset_offset);

}







 /*
	Operates on the coset of size friFoldedCosetSize that start at index.
	It produces 3 outputs:
	  1. The field elements that result from doing FRI reductions on the coset.
	  2. The pointInv elements for the location that corresponds to the first output.
	  3. The root of a Merkle tree for the input layer.
	The input is read either from the queue or from the proof depending on data availability.
	Since the function reads from the queue it returns an updated head pointer.
*/
fn do_fri_steps(
	fri_ctx: usize, fri_queue_tail: usize,  coset_offset_input: Uint256, fri_eval_point: Uint256,
	fri_coset_size: usize, index: Uint256, merkle_queue_idx: usize, ctx: & mut Vec<Uint256>
) {

	let evals_on_coset_idx = fri_ctx + FRI_CTX_TO_COSET_EVALUATIONS_OFFSET;
	let fri_half_inv_group_idx = fri_ctx + FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET;

	let mut fri_val = uint256_ops::get_uint256("0");
	let mut coset_offset = coset_offset_input;


	// Compare to expected FRI step sizes in order of likelihood, step size 3 being most common.
	if fri_coset_size == 8 {
		let (fri_val_tmp, coset_offset_tmp) = do_3_fri_steps( fri_half_inv_group_idx, evals_on_coset_idx, coset_offset, fri_eval_point, ctx );
		fri_val = fri_val_tmp;
		coset_offset = coset_offset_tmp;
	}else if fri_coset_size == 4 {
		let (fri_val_tmp, coset_offset_tmp) = do_2_fri_steps( fri_half_inv_group_idx, evals_on_coset_idx, coset_offset, fri_eval_point, ctx );
		fri_val = fri_val_tmp;
		coset_offset = coset_offset_tmp;
	}else if fri_coset_size == 16  {
		let (fri_val_tmp, coset_offset_tmp) = do_4_fri_steps( fri_half_inv_group_idx, evals_on_coset_idx, coset_offset, fri_eval_point, ctx );
		fri_val = fri_val_tmp;
		coset_offset = coset_offset_tmp;
	}else {
		assert!(false); // Only step sizes of 2, 3 or 4 are supported
	} 

	let idx_in_nxt_step = index / uint256_ops::from_usize(fri_coset_size);
	ctx[merkle_queue_idx] = idx_in_nxt_step.clone();

	let mut hash_data: Vec<u8> = vec![];
	for i in 0..fri_coset_size {
		let data_bytes = uint256_ops::to_fixed_bytes( &ctx[evals_on_coset_idx + i] );
		for j in 0..data_bytes.len() {
			hash_data.push( data_bytes[j] );
		}
	}

	ctx[merkle_queue_idx + 1] = uint256_ops::bitwise_and( 
		&get_hash_mask(), &uint256_ops::keccak_256(&hash_data)
	);
	ctx[fri_queue_tail] = idx_in_nxt_step.clone();
	ctx[fri_queue_tail + 1] = fri_val;
	ctx[fri_queue_tail + 2] = coset_offset;
}


 /*
	Reads 4 elements, and applies 2 + 1 FRI transformations to obtain a single element.
	FRI layer n:                              f0 f1  f2 f3
	-----------------------------------------  \ / -- \ / -----------
	FRI layer n+1:                              f0    f2
	-------------------------------------------- \ ---/ -------------
	FRI layer n+2:                                 f0
	The basic FRI transformation is described in nextLayerElementFromTwoPreviousLayerElements().
*/
fn do_2_fri_steps(
	fri_half_inv_group_idx: usize, evals_on_coset_idx: usize, coset_offset_input: Uint256, fri_eval_point: Uint256, ctx: & mut Vec<Uint256>
) -> (Uint256, Uint256) {

	let fri_eval_point_divbyx = prime_field::fmul( fri_eval_point.clone(), coset_offset_input.clone() );

	let mut f0 = ctx[evals_on_coset_idx].clone();
	let f1 = ctx[evals_on_coset_idx + 1].clone();
	// f0 < 3P ( = 1 + 1 + 1).
	f0 = (f0.clone() + f1.clone()) + prime_field::fmul( fri_eval_point_divbyx.clone(), f0.clone() + (prime_field::get_k_modulus() - f1.clone()) );

	let mut f2 = ctx[evals_on_coset_idx + 2].clone();
	let f3 = ctx[evals_on_coset_idx + 3].clone();
	f2 = prime_field::fadd( 
		f2.clone() + f3.clone(),  
		prime_field::fmul(
			f2.clone() + (prime_field::get_k_modulus() - f3.clone()), 
			prime_field::fmul(
				ctx[fri_half_inv_group_idx + 1].clone(), fri_eval_point_divbyx.clone()
			)
		)
	);

	let mut new_x_inv = prime_field::fmul( coset_offset_input.clone(), coset_offset_input.clone() );
	new_x_inv = prime_field::fmul( new_x_inv.clone(), new_x_inv );

	// f0 + f2 < 4P ( = 3 + 1)
	let next_layer_value = prime_field::fadd(
		f0.clone() + f2.clone(), 
		prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), fri_eval_point_divbyx
			),
			f0.clone() + (prime_field::get_k_modulus() - f2.clone())
		)
	);

	return (next_layer_value, new_x_inv);

}




/*
	Reads 8 elements, and applies 4 + 2 + 1 FRI transformation to obtain a single element.
	See do_2_fri_steps for more detailed explanation.
*/
fn do_3_fri_steps(
	fri_half_inv_group_idx: usize, evals_on_coset_idx: usize, coset_offset_input: Uint256, fri_eval_point: Uint256, ctx: & mut Vec<Uint256>
) -> (Uint256, Uint256) {
	let prime = prime_field::get_k_modulus();
	let m_prime = uint256_ops::get_uint256("8000000000000110000000000000000000000000000000000000000000000010");
	let mut f0 = ctx[evals_on_coset_idx].clone();
	
	let fri_eval_point_divbyx = prime_field::fmul( fri_eval_point.clone(), coset_offset_input.clone() );
	let fri_eval_point_divbyx_squared = prime_field::fmul( fri_eval_point_divbyx.clone(), fri_eval_point_divbyx.clone() );
	let imaginary_unit = ctx[fri_half_inv_group_idx + 1].clone();

	let f1 = ctx[evals_on_coset_idx + 1].clone();
	// f0 < 3P ( = 1 + 1 + 1).
	f0 = (f0.clone() + f1.clone()) + prime_field::fmul( fri_eval_point_divbyx.clone(), f0.clone() + (prime.clone() - f1.clone()) );

	let mut f2 = ctx[evals_on_coset_idx + 2].clone();
	let f3 = ctx[evals_on_coset_idx + 3].clone();
	f2 = (f2.clone() + f3.clone()) + 
		prime_field::fmul(
			f2.clone() + (prime.clone() - f3.clone()), 
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), imaginary_unit.clone()
			)
		);
	
	// f0 < 7P ( = 3 + 3 + 1).
	f0 = (f0.clone() + f2.clone()) + 
		prime_field::fmul(
			fri_eval_point_divbyx_squared.clone(), f0.clone() + (m_prime.clone() - f2.clone())
		);


	let mut f4 = ctx[evals_on_coset_idx + 4].clone();
	let fri_eval_point_div_by_x2 = prime_field::fmul( fri_eval_point_divbyx.clone(), ctx[fri_half_inv_group_idx + 4].clone() );

	let f5 = ctx[evals_on_coset_idx + 5].clone();
	f4 = (f4.clone() + f5.clone()) + prime_field::fmul(
			fri_eval_point_div_by_x2.clone(), f4.clone() + (prime.clone() - f5.clone())
		);

	let mut f6 = ctx[evals_on_coset_idx + 6].clone();
	let f7 = ctx[evals_on_coset_idx + 7].clone();

	 // f6 < 3P ( = 1 + 1 + 1).
	 f6 = (f6.clone() + f7.clone()) + prime_field::fmul(
			f6.clone() + (prime.clone() - f7.clone()), 
			// friEvalPointDivByX2 * imaginaryUnit ==
			// friEvalPointDivByX * ctx[friHalfInvGroupPtr + 3]
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), imaginary_unit.clone()
			)

	 	);

	// f4 < 7P ( = 3 + 3 + 1)
	f4 = (f4.clone() + f6.clone()) + prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), fri_eval_point_divbyx.clone()
			), 
			f4.clone() + (m_prime.clone() - f6.clone())
		);



	
	// f0, f4 < 7P -> f0 + f4 < 14P && 9P < f0 + (MPRIME - f4) < 23P.
	let next_layer_value = prime_field::fadd(
		f0.clone() + f4.clone(), 
		prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx_squared.clone(), fri_eval_point_divbyx_squared.clone()
			),
			f0.clone() + (m_prime.clone() - f4.clone())
		)
	);
	

	let x_inv_2 = prime_field::fmul( coset_offset_input.clone(), coset_offset_input.clone() );
	let x_inv_4 = prime_field::fmul( x_inv_2.clone(), x_inv_2 );
	let new_x_inv = prime_field::fmul( x_inv_4.clone(), x_inv_4 );

	return (next_layer_value, new_x_inv);

}

/*
	This function reads 16 elements, and applies 8 + 4 + 2 + 1 fri transformation
	to obtain a single element.
	See do2FriSteps for more detailed explanation.
*/
fn do_4_fri_steps(
	fri_half_inv_group_idx: usize, evals_on_coset_idx: usize, coset_offset_input: Uint256, fri_eval_point: Uint256, ctx: & mut Vec<Uint256>
) -> (Uint256, Uint256) {

	let prime = prime_field::get_k_modulus();
	let m_prime = uint256_ops::get_uint256("8000000000000110000000000000000000000000000000000000000000000010");
	let mut f0 = ctx[evals_on_coset_idx].clone();
	
	let fri_eval_point_divbyx = prime_field::fmul( fri_eval_point.clone(), coset_offset_input.clone() );
	let imaginary_unit = ctx[fri_half_inv_group_idx + 1].clone();

	let f1 = ctx[evals_on_coset_idx + 1].clone();
	// f0 < 3P ( = 1 + 1 + 1).
	f0 = (f0.clone() + f1.clone()) + prime_field::fmul( fri_eval_point_divbyx.clone(), f0.clone() + (prime.clone() - f1.clone()) );

	let mut f2 = ctx[evals_on_coset_idx + 2].clone();
	let f3 = ctx[evals_on_coset_idx + 3].clone();
	f2 = (f2.clone() + f3.clone()) + 
		prime_field::fmul(
			f2.clone() + (prime.clone() - f3.clone()), 
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), imaginary_unit.clone()
			)
		);
	
	let fri_eval_point_divbyx_squared = prime_field::fmul( fri_eval_point_divbyx.clone(), fri_eval_point_divbyx.clone() );
	let fri_eval_point_divbyx_tessed = prime_field::fmul( fri_eval_point_divbyx_squared.clone(), fri_eval_point_divbyx_squared.clone() );
	// f0 < 7P ( = 3 + 3 + 1).
	f0 = (f0.clone() + f2.clone()) + 
		prime_field::fmul(
			fri_eval_point_divbyx_squared, f0.clone() + (m_prime.clone() - f2.clone())
		);


	let mut f4 = ctx[evals_on_coset_idx + 4].clone();
	let fri_eval_point_div_by_x2 = prime_field::fmul( fri_eval_point_divbyx.clone(), ctx[fri_half_inv_group_idx + 4].clone() );

	let f5 = ctx[evals_on_coset_idx + 5].clone();

	// f4 < 3P ( = 1 + 1 + 1)
	f4 = (f4.clone() + f5.clone()) + prime_field::fmul(
			fri_eval_point_div_by_x2, f4.clone() + (prime.clone() - f5.clone())
		);

	let mut f6 = ctx[evals_on_coset_idx + 6].clone();
	let f7 = ctx[evals_on_coset_idx + 7].clone();

	 // f6 < 3P ( = 1 + 1 + 1).
	 f6 = (f6.clone() + f7.clone()) + prime_field::fmul(
			f6.clone() + (prime.clone() - f7.clone()), 
			// friEvalPointDivByX2 * imaginaryUnit ==
			// friEvalPointDivByX * ctx[friHalfInvGroupPtr + 3]
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), imaginary_unit.clone()
			)

	 	);

	// f4 < 7P ( = 3 + 3 + 1)
	f4 = (f4.clone() + f6.clone()) + prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx.clone(), fri_eval_point_divbyx.clone()
			), 
			f4.clone() + (m_prime.clone() - f6.clone())
		);

	// f0 < 15P ( = 7 + 7 + 1)
	f0 = (f0.clone() + f4.clone()) + prime_field::fmul(
			fri_eval_point_divbyx_tessed.clone(), 
			f0.clone() + (m_prime.clone() - f4.clone())
		);

	let mut f8 = ctx[evals_on_coset_idx + 8].clone();
	let fri_eval_point_div_by_x4 = prime_field::fmul( fri_eval_point_divbyx.clone(), ctx[fri_half_inv_group_idx + 4].clone() );
	let f9 = ctx[evals_on_coset_idx + 9].clone();

	// f8 < 3P ( = 1 + 1 + 1)
	f8 = (f8.clone() + f9.clone()) + prime_field::fmul(
			fri_eval_point_div_by_x4.clone(), 
			f8.clone() + (prime.clone() - f9.clone())
		);

	let mut f10 = ctx[evals_on_coset_idx + 10].clone();

	let f11 = ctx[evals_on_coset_idx + 11].clone();

	// f10 < 3P ( = 1 + 1 + 1)
	f10 = (f10.clone() + f11.clone()) + prime_field::fmul(
			f10.clone() + (prime.clone() - f11.clone()), 
			// friEvalPointDivByX4 * imaginaryUnit ==
			// friEvalPointDivByX * mload(add(friHalfInvGroupPtr, 0xa0))
			prime_field::fmul(
				fri_eval_point_div_by_x4.clone(), imaginary_unit.clone()
			)
		);

	// f8 < 7P ( = 3 + 3 + 1)
	f8 = (f8.clone() + f10.clone()) + prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_div_by_x4.clone(), fri_eval_point_div_by_x4.clone()
			),
			f8.clone() + (m_prime.clone() - f10.clone())
		);

	let mut f12 = ctx[evals_on_coset_idx + 12].clone();
	let fri_eval_point_div_by_x6 = prime_field::fmul( fri_eval_point_divbyx, ctx[fri_half_inv_group_idx + 6].clone() );

	let f13 = ctx[evals_on_coset_idx + 13].clone();
	
	// f12 < 3P ( = 1 + 1 + 1)
	f12 = (f12.clone() + f13.clone()) + prime_field::fmul(
			fri_eval_point_div_by_x6.clone(),
			f12.clone() + (prime.clone() - f13.clone())
		);
	
	let mut f14 = ctx[evals_on_coset_idx + 14].clone();
	let f15 = ctx[evals_on_coset_idx + 15].clone();

	// f14 < 3P ( = 1 + 1 + 1)
	f14 = (f14.clone() + f15.clone()) + prime_field::fmul(
			f14.clone() + (prime.clone() - f15.clone()), 
			// friEvalPointDivByX6 * imaginaryUnit ==
			// friEvalPointDivByX * mload(add(friHalfInvGroupPtr, 0xe0))
			prime_field::fmul(
				fri_eval_point_div_by_x6.clone(), imaginary_unit.clone()
			)
		);

	// f12 < 7P ( = 3 + 3 + 1)
	f12 = (f12 .clone()+ f14.clone()) + prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_div_by_x6.clone(), fri_eval_point_div_by_x6.clone()
			),
			f12.clone() + (m_prime.clone() - f14.clone())
		);

	// f8 < 15P ( = 7 + 7 + 1)
	f8 = (f8.clone() + f12.clone()) + prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx_tessed.clone(), imaginary_unit.clone()
			),
			f8.clone() + (m_prime.clone() - f12.clone())
		);

	
	// f0, f8 < 15P -> f0 + f8 < 30P && 16P < f0 + (MPRIME - f8) < 31P
	let next_layer_value = prime_field::fadd(
		f0.clone() + f8.clone(), 
		prime_field::fmul(
			prime_field::fmul(
				fri_eval_point_divbyx_tessed.clone(), fri_eval_point_divbyx_tessed
			),
			f0 + (m_prime - f8)
		)
	);
	

	let x_inv_2 = prime_field::fmul( coset_offset_input.clone(), coset_offset_input.clone() );
	let x_inv_4 = prime_field::fmul( x_inv_2.clone(), x_inv_2.clone() );
	let x_inv_8 = prime_field::fmul( x_inv_4.clone(), x_inv_4.clone() );
	let new_x_inv = prime_field::fmul( x_inv_8.clone(), x_inv_8.clone() );

	return (next_layer_value, new_x_inv);

}










/* -------------------------
	***** FRI TESTING *******
 --------------------------- */


/* ------------- Testing Harness ------------- */
fn verify_fri(
	proof: Vec<Uint256>, fri_queue: &mut Vec<Uint256>, evaluation_point: Uint256, fri_step_size: usize, expected_root: Uint256
) {
	assert!(fri_step_size <= FRI_MAX_FRI_STEP); //FRI step size too large
	/*
		The fri_queue should have of 3*n_queries + 1 elements, beginning with n_queries triplets
		of the form (query_index, FRI_value, FRI_inverse_point), and ending with a single buffer
		cell set to 0, which is accessed and read during the computation of the FRI layer.
	*/
	assert!( fri_queue.len() % 3 == 1); //FRI Queue must be composed of triplets plus one delimiter cell
	assert!( fri_queue.len() >= 4 ); //No query to process
	let mut n_queries = fri_queue.len() / 3;
	fri_queue[3*n_queries] = uint256_ops::get_uint256("0"); 

	// Verify evaluation point within valid range.
	assert!(evaluation_point < prime_field::get_k_modulus()); //INVALID_EVAL_POINT

	// Queries need to be in the range [2**height .. 2**(height+1)-1] strictly incrementing.
	// i.e. we need to check that Qi+1 > Qi for each i,
	// but regarding the height range - it's sufficient to check that
	// (Q1 ^ Qn) < Q1 Which affirms that all queries are within the same logarithmic step.

	// Verify FRI values and inverses are within valid range.
	// and verify that queries are strictly incrementing.
	let mut prev_query = uint256_ops::get_uint256("0"); //If we pass height, change to: prev_query = 1 << height - 1;
	for i in 0..n_queries {
		assert!( fri_queue[3*i] > prev_query ); //INVALID_QUERY_VALUE
		assert!( fri_queue[3*i + 1] < prime_field::get_k_modulus() ); //INVALID_FRI_VALUE
		assert!( fri_queue[3*i + 2] < prime_field::get_k_modulus() ); //INVALID_FRI_INVERSE_POINT
		prev_query = fri_queue[3*i].clone();
	}

	// Verify all queries are on the same logarithmic step.
	assert!( uint256_ops::bitwise_xor(&fri_queue[0], &fri_queue[3*n_queries-3])  < fri_queue[0] ); //INVALID_QUERIES_RANGE

	//Setup indicies and copy input data to ctx, verifier state
	let mut ctx: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 2315];
	let fri_ctx = map::MM_FRI_CTX;
	let channel_idx = map::MM_CHANNEL;
	let merkle_queue_idx = map::MM_MERKLE_QUEUE;
	let fri_queue_idx = map::MM_FRI_QUEUE;

	//Copy fri queue
	for i in 0..fri_queue.len() {
		ctx[fri_queue_idx + i] = fri_queue[i].clone();
	}

	//Channel points to proof (end of ctx)  = 2315 decimal
	ctx[channel_idx] = uint256_ops::get_uint256("90B"); 

	//Add a copy of proof input to the end of ctx, verifier state
	for i in 0..proof.len() {
		ctx.push(proof[i].clone());
	}
 

	init_fri_groups(fri_ctx, &mut ctx);


	n_queries = compute_next_layer(
		channel_idx, fri_queue_idx, merkle_queue_idx, n_queries, evaluation_point, usize::pow(2, fri_step_size as u32), fri_ctx, &mut ctx
	);

	verify_merkle( channel_idx,  &mut ctx, merkle_queue_idx, expected_root, n_queries );
}



/// ----------
/// UNIT TESTS
/// ----------

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::uint256_ops;

    #[test]
    fn test_honest_fri_data() {
		//Data is taken from: https://etherscan.io/tx/0xe176d1b273b8cbbcea12f2ddf02a87e764b03407747d365eb0ece0ccb49cff65
		let proof = vec![
			uint256_ops::get_uint256("BEB1B8B9A8477DF9D44CC95A9CD5A9194C16D0238E7EC364F72487DF5C4A06"),
			uint256_ops::get_uint256("6D39056A6D89CCC48465706A45EE5937A2750962FBE839942F110B31C84D2E8"),
			uint256_ops::get_uint256("64F8F4A784F6EFCBD02E7AC435081222486EBE2D1B595684400E38B9DA76EBE"),
			uint256_ops::get_uint256("1BBFE3131F065DB9BDC25EF748602B21433EBDCA39E1A43A5C1634167B78B1B"),
			uint256_ops::get_uint256("02c700808b8247a139fc7a01493cd7517e315e48f84d0ee998870e70e554a821"),
			uint256_ops::get_uint256("700E2121B2FEB79631B2049F22B2A089D9940EDD91774F9E794431417B1B826"),
			uint256_ops::get_uint256("2717DA2FC3ECF7B7487CC13B8A26F855C15B9E21306E34016A6F77219A6B713"),
			uint256_ops::get_uint256("250072BD637E851834347BF898498B7E3C13C12176D0E64D7D5279FD19AA0BC"),
			uint256_ops::get_uint256("F98C6F3C8AB41ECC08DCDA66EE52F6748C3ED485F75292C42C9EF3487BE2E9"),
			uint256_ops::get_uint256("21CC220F4D78E164EF0C5B0C4B386E2D15D1C98FC91AF76186007DE5778B69"),
			uint256_ops::get_uint256("64C3C89230E7A8FE737C8EFDC62431195D8D1714AF1CD4392594B57DD480023"),
			uint256_ops::get_uint256("727089973A291CFEF6690C68F69B04055158914D185C4D845396C7C02F9CBB"),
			uint256_ops::get_uint256("3BACBBE67CDD11DD8A6D68842C2A1357C870B3677248C7E0E7B6C7C804542AD"),
			uint256_ops::get_uint256("182B875BC33B2BEBA47D91318DA741D4BD4EBAE30BE00992FD0816A561C0E59"),
			uint256_ops::get_uint256("228E0CE9288C457601491E4C0F6A0F4A7949EF256ED63874265943C0D321F1F"),
			uint256_ops::get_uint256("42009DF4A10A16F4F6A99E161FC3DF4ABA130616EC02FC211E8EAACEC6BF4BA"),
			uint256_ops::get_uint256("2B898C049200BC0DE48705D723F226E22272B8ADC14C0AAA19D9A0745BE717A"),
			uint256_ops::get_uint256("5dddd5106e3159be56cce000ede416abac6a580721ca6f531fc816f6c8b43a5"),
			uint256_ops::get_uint256("1EF1CAD653F4565AA78FEF3151746264BAB97657F0EF5D8683C1BFDAD6738F"),
			uint256_ops::get_uint256("34D0800CC4CA30392991D9E7B511A8619ECC35DAB68262A60A6A751F4890DDF"),
			uint256_ops::get_uint256("41F4F2D2B2159ABA46B26017D6AFC08C2F25ADEF8314C52B745D65593D15E49"),
			uint256_ops::get_uint256("6794EF0EF42141CF1C3443C06C8F90AF84E3A348675F6260EBC448DE8A15F1C"),
			uint256_ops::get_uint256("339919B13A051B00EB70EA95BF16039FC1CEBCF957EAF631AE090BCC977A74D"),
			uint256_ops::get_uint256("11677B4C60A8568BF408614585967BB6A10B17A81E0B65CDF6497A634CA68C8"),
			uint256_ops::get_uint256("6AC8F371FA684892E3FB3BC8DDC953A0C32E890BB8A7999ADCE598E695CECF7"),
			uint256_ops::get_uint256("5B8C436CC72C6D3FCF29DF54C535A33DC9E025D2C65D091666A2238742BBFF"),
			uint256_ops::get_uint256("929851B18224352038A5456DA9E74C2FB4DA54E3611C79286F8686149A9195"),
			uint256_ops::get_uint256("202B5FB99AAFCC8A636B7080421EABE45FAC5EE8D70052166870E0CB838BF4D"),
			uint256_ops::get_uint256("4A77F3E3CBA858F4E756A14A8B32DF599C9F3D79264E83BA94D9089CF2F609"),
			uint256_ops::get_uint256("51FA5AE4C1B87374819AB70F036C5D20EEED6E98E79E54D7C4DEC30D55FD047"),
			uint256_ops::get_uint256("51E4F114769100EFFDF74A42B94BDB0D3AF059A6EF29F3E1FB57AD5093B0172"),
			uint256_ops::get_uint256("2D2F33993E98424EF719A4DFCE79EFA542E7AA437EAFDB147B79A96C2723146"),
			uint256_ops::get_uint256("178E877AF3C131CFAED78F4772AEF4F4D89BCCC18C2C6079B2CDD2F17DBA4F1"),
			uint256_ops::get_uint256("626A818C749C63D9EC70E3BBC0499BA6572E4FB4151EF2BEE44E7A76735C150"),
			uint256_ops::get_uint256("4B19F733A98A7D69B493D323B85BA33CFB23CCAD52144B2C6BCECFC8C3A72D6"),
			uint256_ops::get_uint256("494D4527405C142914F6E87A9443209B8937B0BD4E55D595903403A76678FC2"),
			uint256_ops::get_uint256("106D18C4002F3483D9BAA5D1A81E246AF100460C000000000000000000000000"),
			uint256_ops::get_uint256("ADB892B5E8AEB7EE290B861B2F6F0CEE8CCB1848000000000000000000000000"),
			uint256_ops::get_uint256("F1CC5D70B93B28D284A175B0BD5D897776E7F353000000000000000000000000"),
			uint256_ops::get_uint256("F8FDBE5952C45BE554A91B21D95233AA05915911000000000000000000000000"),
			uint256_ops::get_uint256("86A3E788D52E4AB0DE28500CB1E146EC79CA9E8F000000000000000000000000"),
			uint256_ops::get_uint256("BB21F1F719603928B7E52B8D85CED5864A47B6DC000000000000000000000000"),
			uint256_ops::get_uint256("1172AE9710C1422BFAF17C0059ADE6BF8692B6F0000000000000000000000000"),
			uint256_ops::get_uint256("FF5F7DF9FEF047EB26C4D8A0378F240771A86BD1000000000000000000000000"),
			uint256_ops::get_uint256("D686F17593D61F6616CF6AE5FF05EF739724EBB4000000000000000000000000"),
			uint256_ops::get_uint256("C20BF718DBEA35A8DC0F110EBACF6A2A4E4D7724000000000000000000000000"),
			uint256_ops::get_uint256("C139C853F0C9FC6DD1005D59F5B27F1D6C2E39C8000000000000000000000000"),
			uint256_ops::get_uint256("E097935D108368F333A681E44449C4F77E039D2C000000000000000000000000"),
			uint256_ops::get_uint256("76F7387E37116F084F45045725C48FC43772DAC7000000000000000000000000"),
			uint256_ops::get_uint256("50E9506407B4FBA39815A053EEDAB4CE9521DE9F000000000000000000000000"),
			uint256_ops::get_uint256("F487C52E5D60BAAADAB89670967F960D0B0D9C96000000000000000000000000"),
			uint256_ops::get_uint256("B5348E1191C96E11EA81453FAB4D19DA994C76B4000000000000000000000000"),
			uint256_ops::get_uint256("E567E13F52B09FCE2F8F89FAB28DEF58905C240E000000000000000000000000"),
			uint256_ops::get_uint256("8CEDE3497DF5130EB975FF5D69113AD85A5D7AF4000000000000000000000000"),
			uint256_ops::get_uint256("CB3ACD79075359115A271247A405CE7698C761EC000000000000000000000000"),
			uint256_ops::get_uint256("26CC8E2E340F5C41AD1F316575708F309EF0EBB8000000000000000000000000"),
			uint256_ops::get_uint256("517DDB9096B892E616ADE1B6EA706590420C0405000000000000000000000000"),
			uint256_ops::get_uint256("440C117F0913F4C2722C533F3430A535BBCD5A49000000000000000000000000"),
			uint256_ops::get_uint256("215FF6449858043C9BAC1E83F03BE57E50B991DD000000000000000000000000"),
			uint256_ops::get_uint256("9772FBFF66A1B01BCDD9E424AE4344E6C925F098000000000000000000000000"),
			uint256_ops::get_uint256("B31E8ED91F27E66EC175C116A3E9093CF7F64D7A000000000000000000000000"),
			uint256_ops::get_uint256("F604CE59F41180E4ED57E43E2ACAD3965819E4A1000000000000000000000000"),
			uint256_ops::get_uint256("AA102CA2FB6E727A3D513D705E565B20F7DA44B0000000000000000000000000"),
			uint256_ops::get_uint256("52321A910C3BA8A9C66EC26C38884A313D0A5BD2000000000000000000000000"),
			uint256_ops::get_uint256("39C323743F03325B56A8D67D5AA49D657C2B0968000000000000000000000000"),
			uint256_ops::get_uint256("AF10496AE254309884DA6E4698539D24A29BA6FF000000000000000000000000"),
			uint256_ops::get_uint256("d895fbeb266d323cff4d94e1260045bdc4750fd000000000000000000000000"),
			uint256_ops::get_uint256("B0FAE3BC4EEEB03848059607CB7E75D92D4209BB000000000000000000000000"),
			uint256_ops::get_uint256("A17832669BF07D342DFC5F2BFEF61F6DF339F1ED000000000000000000000000"),
			uint256_ops::get_uint256("DEA0D82B8AA3CF1446CD95237062FBBE0E4A93A9000000000000000000000000"),
			uint256_ops::get_uint256("CC322D149F0EA1264D0954FDE18138C326E2782A000000000000000000000000"),
			uint256_ops::get_uint256("65D027918110B69EA02527CDC5A3D88A26DEBE89000000000000000000000000"),
			uint256_ops::get_uint256("2F62719B66F87F57E7718D6482FEBE2AAA6BA043000000000000000000000000"),
			uint256_ops::get_uint256("6A2A91EBC751F75B655A7988DAA471393D976BB6000000000000000000000000"),
			uint256_ops::get_uint256("D3E82DB9B4B932EF860196F02AA21594B22065E0000000000000000000000000"),
			uint256_ops::get_uint256("CD95B4629D9E44B926D600A02D667E562F99A218000000000000000000000000"),
			uint256_ops::get_uint256("8E7D545047864AEB9CC70865E77350BBEA073E94000000000000000000000000"),
			uint256_ops::get_uint256("284B6391212D971BFF9761625058D6E86D893D50000000000000000000000000"),
			uint256_ops::get_uint256("9AA0661B605070609086D4522E0E8303010419FD000000000000000000000000"),
			uint256_ops::get_uint256("561EAF9809D66A3EFFBBA01505098689EECD519E000000000000000000000000"),
			uint256_ops::get_uint256("D6EF296EDFBF094C4D3F16F98F52170FF3D19CB8000000000000000000000000"),
			uint256_ops::get_uint256("A922D80469F3013619AEA0AF68ABC3E32D711265000000000000000000000000"),
			uint256_ops::get_uint256("AE6303E974129A6B1B941D0762337B1CAE6B0911000000000000000000000000"),
			uint256_ops::get_uint256("B3BC2F14282CBA8548833F13C107F25B34F32A4D000000000000000000000000"),
			uint256_ops::get_uint256("E500EEE1383769F06FD90BB8990C4C7F7EBCAA21000000000000000000000000"),
			uint256_ops::get_uint256("E52C56EC96E20BBAC73CCF0928E66041FA0CF619000000000000000000000000"),
			uint256_ops::get_uint256("CE5EC5BEB8B12614A0C82D9FE2A6249DC60BAAC0000000000000000000000000"),
			uint256_ops::get_uint256("FD8AEF8D55B500BA930EED4AFEC8F5280CEAD706000000000000000000000000"),
			uint256_ops::get_uint256("FE2B8A089B3DCDE898F619E72D13430DFECEBC3E000000000000000000000000"),
			uint256_ops::get_uint256("7B2E4308882F42F434D4B8BED2A9AD7BB533A429000000000000000000000000"),
			uint256_ops::get_uint256("160001BD6E0A616B783A3AAE0F9585C0C41DFDD0000000000000000000000000"),
			uint256_ops::get_uint256("B64BC7723EA8EEB95ECE351DD17BFB54EB15BB90000000000000000000000000"),
			uint256_ops::get_uint256("7AF89C666F90977DAF86E8ED895C41B24ACED94B000000000000000000000000"),
			uint256_ops::get_uint256("49E4F332552805B0FF1A709D2B3CEE7F253DF5DC000000000000000000000000"),
			uint256_ops::get_uint256("E7CA24C8B71B90101674827F55019CDBBA483AF4000000000000000000000000"),
			uint256_ops::get_uint256("FF6037B93FFD47F63BA0CF29A75FA7DBF6BBAB52000000000000000000000000"),
			uint256_ops::get_uint256("2B1840EE97DDFAADDAEB132AF7A154B51FD8B08C000000000000000000000000"),
			uint256_ops::get_uint256("89994C4A71FD5B05BA2F8BFA42B977E6C9798E1F000000000000000000000000"),
			uint256_ops::get_uint256("2A07D4FE6045DB8E1B092CFF05BDE19652779D37000000000000000000000000"),
			uint256_ops::get_uint256("95424BDA14AB2CB02B57BB5EA1FEC953D0E8F986000000000000000000000000"),
			uint256_ops::get_uint256("5F96FC1F1146EB86233E1BC84E38CA4983DE045A000000000000000000000000")
		];
		let mut fri_queue = vec![
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001012"),
			uint256_ops::get_uint256("011462121fb1387a235c051031d539ce8e750f46a279b15c06b2344d40eea3be"),
			uint256_ops::get_uint256("04a4fe0d0fc4cc7019be9cfe12dbe6e34a4a24a5277ce225cd0fca2e433e3390"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010cb"),
			uint256_ops::get_uint256("042b8c16470e756a27007c1e6e255494a908f3f27e13773c27a14c864cfad334"),
			uint256_ops::get_uint256("016ee54bcb725324d9a77a7403c50e1778c52e39418f68360388ffdb8a2e7b40"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010d5"),
			uint256_ops::get_uint256("07bf54d5d95260713415224d1ac3f43806112288793cc56f069a82dc5176f270"),
			uint256_ops::get_uint256("01d143bab1873a7d4f202199f91650095e9868722bc42a014a2a0a0cbac2a11e"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001367"),
			uint256_ops::get_uint256("00db11e36ce9d5696d7ad06c04b69bede5e7afe900d0457e57514e770d979162"),
			uint256_ops::get_uint256("0151dd6716be703689175ccb3382aee2e0e680238b8c37991fa30fc4e4d3817f"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000014e3"),
			uint256_ops::get_uint256("05d2ce3e77d3f0fbb700747c4449e2673e8f6f76de40ea692e2b905328aad6a6"),
			uint256_ops::get_uint256("0194ed93a6f7bfb2a717179a572db8206258fff8a53bf7dc0a79bb20cfddac29"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000015eb"),
			uint256_ops::get_uint256("027d00e17f162358fb96f69bb37d4d3551fd3217091463a64fad40a92c0ce228"),
			uint256_ops::get_uint256("07d314a158f16ae7f9f8ef10337bce1fb170f19d3c4f233711e2954276062b88"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001654"),
			uint256_ops::get_uint256("00583e71b55acd93a70ab68bcca029f2dd4169e4ffc5fca2d1f029be0935c59b"),
			uint256_ops::get_uint256("028636f271a6624cccabbf60a345b401bb6a6348b35446d22182d71fb02f94c2"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000017e0"),
			uint256_ops::get_uint256("05c3d803111c8d29e499ee8fb6022379a3ea6a92964013d5bc51701a35f4ce0c"),
			uint256_ops::get_uint256("0048902737f37b5f39d1ee9f1ed776883acdcac3a48cc526b83da80641c1dddb"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ad0"),
			uint256_ops::get_uint256("02f843dd049ad582234eac82a29e453e37b41f58d8ea4f6f4282f3dacd975f3f"),
			uint256_ops::get_uint256("04fc4b40f37d705cd1c9415d6d5a2833a07e6199c682491b586c5ea03f045500"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ece"),
			uint256_ops::get_uint256("066481d9ad6dbac0f9d98f65c06b82d7be056e0f01c1be76a6dbe03f55b03e7f"),
			uint256_ops::get_uint256("04f7e3be67a9c6d111c6663370aa7a11d521411b24ad2492dab13f745aaa6eab"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001eda"),
			uint256_ops::get_uint256("0294c7459dfb0e0a3ae9ab2e17b0df5d549673e15a994c9bde177f3389a32680"),
			uint256_ops::get_uint256("02733978296e6604476ed88825042d9c2fe43799c1381efe023b00cc90cf9b09"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001f3d"),
			uint256_ops::get_uint256("01a545ce07207992a0188ad8762bf0a032cc779974fc5d3d3d868a0be0051718"),
			uint256_ops::get_uint256("051faff8712b004841ebaa11f23191c9e5a1157489f01c5305ce18606948cec9"),
			uint256_ops::get_uint256("0")
		];
		let evaluation_point = uint256_ops::get_uint256("1E2ACDCED5CE1C2C6CD77A8CA31515B0A75FA8C7EFDC38C311FF00D23BF4E0F");
		let expected_root = uint256_ops::get_uint256("7FF714006C0A255A7B0CBF77E138196383ACAC52000000000000000000000000");
		let fri_step_size = 2;

		verify_fri(proof, &mut fri_queue, evaluation_point, fri_step_size, expected_root);
    }

	#[test]
	#[should_panic] // code in verify_fri is going to panic when receiving false data
    fn test_dishonest_fri_data() {
		//Simulate dishonest prover by randomly changing bits from honest data
		let proof = vec![
			uint256_ops::get_uint256("BEB1B8B9A8477DF9D44CC95A9CD5A9194C16D0238E7EC364F72487DF5C4A06"),
			uint256_ops::get_uint256("6D39056A6D89CCC48465706A45EE5937A2750962FBE839942F110B31C84D2E8"),
			uint256_ops::get_uint256("64F8F4A784F6EFCBD02E7AC435081222486EBE2D1B595684400E38B9DA76EBE"),
			uint256_ops::get_uint256("1BBFE3131F065DB9BDC25EF748602B21433EBDCA39E1A43A5C1634167B78B1B"),
			uint256_ops::get_uint256("02c700808b8247a139fc7a01493cd7517e315e48f84d0ee998870e70e554a821"),
			uint256_ops::get_uint256("700E2121B2FEB79631B2049F22B2A089D9940EDD91774F9E794431417B1B826"),
			uint256_ops::get_uint256("2717DA2FC3ECF7B7487CC13B8A26F855C15B9E21306E34016A6F77219A6B713"),
			uint256_ops::get_uint256("250072BD637E851834347BF898498B7E3C13C12176D0E64D7D5279FD19AA0BC"),
			uint256_ops::get_uint256("F98C6F3C8AB41ECC08DCDA66EE52F6748C3ED485F75292C42C9EF3487BE2E9"),
			uint256_ops::get_uint256("21CC220F4D78E164EF0C5B0C4B386E2D15D1C98FC91AF76186007DE5778B69"),
			uint256_ops::get_uint256("64C3C89230E7A8FE737C8EFDC62431195D8D1714AF1CD4392594B57DD480023"),
			uint256_ops::get_uint256("727089973A291CFEF6690C68F69B04055158914D185C4D845396C7C02F9CBB"),
			uint256_ops::get_uint256("3BACBBE67CDD11DD8A6D68842C2A1357C870B3677248C7E0E7B6C7C804542AD"),
			uint256_ops::get_uint256("182B875BC33B2BE5A47D91318DA741D4BD4EBAE30BE00992FD0816A561C0E59"),
			uint256_ops::get_uint256("228E0CE9288C457601491E4C0F6A0F4A7949EF256ED63874265943C0D321F1F"),
			uint256_ops::get_uint256("42009DF4A10A16F4F6A99E161FC3DF4ABA130616EC02FC211E8EAACEC6BF4BA"),
			uint256_ops::get_uint256("2B898C049200BC0DE48705D723F226E22272B8ADC14C0AAA19D9A0745BE717A"),
			uint256_ops::get_uint256("5dddd5106e3159be56cce000ede416abac6a580721ca6f531fc816f6c8b43a5"),
			uint256_ops::get_uint256("1EF1CAD653F4565AA78FEF3157746264BAB97657F0EF5D8683C1BFDAD6738F"),
			uint256_ops::get_uint256("34D0800CC4CA30392991D9E7B511A8619ECC35DAB68262A60A6A751F4890DDF"),
			uint256_ops::get_uint256("41F4F2D2B2159ABA46B26017D6AFC08C2F25ADEF8314C52B745D65593D15E49"),
			uint256_ops::get_uint256("6794EF0EF42141CF1C3443C06C8F90AF84E3A348675F6260EBC448DE8A15F1C"),
			uint256_ops::get_uint256("339919B13A051B00EB70EA958F16039FC1CEBCF957EAF631AE090BCC977A74D"),
			uint256_ops::get_uint256("11677B4C60A8568BF408614585967BB6A10B17A81E0B65CDF6497A634CA68C8"),
			uint256_ops::get_uint256("6AC8F371FA684892E3FB3BC8DDC953A0C32E890BB8A7999ADCE598E695CECF7"),
			uint256_ops::get_uint256("5B8C436CC72C6D3FCF29DF54C535A33DC9E025D2C65D091666A2238742BBFF"),
			uint256_ops::get_uint256("929851B18224352038A5456DA9E74C2FB49A54E3611C79286F8686149A9195"),
			uint256_ops::get_uint256("202B5FB99AAFCC8A636B7080421EABE45FAC5EE8D70052166870E0CB838BF4D"),
			uint256_ops::get_uint256("4A77F3E3CBA858F4E756A14A8B32DF599C9F3D79264E83BA94D9089CF2F609"),
			uint256_ops::get_uint256("51FA5AE4C1B87374819AB70F036C5D20EEED6E98E79E54D7C4DEC30D55FD047"),
			uint256_ops::get_uint256("51E4F114769100EFFDF74A42B94BDB0D3AF059A6EF29F3E1FB57AD5093B0172"),
			uint256_ops::get_uint256("2D2F33993E98424EF719A4DFCE79EFA542E7AA437EAFDB147B79A96C2723146"),
			uint256_ops::get_uint256("178E877AF3C131CFAED78F4772AEF4F4D89BCCC18C2C6079B2CDD2F17DBA4F1"),
			uint256_ops::get_uint256("626A818C749C63D9EC70E3BBC0499BA6572E4FB4151EF2BEE44E7A76735C150"),
			uint256_ops::get_uint256("4B19F733A98A7D69B493D323B85BA33CFB23CCAD52144B2C6BCECFC8C3A72D6"),
			uint256_ops::get_uint256("494D4527405C142914F6E87A9443209B8937B0BD4E55D595903403A76678FC2"),
			uint256_ops::get_uint256("106D18C4002F3483D9BAA5D1A81E246AF100460C000000000000000000000000"),
			uint256_ops::get_uint256("ADB892B5E8AEB7EE290B861B2F6F0CEE8CCB1848000000000000000000000000"),
			uint256_ops::get_uint256("F1CC5D70B93B28D284A175B0BD5D897776E7F353000000000000000000000000"),
			uint256_ops::get_uint256("F8FDBE5952C45BE554A91B21D95233AA05915911000000000000000000000000"),
			uint256_ops::get_uint256("86A3E788D52E4AB0DE28500CB1E146EC79CA9E8F000000000000000000000000"),
			uint256_ops::get_uint256("BB21F1F719603928B7E52B8D85CED5864A47B6DC000000000000000000000000"),
			uint256_ops::get_uint256("1172AE9710C1422BFAF17C0059ADE6BF8692B6F0000000000000000000000000"),
			uint256_ops::get_uint256("FF5F7DF9FEF047EB26C4D8A0378F240771A86BD1000000000000000000000000"),
			uint256_ops::get_uint256("D686F17593D61F6616CF6AE5FF05EF739724EBB4000000000000000000000000"),
			uint256_ops::get_uint256("C20BF718DBEA35A8DC0F110EBACF6A2A4E4D7724000000000000000000000000"),
			uint256_ops::get_uint256("C139C853F0C9FC6DD1005D59F5B27F1D6C2E39C8000000000000000000000000"),
			uint256_ops::get_uint256("E097935D108368F333A681E44449C4F77E039D2C000000000000000000000000"),
			uint256_ops::get_uint256("76F7387E37116F084F45045725C48FC43772DAC7000000000000000000000000"),
			uint256_ops::get_uint256("50E9506407B4FBA39815A053EEDAB4CE9521DE9F000000000000000000000000"),
			uint256_ops::get_uint256("F487C52E5D60BAAADAB89670967F960D0B0D9C96000000000000000000000000"),
			uint256_ops::get_uint256("B5348E1191C96E11EA81453FAB4D19DA994C76B4000000000000000000000000"),
			uint256_ops::get_uint256("E567E13F52B09FCE2F8F89FAB28DEF58905C240E000000000000000000000000"),
			uint256_ops::get_uint256("8CEDE3497DF5130EB975FF5D69113AD85A5D7AF4000000000000000000000000"),
			uint256_ops::get_uint256("CB3ACD79075359115A271247A405CE7698C761EC000000000000000000000000"),
			uint256_ops::get_uint256("26CC8E2E340F5C41AD1F316575708F309EF0EBB8000000000000000000000000"),
			uint256_ops::get_uint256("517DDB9096B892E616ADE1B6EA706590420C0405000000000000000000000000"),
			uint256_ops::get_uint256("440C117F0913F4C2722C533F3430A535BBCD5A49000000000000000000000000"),
			uint256_ops::get_uint256("215FF6449858043C9BAC1E83F03BE57E50B991DD000000000000000000000000"),
			uint256_ops::get_uint256("9772FBFF66A1B01BCDD9E424AE4344E6C925F098000000000000000000000000"),
			uint256_ops::get_uint256("B31E8ED91F27E66EC175C116A3E9093CF7F64D7A000000000000000000000000"),
			uint256_ops::get_uint256("F604CE59F41180E4ED57E43E2ACAD3965819E4A1000000000000000000000000"),
			uint256_ops::get_uint256("AA102CA2FB6E727A3D513D705E565B20F7DA44B0000000000000000000000000"),
			uint256_ops::get_uint256("52321A910C3BA8A9C66EC26C38884A313D0A5BD2000000000000000000000000"),
			uint256_ops::get_uint256("39C323743F03325B56A8D67D5AA49D657C2B0968000000000000000000000000"),
			uint256_ops::get_uint256("AF10496AE254309884DA6E4698539D24A29BA6FF000000000000000000000000"),
			uint256_ops::get_uint256("d895fbeb266d323cff4d94e1260045bdc4750fd000000000000000000000000"),
			uint256_ops::get_uint256("B0FAE3BC4EEEB03848059607CB7E75D92D4209BB000000000000000000000000"),
			uint256_ops::get_uint256("A17832669BF07D342DFC5F2BFEF61F6DF339F1ED000000000000000000000000"),
			uint256_ops::get_uint256("DEA0D82B8AA3CF1446CD95237062FBBE0E4A93A9000000000000000000000000"),
			uint256_ops::get_uint256("CC322D149F0EA1264D0954FDE18138C326E2782A000000000000000000000000"),
			uint256_ops::get_uint256("65D027918110B69EA02527CDC5A3D88A26DEBE89000000000000000000000000"),
			uint256_ops::get_uint256("2F62719B66F87F57E7718D6482FEBE2AAA6BA043000000000000000000000000"),
			uint256_ops::get_uint256("6A2A91EBC751F75B655A7988DAA471393D976BB6000000000000000000000000"),
			uint256_ops::get_uint256("D3E82DB9B4B932EF860196F02AA21594B22065E0000000000000000000000000"),
			uint256_ops::get_uint256("CD95B4629D9E44B926D600A02D667E562F99A218000000000000000000000000"),
			uint256_ops::get_uint256("8E7D545047864AEB9CC70865E77350BBEA073E94000000000000000000000000"),
			uint256_ops::get_uint256("284B6391212D971BFF9761625058D6E86D893D50000000000000000000000000"),
			uint256_ops::get_uint256("9AA0661B605070609086D4522E0E8303010419FD000000000000000000000000"),
			uint256_ops::get_uint256("561EAF9809D66A3EFFBBA01505098689EECD519E000000000000000000000000"),
			uint256_ops::get_uint256("D6EF296EDFBF094C4D3F16F98F52170FF3D19CB8000000000000000000000000"),
			uint256_ops::get_uint256("A922D80469F3013619AEA0AF68ABC3E32D711265000000000000000000000000"),
			uint256_ops::get_uint256("AE6303E974129A6B1B941D0762337B1CAE6B0911000000000000000000000000"),
			uint256_ops::get_uint256("B3BC2F14282CBA8548833F13C107F25B34F32A4D000000000000000000000000"),
			uint256_ops::get_uint256("E500EEE1383769F06FD90BB8990C4C7F7EBCAA21000000000000000000000000"),
			uint256_ops::get_uint256("E52C56EC96E20BBAC73CCF0928E66041FA0CF619000000000000000000000000"),
			uint256_ops::get_uint256("CE5EC5BEB8B12614A0C82D9FE2A6249DC60BAAC0000000000000000000000000"),
			uint256_ops::get_uint256("FD8AEF8D55B500BA930EED4AFEC8F5280CEAD706000000000000000000000000"),
			uint256_ops::get_uint256("FE2B8A089B3DCDE898F619E72D13430DFECEBC3E000000000000000000000000"),
			uint256_ops::get_uint256("7B2E4308882F42F434D4B8BED2A9AD7BB533A429000000000000000000000000"),
			uint256_ops::get_uint256("160001BD6E0A616B783A3AAE0F9585C0C41DFDD0000000000000000000000000"),
			uint256_ops::get_uint256("B64BC7723EA8EEB95ECE351DD17BFB54EB15BB90000000000000000000000000"),
			uint256_ops::get_uint256("7AF89C666F90977DAF86E8ED895C41B24ACED94B000000000000000000000000"),
			uint256_ops::get_uint256("49E4F332552805B0FF1A709D2B3CEE7F253DF5DC000000000000000000000000"),
			uint256_ops::get_uint256("E7CA24C8B71B90101674827F55019CDBBA483AF4000000000000000000000000"),
			uint256_ops::get_uint256("FF6037B93FFD47F63BA0CF29A75FA7DBF6BBAB52000000000000000000000000"),
			uint256_ops::get_uint256("2B1840EE97DDFAADDAEB132AF7A154B51FD8B08C000000000000000000000000"),
			uint256_ops::get_uint256("89994C4A71FD5B05BA2F8BFA42B977E6C9798E1F000000000000000000000000"),
			uint256_ops::get_uint256("2A07D4FE6045DB8E1B092CFF05BDE19652779D37000000000000000000000000"),
			uint256_ops::get_uint256("95424BDA14AB2CB02B57BB5EA1FEC953D0E8F986000000000000000000000000"),
			uint256_ops::get_uint256("5F96FC1F1146EB86233E1BC84E38CA4983DE045A000000000000000000000000")
		];
		let mut fri_queue = vec![
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001012"),
			uint256_ops::get_uint256("011462121fb1387a235c051031d539ce8e750f46a279b15c06b2344d40eea3be"),
			uint256_ops::get_uint256("04a4fe0d0fc4cc7019be9cfe12dbe6e34a4a24a5277ce225cd0fca2e433e3390"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010cb"),
			uint256_ops::get_uint256("042b8c16470e756a27007c1e6e255494a908f3f27e13773c27a14c864cfad334"),
			uint256_ops::get_uint256("016ee54bcb725324d9a77a7403c50e1778c52e39418f68360388ffdb8a2e7b40"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010d5"),
			uint256_ops::get_uint256("07bf54d5d95260713415224d1ac3f43806112288793cc56f069a82dc5176f270"),
			uint256_ops::get_uint256("01d143bab1873a7d4f202199f91650095e9868722bc42a014a2a0a0cbac2a11e"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001367"),
			uint256_ops::get_uint256("00db11e36ce9d5696d7ad06c04b69bede5e7afe900d0457e57514e770d979162"),
			uint256_ops::get_uint256("0151dd6716be703689175ccb3382aee2e0e680238b8c37991fa30fc4e4d3817f"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000014e3"),
			uint256_ops::get_uint256("05d2ce3e77d3f0fbb700747c4449e2673e8f6f76de40ea692e2b905328aad6a6"),
			uint256_ops::get_uint256("0194ed93a6f7bfb2a717179a572db8206258fff8a53bf7dc0a79bb20cfddac29"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000015eb"),
			uint256_ops::get_uint256("027d00e17f162358fb96f69bb37d4d3551fd3217091463a64fad40a92c0ce228"),
			uint256_ops::get_uint256("07d314a158f16ae7f9f8ef10337bce1fb170f19d3c4f233711e2954276062b88"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001654"),
			uint256_ops::get_uint256("00583e71b55acd93a70ab68bcca029f2dd4169e4ffc5fca2d1f029be0935c59b"),
			uint256_ops::get_uint256("028636f271a6624cccabbf60a345b401bb6a6348b35446d22182d71fb02f94c2"),
			uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000017e0"),
			uint256_ops::get_uint256("05c3d803111c8d29e499ee8fb6022379a3ea6a92964013d5bc51701a35f4ce0c"),
			uint256_ops::get_uint256("0048902737f37b5f39d1ee9f1ed776883acdcac3a48cc526b83da80641c1dddb"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ad0"),
			uint256_ops::get_uint256("02f843dd049ad582234eac82a29e453e37b41f58d8ea4f6f4282f3dacd975f3f"),
			uint256_ops::get_uint256("04fc4b40f37d705cd1c9415d6d5a2833a07e6199c682491b586c5ea03f045500"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ece"),
			uint256_ops::get_uint256("066481d9ad6dbac0f9d98f65c06b82d7be056e0f01c1be76a6dbe03f55b03e7f"),
			uint256_ops::get_uint256("04f7e3be67a9c6d111c6663370aa7a11d521411b24ad2492dab13f745aaa6eab"),
			uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001eda"),
			uint256_ops::get_uint256("0294c7459dfb0e0a3ae9ab2e17b0df5d549673e15a994c9bde177f3389a32680"),
			uint256_ops::get_uint256("02733978296e6604476ed88825042d9c2fe43799c1381efe023b00cc90cf9b09"),
			uint256_ops::get_uint256("00000000000000s00000000000000000000000000000000000000000000001f3d"),
			uint256_ops::get_uint256("01a545ce07207992a0188ad8762bf0a032cc779974fc5d3d3d868a0be0051718"),
			uint256_ops::get_uint256("051faff8712b004841ebaa11f23191c9e5a1157489f01c5305ce18606948cec9"),
			uint256_ops::get_uint256("0")
		];
		let evaluation_point = uint256_ops::get_uint256("1E2ACDCED5CE1C2C6CD77A8CA31515B0A75FA8C7EFDC38C311FF00D23BF4E0F");
		let expected_root = uint256_ops::get_uint256("7FF714006C0A255A7B0CBF77E138196383ACAC52000000000000000000000000");
		let fri_step_size = 2;

		verify_fri(proof, &mut fri_queue, evaluation_point, fri_step_size, expected_root);
    }

}