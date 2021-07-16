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

/* ------------- Testing Harness ------------- */
pub fn verifyFRI(
	proof: Vec<Uint256>, friQueue: &mut Vec<Uint256>, evaluationPoint: Uint256, friStepSize: usize, expectedRoot: Uint256
) {
	assert!(friStepSize <= FRI_MAX_FRI_STEP); //FRI step size too large
	/*
		The friQueue should have of 3*nQueries + 1 elements, beginning with nQueries triplets
		of the form (query_index, FRI_value, FRI_inverse_point), and ending with a single buffer
		cell set to 0, which is accessed and read during the computation of the FRI layer.
	*/
	assert!( friQueue.len() % 3 == 1); //FRI Queue must be composed of triplets plus one delimiter cell
	assert!( friQueue.len() >= 4 ); //No query to process
	let mut nQueries = friQueue.len() / 3;
	friQueue[3*nQueries] = uint256_ops::get_uint256("0");  // NOLINT: divide-before-multiply.

	assert!(evaluationPoint < prime_field::get_k_modulus()); //INVALID_EVAL_POINT

	// Verify all queries are on the same logarithmic step.
	// NOLINTNEXTLINE: divide-before-multiply.
	assert!( uint256_ops::bitwise_xor( &friQueue[0], &friQueue[3*nQueries-3] ) < friQueue[0]); //INVALID_QUERIES_RANGE

	// Queries need to be in the range [2**height .. 2**(height+1)-1] strictly incrementing.
	// i.e. we need to check that Qi+1 > Qi for each i,
	// but regarding the height range - it's sufficient to check that
	// (Q1 ^ Qn) < Q1 Which affirms that all queries are within the same logarithmic step.

	// Verify FRI values and inverses are within valid range.
	// and verify that queries are strictly incrementing.
	let mut prevQuery = uint256_ops::get_uint256("0"); //If we pass height, change to: prevQuery = 1 << height - 1;
	for i in 0..nQueries {
		assert!( friQueue[3*i] > prevQuery ); //INVALID_QUERY_VALUE
		assert!( friQueue[3*i + 1] < prime_field::get_k_modulus() ); //INVALID_FRI_VALUE
		assert!( friQueue[3*i + 2] < prime_field::get_k_modulus() ); //INVALID_FRI_INVERSE_POINT
		prevQuery = friQueue[3*i].clone();
	}

	// Verify all queries are on the same logarithmic step.
	// NOLINTNEXTLINE: divide-before-multiply.
	assert!( uint256_ops::bitwise_xor(&friQueue[0], &friQueue[3*nQueries-3])  < friQueue[0] ); //INVALID_QUERIES_RANGE

	//Copy input data to ctx
	let mut ctx: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); 2315]; //TODO: Find alternative to this
	let fri_ctx = map::MM_FRI_CTX;
	let channel_idx = map::MM_CHANNEL;
	let merkle_queue_idx = map::MM_MERKLE_QUEUE;
	let fri_queue_idx = map::MM_FRI_QUEUE;

	for i in 1..friQueue.len() {
		ctx[fri_queue_idx + i - 1] = friQueue[i].clone();
	}

	ctx[channel_idx] = uint256_ops::get_uint256("90B"); //Channel points to proof (end of array) 2315 decimal
	for i in 1..proof.len() {
		ctx.push(proof[i].clone());
	}

	for i in 2..proof.len() {
		ctx[merkle_queue_idx + i - 2] = proof[i].clone();
	}

	for i in 0..40 {
		ctx[fri_ctx+i] = ctx[merkle_queue_idx + 2*nQueries].clone();
	}
 

	init_fri_groups(fri_ctx, &mut ctx);


	nQueries = compute_next_layer(
		channel_idx, fri_queue_idx, merkle_queue_idx, nQueries, evaluationPoint, usize::pow(2, friStepSize as u32), fri_ctx, &mut ctx
	);

	verify_merkle( channel_idx,  &mut ctx, merkle_queue_idx, expectedRoot, nQueries );

	//Compute Fri Fact Hash and Store
	//bytes32 factHash;
	// assembly {
	// 	// Hash FRI outputs and add to dataToHash.
	// 	mstore(add(dataToHash, 0x60), keccak256(friQueuePtr, mul(0x60, nQueries)))
	// 	factHash := keccak256(dataToHash, 0xa0)
	// }
	//registerFact(factHash)

}
/* ------------- End Testing Harness ------------- */



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
	// ^ TODO: I guess this brings up the bigger question I have which is which layouts are used when and how can I use it here

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
	ctx[fri_group_idx + 1] = prime_field::get_k_modulus() - uint256_ops::get_uint256("1"); //PRIME - 1
	// ^ TODO: Or is it prime_field::fsub(0, 1)

	let mut last_val = uint256_ops::get_uint256("1");
	let mut last_val_inv = uint256_ops::get_uint256("1"); 

	// To compute [1, -1 (== g^n/2), g^n/4, -g^n/4, ...]
	// we compute half the elements and derive the rest using negation.
	for i in 1..MAX_COSET_SIZE/2 {
		last_val = prime_field::fmul(last_val.clone(), gen_fri_group.clone());
		last_val_inv = prime_field::fmul(last_val_inv.clone(), gen_fri_group_inv.clone());

		let idx = bit_reverse( uint256_ops::from_usize(i), FRI_MAX_FRI_STEP-1);

		ctx[fri_half_inv_group_idx + idx] = last_val_inv.clone();
		ctx[fri_group_idx + 2*idx] = last_val.clone();
		ctx[fri_group_idx + 2*idx + 1] = prime_field::get_k_modulus() - last_val.clone(); //TODO: PRIME-last_val or fsub(0, lastVal)?;

	}
}

/*
	Returns the bit reversal of num assuming it has the given number of bits.
	For example, if we have numberOfBits = 6 and num = (0b)1101 == (0b)001101,
	the function will return (0b)101100.
*/
fn bit_reverse(num: Uint256, num_of_bits: usize) -> usize { //TODO: return Uint256
	assert!( num_of_bits == 256 || num < prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::from_usize(num_of_bits) ) ); // Make sure number size is correctly specified
	//^ TODO: Do we have to check for overflow on 2^num_of_bits or use alternate method

	let mut r = 0;
	let mut n = num_of_bits;
	for _ in 0..num_of_bits {
		r = (r * 2) | (n % 2);
		n = n / 2;
	}

	return r;
}

fn get_fri_steps(ctx: &mut Vec<Uint256>) -> Vec<Uint256> {
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
	friQueueHead - friQueueHead_ + 0x60  * (# elements that were taken from the queue).
	cosetIdx - the start index of the coset that was gathered.
	cosetOffset_ - the xInv field element that corresponds to cosetIdx.
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
	//println!("coset_size-1: {}. Negated: {}", coset_size-1, negated); //TODO: The negation seems sus, should orobably test in solidity, maybe use Uint256 in rust
	let coset_idx = uint256_ops::bitwise_and( &queue_item_idx.clone(), &negated );
	//let coset_idx_usize = uint256_ops::to_usize(&coset_idx);
	let next_coset_idx = coset_idx.clone() + uint256_ops::from_usize(coset_size);
	

	// Get the algebraic coset offset:
	// I.e. given c*g^(-k) compute c, where
	//      g is the generator of the coset group.
	//      k is bitReverse(offsetWithinCoset, log2(cosetSize)).
	//
	// To do this we multiply the algebraic coset offset at the top of the queue (c*g^(-k))
	// by the group element that corresponds to the index inside the coset (g^k)
	let coset_offset = prime_field::fmul(
		ctx[fri_queue_head + 2].clone(),  //TODO: We have to read fro, ctx ...
		ctx[fri_group_idx + uint256_ops::to_usize( &(queue_item_idx.clone() - coset_idx.clone()) ) ].clone()
	);

	let mut proof_idx = uint256_ops::to_usize(&ctx[channel_idx]);

	let mut index = coset_idx.clone();
	while index < next_coset_idx {
		// Inline channel operation:
		// Assume we are going to read the next element from the proof.
		// If this is not the case add(proofPtr, 0x20) will be reverted.
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
		ctx[evals_on_coset_idx] = prime_field::mod_prime( uint256_ops::make_copy(&ctx[field_elem_idx]) ); //mod (val, PRIME)
		evals_on_coset_idx += 1;

		index += uint256_ops::get_uint256("1");
	} 

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
) { //TODO: Maybe use Uint256 for index?

	let evals_on_coset_idx = fri_ctx + FRI_CTX_TO_COSET_EVALUATIONS_OFFSET;
	let fri_half_inv_group_idx = fri_ctx + FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET;

	let mut fri_val = uint256_ops::get_uint256("0");
	let mut coset_offset = coset_offset_input;

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
	//println!("merkle_queue_idx: {}", merkle_queue_idx);
	//println!("idx_in_nxt_step: {}", idx_in_nxt_step);
	ctx[merkle_queue_idx] = idx_in_nxt_step.clone();

	let mut hash_data: Vec<u8> = vec![];
	for i in 0..fri_coset_size {
		let data_bytes = ctx[evals_on_coset_idx + i].to_bytes_le();
		for j in 0..data_bytes.len() {
			hash_data.push( data_bytes[j] );
		}
	}
	//println!("merkle_queue_idx + 1: {}", merkle_queue_idx + 1);
	//println!("hash stuff: {}", uint256_ops::bitwise_and( &get_hash_mask(), &uint256_ops::keccak_256(&hash_data)));
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