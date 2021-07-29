use num256::uint256::Uint256 as Uint256;
use std::collections::HashMap;

use crate::uint256_ops;
use crate::prime_field;
use crate::memory_map as map;
use crate::verify_merkle::{verify_merkle, get_hash_mask};
use crate::verifier_channel;
use crate::stark_params;
use crate::public_input_offsets as pub_input;
use crate::fri;
use crate::penderson_hash_x_column as penderson_hash_x;
use crate::penderson_hash_y_column as penderson_hash_y;
use crate::ecdsa_points_x_column as ecdsa_points_x;
use crate::ecdsa_points_y_column as ecdsa_points_y;
use crate::oods_check::{oods_check};
use crate::memory_fact_registry;



static MIN_PROOF_OF_WORK_BITS: usize = 0;
static NUM_SECURITY_BITS: usize = 80;





static PROOF_PARAMS_N_QUERIES_OFFSET:usize = 0;
static PROOF_PARAMS_LOG_BLOWUP_FACTOR_OFFSET:usize = 1;
static PROOF_PARAMS_PROOF_OF_WORK_BITS_OFFSET:usize = 2;
static PROOF_PARAMS_FRI_LAST_LAYER_DEG_BOUND_OFFSET:usize = 3;
static PROOF_PARAMS_N_FRI_STEPS_OFFSET:usize = 4;
static PROOF_PARAMS_FRI_STEPS_OFFSET:usize = 5;


pub static N_BUILTINS: usize = 4;
pub static N_MAIN_ARGS: usize = N_BUILTINS;
pub static N_MAIN_RETURN_VALUES: usize = N_BUILTINS;


//TODO: change every instance where i fucked up and put a decimal number instead of hex for get_uint256
/*
    Checks that the proof parameters are valid and initializes the verifier state
*/
pub fn init_verifier_params(public_input: & Vec<Uint256>, proof_params: & Vec<Uint256>, ctx: & mut Vec<Uint256>) {
    assert!(proof_params.len() > PROOF_PARAMS_FRI_STEPS_OFFSET); //Invalid proof_params
    assert!(proof_params.len() == PROOF_PARAMS_FRI_STEPS_OFFSET + uint256_ops::to_usize(&proof_params[PROOF_PARAMS_N_FRI_STEPS_OFFSET])); //Invalid proof_params

    let log_blowup_factor = proof_params[PROOF_PARAMS_LOG_BLOWUP_FACTOR_OFFSET].clone();
    assert!(log_blowup_factor <= uint256_ops::get_uint256("10")); //log_blowup_factor must be at most 16
    assert!(log_blowup_factor >= uint256_ops::get_uint256("1")); //log_blowup_factor must be at least 1

    let pow_bits = proof_params[PROOF_PARAMS_PROOF_OF_WORK_BITS_OFFSET].clone();
    assert!(pow_bits <= uint256_ops::get_uint256("32")); //pow_bits must be at most 50
    assert!( uint256_ops::to_usize(&pow_bits) >= MIN_PROOF_OF_WORK_BITS ); //MIN_PROOF_OF_WORK_BITS
    assert!( uint256_ops::to_usize(&pow_bits) < NUM_SECURITY_BITS ); //Proofs may not be purely based on PoW

     let log_fri_last_layer_deg_bound = proof_params[PROOF_PARAMS_FRI_LAST_LAYER_DEG_BOUND_OFFSET].clone();
     assert!(log_fri_last_layer_deg_bound <= uint256_ops::get_uint256("A")); //log_fri_last_layer_deg_bound must be at most 10

     let n_fri_steps = uint256_ops::to_usize(&proof_params[PROOF_PARAMS_N_FRI_STEPS_OFFSET]);
     assert!(n_fri_steps <= 10); //Too many fri steps
     assert!(n_fri_steps > 1); //Not enough fri steps

     let mut fri_steps: Vec<Uint256> = Vec::new();
     for i in 0..n_fri_steps {
        fri_steps.push( proof_params[PROOF_PARAMS_FRI_STEPS_OFFSET + i].clone() );
     }

     let log_trace_length = air_specific_init(public_input, ctx);

     validate_fri_params(&mut fri_steps, log_trace_length.clone(), log_fri_last_layer_deg_bound.clone());

     let n_queries = uint256_ops::to_usize(&proof_params[PROOF_PARAMS_N_QUERIES_OFFSET]);
     assert!(n_queries > 0); //Number of queries must be at least one
     assert!(n_queries <= map::MAX_N_QUERIES); //Too many queries
     assert!( uint256_ops::from_usize(n_queries) * log_blowup_factor.clone() + pow_bits.clone() >= uint256_ops::from_usize(NUM_SECURITY_BITS) ); //Proof params do not satisfy security requirements



     /* Storing the verified parameters in the verifier context/state */
     for i in 0..n_fri_steps {
        ctx[map::MM_FRI_STEPS_PTR + i] = fri_steps[i].clone();
     }
     ctx[map::MM_FRI_LAST_LAYER_DEG_BOUND] = prime_field::fpow(&uint256_ops::get_uint256("2"), &log_fri_last_layer_deg_bound); //2^log_fri_last_layer_deg_bound Note: no overflow for PRIME, we can use prime_field
     ctx[map::MM_TRACE_LENGTH] = prime_field::fpow(&uint256_ops::get_uint256("2"), &log_trace_length); //2^log_trace_length
     ctx[map::MM_BLOW_UP_FACTOR] = prime_field::fpow(&uint256_ops::get_uint256("2"), &log_blowup_factor); //2^log_blowup_factor
     ctx[map::MM_PROOF_OF_WORK_BITS] = pow_bits.clone();
     ctx[map::MM_N_UNIQUE_QUERIES] = uint256_ops::from_usize(n_queries);
     // We start with log_evalDomainSize = logTraceSize and update it here.
     ctx[map::MM_LOG_EVAL_DOMAIN_SIZE] = log_trace_length.clone() + log_blowup_factor.clone();
     ctx[map::MM_EVAL_DOMAIN_SIZE] = prime_field::fpow(&uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_EVAL_DOMAIN_SIZE]);//2^ctx[MM_LOG_EVAL_DOMAIN_SIZE]
     ctx[map::MM_EVAL_DOMAIN_GENERATOR] = prime_field::fpow( 
         &prime_field::get_generator_val(), 
         &( ( prime_field::get_k_modulus()-uint256_ops::get_uint256("1") ) / ctx[map::MM_EVAL_DOMAIN_SIZE].clone() ) 
    );
     ctx[map::MM_TRACE_GENERATOR] = prime_field::fpow( &ctx[map::MM_EVAL_DOMAIN_GENERATOR], &ctx[map::MM_BLOW_UP_FACTOR] );
}

pub fn air_specific_init(public_input: & Vec<Uint256>, ctx: & mut Vec<Uint256>) -> Uint256 {
    assert!(public_input.len() >= pub_input::OFFSET_PUBLIC_MEMORY); //public_input is too short

    // Context for generated code
    ctx[map::MM_OFFSET_SIZE] = prime_field::fpow(&uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("16"));
    ctx[map::MM_HALF_OFFSET_SIZE] = prime_field::fpow(&uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("15"));

    // Number of steps
    let log_n_steps = public_input[pub_input::OFFSET_LOG_N_STEPS].clone();
    assert!(log_n_steps < uint256_ops::get_uint256("32")); //Number of steps is too large
    ctx[map::MM_LOG_N_STEPS] = log_n_steps.clone();
    let log_trace_length = log_n_steps.clone() + uint256_ops::from_usize(stark_params::LOG_CPU_COMPONENT_HEIGHT);

     // Range check limits.
    ctx[map::MM_RC_MIN] = public_input[pub_input::OFFSET_RC_MIN].clone();
    ctx[map::MM_RC_MAX] = public_input[pub_input::OFFSET_RC_MAX].clone();
    assert!(ctx[map::MM_RC_MIN] <= ctx[map::MM_RC_MAX]); //rc_min must be <= rc_max
    assert!(ctx[map::MM_RC_MAX] <= ctx[map::MM_OFFSET_SIZE]); //rc_max out of range

     // Layout
     assert!(public_input[pub_input::OFFSET_LAYOUT_CODE] == uint256_ops::from_usize(stark_params::LAYOUT_CODE)); //Layout code mismatch

     // Initial and final pc ("program" memory segment)
     ctx[map::MM_INITIAL_PC] = public_input[pub_input::OFFSET_PROGRAM_BEGIN_ADDR].clone();
     ctx[map::MM_FINAL_PC] = public_input[pub_input::OFFSET_PROGRAM_STOP_PTR].clone();
     // Invalid final pc may indicate that the program end was moved, or the program didn't
    // complete.
    assert!(ctx[map::MM_INITIAL_PC] == uint256_ops::from_usize(pub_input::INITIAL_PC)); // Invalid initial pc
    assert!(ctx[map::MM_FINAL_PC] == uint256_ops::from_usize(pub_input::FINAL_PC)); // Invalid final pc

    // Initial and final ap ("execution" memory segment)
    ctx[map::MM_INITIAL_AP] = public_input[pub_input::OFFSET_EXECUTION_BEGIN_ADDR].clone();
    ctx[map::MM_FINAL_AP] = public_input[pub_input::OFFSET_EXECUTION_STOP_PTR].clone();

     // "output" memory segment.
     let output_begin_addr = public_input[pub_input::OFFSET_OUTPUT_BEGIN_ADDR].clone();
     let output_stop_ptr = public_input[pub_input::OFFSET_OUTPUT_STOP_PTR].clone();
     assert!(output_begin_addr <= output_stop_ptr); //output begin_addr must be <= stop_ptr
     let bit_64 = prime_field::fpow(&uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("64"));
     assert!(output_stop_ptr < bit_64); // Out of range output stop_ptr

     // "pedersen" memory segment
     ctx[map::MM_INITIAL_PEDERSEN_ADDR] = public_input[pub_input::OFFSET_PEDERSEN_BEGIN_ADDR].clone();
     assert!(ctx[map::MM_INITIAL_PEDERSEN_ADDR] < bit_64); // Out of range pedersen begin_addr
     let pedersen_stop_ptr = public_input[pub_input::OFFSET_PEDERSEN_STOP_PTR].clone();
     let pedersen_max_stop_ptr = ctx[map::MM_INITIAL_PEDERSEN_ADDR].clone() + uint256_ops::get_uint256("3") * uint256_ops::safe_div(
        &prime_field::fpow( &uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_N_STEPS]),   &uint256_ops::from_usize(stark_params::PEDERSEN_BUILTIN_RATIO)
    );
     assert!(ctx[map::MM_INITIAL_PEDERSEN_ADDR] <= pedersen_stop_ptr &&  pedersen_stop_ptr <= pedersen_max_stop_ptr); // Invalid pedersen stop_ptr

     // "range_check" memory segment
     ctx[map::MM_INITIAL_RC_ADDR] = public_input[pub_input::OFFSET_RANGE_CHECK_BEGIN_ADDR].clone();
     assert!(ctx[map::MM_INITIAL_RC_ADDR] < bit_64); // Out of range range_check begin_addr
     let rc_stop_ptr = public_input[pub_input::OFFSET_RANGE_CHECK_STOP_PTR].clone();
     let rc_max_stop_ptr = ctx[map::MM_INITIAL_RC_ADDR].clone() + uint256_ops::safe_div(
         &prime_field::fpow( &uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_N_STEPS]),   &uint256_ops::from_usize(stark_params::RC_BUILTIN_RATIO)
    );
     assert!(ctx[map::MM_INITIAL_RC_ADDR] <= rc_stop_ptr && rc_stop_ptr <= rc_max_stop_ptr); // Invalid range_check stop_ptr

     // "ecdsa" memory segment
     ctx[map::MM_INITIAL_ECDSA_ADDR] = public_input[pub_input::OFFSET_ECDSA_BEGIN_ADDR].clone();
     assert!(ctx[map::MM_INITIAL_ECDSA_ADDR] < bit_64); // Out of range ecdsa begin_addr
     let ecdsa_stop_ptr = public_input[pub_input::OFFSET_ECDSA_STOP_PTR].clone();
     let ecdsa_max_stop_ptr = ctx[map::MM_INITIAL_ECDSA_ADDR].clone() + uint256_ops::get_uint256("2") * uint256_ops::safe_div(
         &prime_field::fpow( &uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_N_STEPS]),    &uint256_ops::from_usize(stark_params::ECDSA_BUILTIN_RATIO)
    );
     assert!(ctx[map::MM_INITIAL_ECDSA_ADDR] <= ecdsa_stop_ptr && ecdsa_stop_ptr <= ecdsa_max_stop_ptr); // Invalid ecdsa stop_ptr

    // Public memory
    assert!(public_input[pub_input::OFFSET_N_PUBLIC_MEMORY_PAGES] >= uint256_ops::get_uint256("1") && public_input[pub_input::OFFSET_N_PUBLIC_MEMORY_PAGES] < uint256_ops::get_uint256("186A0")); // Invalid number of memory pages (1 < page < 100000)
    ctx[map::MM_N_PUBLIC_MEM_PAGES] = public_input[pub_input::OFFSET_N_PUBLIC_MEMORY_PAGES].clone();

    // Compute the total number of public memory entries.
    let mut n_public_memory_entries = uint256_ops::get_uint256("0");
    for page in 0.. uint256_ops::to_usize(&ctx[map::MM_N_PUBLIC_MEM_PAGES]) {
        let n_page_entries = public_input[pub_input::get_offset_page_size(page)].clone();
        assert!( n_page_entries < prime_field::fpow(&uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("30")) ); // Too many public memory entries in one page
        n_public_memory_entries += n_page_entries;
    }
    ctx[map::MM_N_PUBLIC_MEM_ENTRIES] = n_public_memory_entries;

    let expected_pub_input_len = pub_input::get_public_input_length( uint256_ops::to_usize(&ctx[map::MM_N_PUBLIC_MEM_PAGES]) );
    assert!(expected_pub_input_len == public_input.len()); // Public input length mismatch


    // Set public input pointer to point at the first word of the public input
    ctx[map::MM_PUBLIC_INPUT_PTR+1] = uint256_ops::from_usize(ctx.len());
    for i in 0..public_input.len() { //Append public input to end of verifier state / context
        ctx.push(public_input[i].clone());
    }

    // Pedersen's shiftPoint values
    ctx[map::MM_PEDERSEN__SHIFT_POINT_X] = uint256_ops::get_uint256("49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
    ctx[map::MM_PEDERSEN__SHIFT_POINT_Y] = uint256_ops::get_uint256("3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");

    ctx[map::MM_RC16__PERM__PUBLIC_MEMORY_PROD] = uint256_ops::get_uint256("1");
    ctx[map::MM_ECDSA__SIG_CONFIG_ALPHA] = uint256_ops::get_uint256("1");
    ctx[map::MM_ECDSA__SIG_CONFIG_BETA] = uint256_ops::get_uint256("6f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89");
    ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X] = uint256_ops::get_uint256("49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
    ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y] = uint256_ops::get_uint256("3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");

    return log_trace_length;
}


/*
    Reads the last FRI layer (i.e. the polynomial's coefficients) from the channel.
    This differs from standard reading of channel field elements in several ways:
    -- The digest is updated by hashing it once with all coefficients simultaneously, rather than
        iteratively one by one.
    -- The coefficients are kept in Montgomery form, as is the case throughout the FRI
        computation.
    -- The coefficients are not actually read and copied elsewhere, but rather only a pointer to
        their location in the channel is stored.
*/
pub fn read_last_fri_layer(ctx: & mut Vec<Uint256>) {
    let fri_last_layer_deg_bound = ctx[map::MM_FRI_LAST_LAYER_DEG_BOUND].clone();
    let last_layer_idx: usize = uint256_ops::to_usize(&ctx[map::MM_CHANNEL]);
    //let last_layer_idx = ctx[channel_idx];
    let mut bad_input = 0;

    let prime_minus_one = prime_field::get_k_modulus() - prime_field::get_one_val();
    let channel_idx = 1 + map::MM_CHANNEL;

     // Make sure all the values are valid field elements.
    let last_layer_end = last_layer_idx + uint256_ops::to_usize( &fri_last_layer_deg_bound );
    let mut coefs_idx = last_layer_idx;
    while coefs_idx < last_layer_end {
        if ctx[coefs_idx] > prime_minus_one {
            bad_input = 1;
            break;
        }
        coefs_idx += 1;
    }

    // Copy the digest to the proof area
    // (store it before the coefficients - this is done because
    // keccak256 needs all data to be consecutive),
    // then hash and place back in digest_idx.
    let new_digest_idx = last_layer_idx - 1;
    let digest_idx = channel_idx + 1;
    // Overwriting the proof to minimize copying of data.
    ctx[new_digest_idx] = ctx[digest_idx].clone();

     // prng.digest := keccak256(digest||lastLayerCoefs).
     // Hash the sibling data
    let mut combined_data: Vec<u8> = vec![0; 32*(last_layer_end+2 - new_digest_idx)];
    for i in new_digest_idx..=last_layer_end+1 {
        let bytes = uint256_ops::to_fixed_bytes( &ctx[i] );
        for j in 0..bytes.len() {
            combined_data[ 32*(i-new_digest_idx) + j] = bytes[j];
        }
    }
    ctx[digest_idx] = uint256_ops::keccak_256(&combined_data);

    // prng.counter := 0.
    ctx[channel_idx + 2] = uint256_ops::get_uint256("0");

    // Note: proof pointer is not incremented until this point.
    ctx[channel_idx] = uint256_ops::from_usize(last_layer_end);

    //Invalid field element
    assert!( bad_input == 0 );
    ctx[map::MM_FRI_LAST_LAYER_PTR] = uint256_ops::from_usize(last_layer_idx);
}




/*
    Reads query responses for n_columns from the channel with the corresponding authentication
    paths. Verifies the consistency of the authentication paths with respect to the given
    merkle_root, and stores the query values in proofDataPtr.

    n_total_columns is the total number of columns represented in proofDataPtr (which should be
    an array of n_unique_queries rows of size n_total_columns). n_columns is the number of columns
    for which data will be read by this function.
    The change to the proofDataPtr array will be as follows:
    * The first n_columns cells will be set,
    * The next n_total_columns - n_columns will be skipped,
    * The next n_columns cells will be set,
    * The next n_total_columns - n_columns will be skipped,
    * ...

    To set the last columns for each query simply add an offset to proofDataPtr before calling the
    function.
*/
pub fn read_query_responses_and_decommit(
    ctx: & mut Vec<Uint256>, 
    n_total_columns: usize, 
    n_columns: usize, 
    proof_data_idx: usize, 
    merkle_root: Uint256
) {
    assert!( n_columns <= stark_params::N_COLUMNS_IN_MASK + stark_params::CONSTRAINTS_DEGREE_BOUND ); //Too many columns

    let n_unique_queries = ctx[map::MM_N_UNIQUE_QUERIES].clone();
    let channel_idx = map::MM_CHANNEL;
    let mut fri_queue = map::MM_FRI_QUEUE;
    let fri_queue_end = fri_queue + uint256_ops::to_usize(&n_unique_queries) * 3;
    let mut merkle_ptr = map::MM_MERKLE_QUEUE;
    let l_hash_mask = get_hash_mask();
    let proof_data_skip_bytes = n_total_columns - n_columns;
    let mut proof_data_i = proof_data_idx; 
    let mut proof_idx = uint256_ops::to_usize(&ctx[channel_idx]);

    while fri_queue < fri_queue_end {
        let mut combined_data: Vec<u8> = vec![0; 32*n_columns];
        for i in proof_idx..=proof_idx+n_columns {
            let bytes = uint256_ops::to_fixed_bytes( &ctx[i] );
            for j in 0..bytes.len() {
                combined_data[ 32*(i-proof_idx) + j] = bytes[j];
            }
        }
        let mut merkle_leaf = uint256_ops::bitwise_and( &l_hash_mask, &uint256_ops::keccak_256(&combined_data) );

        // If a leaf contains only 1 field element we don't hash it.
        if n_columns == 1 {
            merkle_leaf = ctx[proof_idx].clone();
        }

        // push(queryIdx, hash(row)) to merkleQueue.
        ctx[merkle_ptr] = ctx[fri_queue].clone();
        ctx[merkle_ptr+1] = merkle_leaf;
        merkle_ptr += 2;

        // Copy query responses to proofData array.
        // This array will used in OODS
        let end = proof_idx + n_columns;
        while proof_idx < end {
            ctx[proof_data_i] = ctx[proof_idx].clone();
            proof_data_i += 1;
            proof_idx += 1;
        }

        proof_data_i += proof_data_skip_bytes;
        fri_queue += 3;
    }

    ctx[channel_idx] = uint256_ops::from_usize(proof_idx);

    verify_merkle(channel_idx, ctx, merkle_ptr, merkle_root,  uint256_ops::to_usize(&n_unique_queries));
}




/*
    Computes the first FRI layer by reading the query responses and calling
    the OODS contract.

    The OODS contract will build and sum boundary constraints that check that
    the prover provided the proper evaluations for the Out of Domain Sampling.

    I.e. if the prover said that f(z) = c, the first FRI layer will include
    the term (f(x) - c)/(x-z).
*/
pub fn compute_first_fri_layer(ctx: & mut Vec<Uint256>) {
    //Prepare evaluation point
    adjust_query_indicies_and_prepare_eval_points(ctx);

    //Read and decommit trace
    read_query_responses_and_decommit(
        ctx, 
        stark_params::N_COLUMNS_IN_MASK, 
        stark_params::N_COLUMNS_IN_TRACE0, 
        map::MM_TRACE_QUERY_RESPONSES, 
        ctx[map::MM_TRACE_COMMITMENT].clone()
    );

    if stark_params::N_COLUMNS_IN_TRACE1 > 0 { //true - hash (simulated) interaction
        //Read and decommit second trace
        read_query_responses_and_decommit(
            ctx, 
            stark_params::N_COLUMNS_IN_MASK, 
            stark_params::N_COLUMNS_IN_TRACE1, 
            map::MM_TRACE_QUERY_RESPONSES + stark_params::N_COLUMNS_IN_TRACE0, 
            ctx[map::MM_TRACE_COMMITMENT+1].clone()
        );
    }

    //Read and decommit composition polynomial
    read_query_responses_and_decommit(
        ctx, 
        stark_params::CONSTRAINTS_DEGREE_BOUND, 
        stark_params::CONSTRAINTS_DEGREE_BOUND, 
        map::MM_COMPOSITION_QUERY_RESPONSES,
        ctx[map::MM_OODS_COMMITMENT].clone()
    );

    //Make sure the prover provided proper evaluations
    oods_check(ctx);
}

/*
    Adjusts the query indices and generates evaluation points for each query index.

    Indices adjustment:
        The query indices adjustment is needed because both the Merkle verification and FRI
        expect queries "full binary tree in array" indices.
        The adjustment is simply adding evalDomainSize to each query.
        Note that evalDomainSize == 2^(#FRI layers) == 2^(Merkle tree hight).

    evalPoints generation:
        for each query index "idx" we compute the corresponding evaluation point:
            g^(bitReverse(idx, log_evalDomainSize).
*/
fn adjust_query_indicies_and_prepare_eval_points(ctx: & mut Vec<Uint256>) {
    let n_unique_queries = uint256_ops::to_usize(&ctx[map::MM_N_UNIQUE_QUERIES]);
    let mut fri_queue = map::MM_FRI_QUEUE;
    let fri_queue_end = fri_queue + 3*n_unique_queries;
    let eval_domain_size = uint256_ops::to_usize(&ctx[map::MM_EVAL_DOMAIN_SIZE]);
    let log_eval_domain_size = uint256_ops::to_usize(&ctx[map::MM_LOG_EVAL_DOMAIN_SIZE]);
    let eval_domain_generator = ctx[map::MM_EVAL_DOMAIN_GENERATOR].clone();

    while fri_queue < fri_queue_end {
        let mut query_idx = uint256_ops::to_usize(&ctx[fri_queue]);

        // // Adjust queryIdx, see comment in function description.
        let adjusted_query_idx = query_idx + eval_domain_size;

        // Compute the evaluation point corresponding to the current queryIdx. 
        ctx[map::MM_OODS_EVAL_POINTS] = prime_field::fpow(
            &eval_domain_generator, & uint256_ops::from_usize( bit_reverse(query_idx, log_eval_domain_size) )
        );

        fri_queue += 3;
    }
}
fn bit_reverse(num: usize, num_of_bits: usize) -> usize {
	assert!( num_of_bits == 256 || num < uint256_ops::to_usize(&prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::from_usize(num_of_bits) )) ); // Make sure number size is correctly specified

	let mut r = 0;
	let mut n = num;
	for _ in 0..num_of_bits {
		r = (r * 2) | (n % 2);
		n = n / 2;
	}

	return r;
}






/*
    Checks that the trace and the compostion agree at oodsPoint, assuming the prover provided us
    with the proper evaluations.

    Later, we will use boundery constraints to check that those evaluations are actully consistent
    with the commited trace and composition ploynomials.
*/
pub fn oods_consistency_check(ctx: & mut Vec<Uint256>) {
    //TODO: Implement verifyMemoryPageFacts
    //verifyMemoryPageFacts(ctx);

    let oods_point = ctx[map::MM_OODS_POINT].clone();

    // The number of copies in the pedersen hash periodic columns is
    // nSteps / PEDERSEN_BUILTIN_RATIO / PEDERSEN_BUILTIN_REPETITIONS
    let n_penderson_hash_copies = uint256_ops::safe_div(
       &prime_field::fpow(&uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_N_STEPS]), 
        &uint256_ops::from_usize(stark_params::PEDERSEN_BUILTIN_RATIO * stark_params::PEDERSEN_BUILTIN_REPETITIONS)
    );
    let z_point_pow_penderson = prime_field::fpow( &oods_point, &n_penderson_hash_copies );

    ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X] = penderson_hash_x::compute(z_point_pow_penderson.clone());
    ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y] = penderson_hash_y::compute(z_point_pow_penderson.clone());

    // The number of copies in the ECDSA signature periodic columns is
    // nSteps / ECDSA_BUILTIN_RATIO / ECDSA_BUILTIN_REPETITIONS
    let n_ecdsa_signature_copy = uint256_ops::safe_div(
        &prime_field::fpow(&uint256_ops::get_uint256("2"), &ctx[map::MM_LOG_N_STEPS]),
        &uint256_ops::from_usize(stark_params::ECDSA_BUILTIN_RATIO * stark_params::ECDSA_BUILTIN_REPETITIONS)
    );
    let z_point_pow_ecdsa = prime_field::fpow( &oods_point, &n_ecdsa_signature_copy );

    ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X] = ecdsa_points_x::compute(z_point_pow_ecdsa.clone());
    ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__Y] = ecdsa_points_y::compute(z_point_pow_ecdsa.clone());

    ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM] = ctx[map::MM_INTERACTION_ELEMENTS].clone();
    ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0] = ctx[map::MM_INTERACTION_ELEMENTS + 1].clone();
    ctx[map::MM_RC16__PERM__INTERACTION_ELM] = ctx[map::MM_INTERACTION_ELEMENTS + 2].clone();

    ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__PUBLIC_MEMORY_PROD] = uint256_ops::get_uint256("TODO: Implement computePublicMemoryQuotient");//TODO: Implement computePublicMemoryQuotient(ctx);

    let composition_from_trace_value = uint256_ops::get_uint256("TODO: Unimlpemented comntract"); //TODO: Implement polynomial contraints contract to obtain composition_from_trace_value

    let claimed_composition = prime_field::fadd(
        ctx[map::MM_OODS_VALUES + stark_params::MASK_SIZE].clone(),
        prime_field::fmul(
            oods_point.clone(), ctx[map::MM_OODS_VALUES + stark_params::MASK_SIZE + 1].clone()
        )
    );

    assert!( composition_from_trace_value == claimed_composition ); //claimedComposition does not match trace
}



pub fn validate_fri_params(fri_steps: & mut Vec<Uint256>, log_trace_length: Uint256, log_fri_last_layer_deg_bound: Uint256) {
    assert!(fri_steps[0] == uint256_ops::get_uint256("0")); //Only eta0 == 0 is currently supported

    let mut expected_log_deg_bound = log_fri_last_layer_deg_bound;
    let n_fri_steps = fri_steps.len();
    for i in 1..n_fri_steps {
        let fri_step = fri_steps[i].clone();
        assert!(fri_step > uint256_ops::get_uint256("0")); // Only the first fri step can be 0
        assert!(fri_step <= uint256_ops::get_uint256("4")); //Max supported fri step is 4.
        expected_log_deg_bound += fri_step;
    }

    // FRI starts with a polynomial of degree 'traceLength'.
    // After applying all the FRI steps we expect to get a polynomial of degree less
    // than friLastLayerDegBound.
    assert!(expected_log_deg_bound == log_trace_length); //Fri params do not match trace length
}












/*
    Interace for Verifying Proof:
        - Checks arithmetization
        - Checks low-degreeness using FRI protocol
*/
pub fn verify_proof(
    proof_params: Vec<Uint256>,
    proof: Vec<Uint256>,
    task_meta_data: Vec<Uint256>,
    cairo_aux_input: Vec<Uint256>,
    cairo_verifier_id: Uint256,
) {

    /* ------------ GPS Statement Verifier ----------- */

    //TODO: Some asserts from verifyProofAndRegister

    //TODO: Create public input (cairo_aux_inoput w/o z and alpha)

    /* Transform cairo_aux_input -> cairoPublic input (- z, alpha) */
    // The values z and alpha are used only for the fact registration of the main page.
    // They are not needed in the auxiliary input of CpuVerifier as they are computed there.
    // Create a copy of cairo_aux_input without z and alpha.
    let mut cairo_pub_input: Vec<Uint256> = vec![ uint256_ops::get_uint256("0"); cairo_aux_input.len()-2 ];
    for i in 0..cairo_aux_input.len()-2 {
        cairo_pub_input[i] = cairo_aux_input[i].clone();
    }

    // Process public memory and store facts of registered pages in hashmap
    let mut memory_page_fact_reg: HashMap<Uint256, bool> = HashMap::new();
    let (pub_mem_len, mem_hash, prod) = memory_fact_registry::register_public_memory_main_page(
        &task_meta_data, &cairo_aux_input, &mut memory_page_fact_reg
    );


    //TODO: Some asserts from verifyProofAndRegister

    /* --------- StarkVerifier.verifyProof --------- */

    let mut verifier_state: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); map::MM_CONTEXT_SIZE];//Blank init size MM_CONTEXT_SIZE
    init_verifier_params(&cairo_pub_input, &proof_params, & mut verifier_state);

    let channel_idx = map::MM_CHANNEL;

    //Append the proof to the end of the verifier state and store a pointer there
    let proof_idx = verifier_state.len();
    for i in 0..proof.len() {
        verifier_state.push( proof[i].clone() );
    }

    //Init the channel
    verifier_channel::init_channel( channel_idx, proof_idx, pub_input::get_public_input_hash(&cairo_pub_input), &mut verifier_state );

    //Read trace commitment
    verifier_state[map::MM_TRACE_COMMITMENT] = verifier_channel::read_hash(channel_idx, true, &mut verifier_state);


     if stark_params::N_COLUMNS_IN_TRACE1 > 0 { //true - has (simulated) interaction w/ prover
        // Send interaction elements
        verifier_channel::send_field_elements(channel_idx, stark_params::N_INTERACTION_ELEMENTS, map::MM_INTERACTION_ELEMENTS, & mut verifier_state);

        // Read second trace commitment
        verifier_state[map::MM_TRACE_COMMITMENT + 1] = verifier_channel::read_hash(channel_idx, true, &mut verifier_state);
     }


    verifier_channel::send_field_elements(channel_idx, stark_params::N_COEFFICIENTS, map::MM_COEFFICIENTS, & mut verifier_state);

    verifier_state[map::MM_OODS_COMMITMENT] = verifier_channel::read_hash(channel_idx, true, &mut verifier_state);
    

    //Send out domain sampling point
    verifier_channel::send_field_elements(channel_idx, 1, map::MM_OODS_POINT, & mut verifier_state);


    //Read the answers to the Out of Domain Sampling
    let lmm_oods_vals: usize = map::MM_OODS_VALUES;
    for i in lmm_oods_vals..(lmm_oods_vals + stark_params::N_OODS_VALUES) {
        verifier_state[i] = verifier_channel::read_field_elements(channel_idx, true, &mut verifier_state);
    }


    oods_consistency_check(&mut verifier_state);


    verifier_channel::send_field_elements(channel_idx, stark_params::N_OODS_COEFFICIENTS, map::MM_OODS_COEFFICIENTS, & mut verifier_state);
    verifier_state[map::MM_FRI_COMMITMENTS] = verifier_channel::read_hash(channel_idx, true, &mut verifier_state);


    let n_fri_steps = fri::get_fri_steps(&mut verifier_state).len();
    for i in 1..(n_fri_steps-1) {
        verifier_channel::send_field_elements(channel_idx, 1, map::MM_FRI_EVAL_POINTS + i, & mut verifier_state);
        verifier_state[ map::MM_FRI_COMMITMENTS + i ] = verifier_channel::read_hash(channel_idx, true, &mut verifier_state);
    }


    // Send last random FRI evaluation point.
    verifier_channel::send_field_elements(channel_idx, 1, map::MM_FRI_EVAL_POINTS + n_fri_steps - 1, & mut verifier_state);


    // Read FRI last layer commitment.
    read_last_fri_layer(&mut verifier_state);


     // Generate queries.
    verifier_channel::verify_pow( channel_idx, uint256_ops::to_usize(&verifier_state[map::MM_PROOF_OF_WORK_BITS]), &mut verifier_state );

    verifier_state[map::MM_N_UNIQUE_QUERIES] = verifier_channel::send_random_queries(
        channel_idx, uint256_ops::to_usize(&verifier_state[map::MM_N_UNIQUE_QUERIES]), verifier_state[map::MM_EVAL_DOMAIN_SIZE].clone()-uint256_ops::get_uint256("1"), map::MM_FRI_QUEUE, 3, &mut verifier_state
    );


    compute_first_fri_layer(&mut verifier_state);


    fri::fri_verify_layers(&mut verifier_state);


    /* --------- End of StarkVerifier.verifyProof --------- */

    //TODO: registerGpsFact? and somwhow ensure the program_hash was encoded in the proof data
    //Maybe get the program hashes from register_public_memory_main_page and iterate through tasks

}