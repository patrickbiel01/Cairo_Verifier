use num256::uint256::Uint256 as Uint256;
use sha3::Keccak256;
use sha3::Digest;

mod prime_field;
mod uint256_ops;

//Most pressing Qs are:
    // Which layout are we using (0, 1, 2) or is it determined by SHARP through prev. transactions?
    // What are the values for minPoWBits and minSecurityBits

//TODO: Create and import a file that defines all the constants we use
//TODO: change every instance where i fucked up and put a decimal number instead of hex for get_uin256

/*
    Checks that the proof parameters are valid and initializes the verifier state
*/
pub fn initVerifierParams(publicInput: & Vec<Uint256>, proofParams: & Vec<Uint256>, ctx: & mut Vec<Uint256>) {
    assert!(proofParams.len() > PROOF_PARAMS_FRI_STEPS_OFFSET); //Invalid proofParams
    assert!(proofParams.len() v== PROOF_PARAMS_FRI_STEPS_OFFSET + proofParams[PROOF_PARAMS_N_FRI_STEPS_OFFSET]); //Invalid proofParams

    let logBlowupFactor = proofParams[PROOF_PARAMS_LOG_BLOWUP_FACTOR_OFFSET];
    assert!(logBlowupFactor <= 16); //logBlowupFactor must be at most 16
    assert!(logBlowupFactor >= 1); //logBlowupFactor must be at least 1

    let proofOfWorkBits = proofParams[PROOF_PARAMS_PROOF_OF_WORK_BITS_OFFSET];
    assert!(proofOfWorkBits <= 50); //proofOfWorkBits must be at most 50
    assert!(proofOfWorkBits >= minProofOfWorkBits); //minProofOfWorkBits
    // ^ TODO: Figure out what is minProofOfWorkBits or how its assigned
    assert!(proofOfWorkBits < numSecurityBits); //Proofs may not be purely based on PoW
     // ^ TODO: Figure out what is numSecurityBits or how its assigned

     let logFriLastLayerDegBound = proofParams[PROOF_PARAMS_FRI_LAST_LAYER_DEG_BOUND_OFFSET];
     assert!(logFriLastLayerDegBound <= 10); //logFriLastLayerDegBound must be at most 10

     let nFriSteps = proofParams[PROOF_PARAMS_N_FRI_STEPS_OFFSET];
     assert!(nFriSteps <= 10); //Too many fri steps
     assert!(nFriSteps > 1); //Not enough fri steps

     let mut friSteps: Vec<Uint256> = Vec::new();
     for i in 0..nFriSteps {
        friSteps.push( proofParams[PROOF_PARAMS_FRI_STEPS_OFFSET + i] );
     }

     let logTraceLength = airSpecificInit(publicInput, ctx);

     validateFriParams(friSteps, logTraceLength, logFriLastLayerDegBound);

     let nQueries = proofParams[PROOF_PARAMS_N_QUERIES_OFFSET];
     assert!(nQueries > 0); //Number of queries must be at least one
     assert!(nQueries <= MAX_N_QUERIES); //Too many queries
     assert!(nQueries * logBlowupFactor + proofOfWorkBits >= numSecurityBits); //Proof params do not satisfy security requirements



     /* Storing the verified parameters in the verifier context/state */
     for i in 0..nFriSteps {
        ctx[MM_FRI_STEPS_PTR + i] = friSteps[i];
     }
     ctx[MM_FRI_LAST_LAYER_DEG_BOUND] = fpow(get_uint256("2"), logFriLastLayerDegBound); //2^logFriLastLayerDegBound Note: no overflow for PRIME, we can use prime_field
     ctx[MM_TRACE_LENGTH] = fpow(get_uint256("2"), logTraceLength); //2^logTraceLength
     ctx[MM_BLOW_UP_FACTOR] = fpow(get_uint256("2"), logBlowupFactor); //2^logBlowupFactor
     ctx[MM_PROOF_OF_WORK_BITS] = proofOfWorkBits;
     ctx[MM_N_UNIQUE_QUERIES] = nQueries;
     // We start with log_evalDomainSize = logTraceSize and update it here.
     ctx[MM_LOG_EVAL_DOMAIN_SIZE] = logTraceLength + logBlowupFactor;
     ctx[MM_EVAL_DOMAIN_SIZE] = fpow(get_uint256("2"), ctx[MM_LOG_EVAL_DOMAIN_SIZE]);//2^ctx[MM_LOG_EVAL_DOMAIN_SIZE]
     ctx[MM_EVAL_DOMAIN_GENERATOR] = prime_field::fpow( get_generator_val(), (get_k_modulus()-1) / ctx[MM_EVAL_DOMAIN_SIZE] );
     ctx[MM_TRACE_GENERATOR] = prime_field::fpow( ctx[MM_EVAL_DOMAIN_GENERATOR], ctx[MM_BLOW_UP_FACTOR] );
}

pub fn airSpecificInit(publicInput: & Vec<Uint256>, ctx: & mut Vec<Uint256>) -> Uint256 {
    let OFFSET_PUBLIC_MEMORY = 19;
    assert!(publicInput.len() >= OFFSET_PUBLIC_MEMORY); //publicInput is too short

    // Context for generated code
    ctx[MM_OFFSET_SIZE] = fpow(get_uint256("2"), get_uint256("16"));
    ctx[MM_HALF_OFFSET_SIZE] = fpow(get_uint256("2"), get_uint256("15"));

    // Number of steps
    let logNSteps = publicInput[OFFSET_LOG_N_STEPS];
    assert!(logNSteps < 50); //Number of steps is too large
    ctx[MM_LOG_N_STEPS] = logNSteps;
    let logTraceLength = logNSteps + LOG_CPU_COMPONENT_HEIGHT;

     // Range check limits.
    ctx[MM_RC_MIN] = publicInput[OFFSET_RC_MIN];
    ctx[MM_RC_MAX] = publicInput[OFFSET_RC_MAX];
    assert!((ctx[MM_RC_MIN] <= ctx[MM_RC_MAX]); //rc_min must be <= rc_max
    assert!((ctx[MM_RC_MAX] <= ctx[MM_OFFSET_SIZE]); //rc_max out of range

     // Layout
     assert!(publicInput[OFFSET_LAYOUT_CODE] == LAYOUT_CODE); //Layout code mismatch
     //^TODO: Understand which LAYOUT_CODE is being used

     // Initial and final pc ("program" memory segment)
     ctx[MM_INITIAL_PC] = publicInput[OFFSET_PROGRAM_BEGIN_ADDR];
     ctx[MM_FINAL_PC] = publicInput[OFFSET_PROGRAM_STOP_PTR];
     // Invalid final pc may indicate that the program end was moved, or the program didn't
    // complete.
    assert!(ctx[MM_INITIAL_PC] == INITIAL_PC); // Invalid initial pc
    assert!(ctx[MM_FINAL_PC] == FINAL_PC); // Invalid final pc

    // Initial and final ap ("execution" memory segment)
    ctx[MM_INITIAL_AP] = publicInput[OFFSET_EXECUTION_BEGIN_ADDR];
    ctx[MM_FINAL_AP] = publicInput[OFFSET_EXECUTION_STOP_PTR];

     // "output" memory segment.
     let outputBeginAddr = publicInput[OFFSET_OUTPUT_BEGIN_ADDR];
     let outputStopPtr = publicInput[OFFSET_OUTPUT_STOP_PTR];
     assert!(outputBeginAddr <= outputStopPtr); //output begin_addr must be <= stop_ptr
     let bit_64 = fpow(get_uint256("2"), get_uint256("64"));
     assert!(outputStopPtr < bit_64); // Out of range output stop_ptr
     //^TODO: Maybe use u128

     // "pedersen" memory segment
     ctx[MM_INITIAL_PEDERSEN_ADDR] = publicInput[OFFSET_PEDERSEN_BEGIN_ADDR];
     assert!(ctx[MM_INITIAL_PEDERSEN_ADDR] < bit_64); // Out of range pedersen begin_addr
     let pedersenStopPtr = publicInput[OFFSET_PEDERSEN_STOP_PTR];
     let pedersenMaxStopPtr = ctx[MM_INITIAL_PEDERSEN_ADDR] + 3 * safeDiv(2 ** ctx[MM_LOG_N_STEPS], PEDERSEN_BUILTIN_RATIO); //TODO: IMPLEMENT safeDiv
     assert!(ctx[MM_INITIAL_PEDERSEN_ADDR] <= pedersenStopPtr &&  pedersenStopPtr <= pedersenMaxStopPtr); // Invalid pedersen stop_ptr

     // "range_check" memory segment
     ctx[MM_INITIAL_RC_ADDR] = publicInput[OFFSET_RANGE_CHECK_BEGIN_ADDR];
     assert!((ctx[MM_INITIAL_RC_ADDR] < bit_64); // Out of range range_check begin_addr
     let rcStopPtr = publicInput[OFFSET_RANGE_CHECK_STOP_PTR];
     le† rcMaxStopPtr = ctx[MM_INITIAL_RC_ADDR] + safeDiv(2 ** ctx[MM_LOG_N_STEPS], RC_BUILTIN_RATIO); //TODO: IMPLEMENT safeDiv
     assert!(ctx[MM_INITIAL_RC_ADDR] <= rcStopPtr && rcStopPtr <= rcMaxStopPtr); // Invalid range_check stop_ptr

     // "ecdsa" memory segment
     ctx[MM_INITIAL_ECDSA_ADDR] = publicInput[OFFSET_ECDSA_BEGIN_ADDR];
     assert!(ctx[MM_INITIAL_ECDSA_ADDR] < bit_64); // Out of range ecdsa begin_addr
     let ecdsaStopPtr = publicInput[OFFSET_ECDSA_STOP_PTR];
     let ecdsaMaxStopPtr = ctx[MM_INITIAL_ECDSA_ADDR] + 2 * safeDiv(2 ** ctx[MM_LOG_N_STEPS], ECDSA_BUILTIN_RATIO); //TODO: IMPLEMENT safeDiv
     assert!(ctx[MM_INITIAL_ECDSA_ADDR] <= ecdsaStopPtr && ecdsaStopPtr <= ecdsaMaxStopPtr); // Invalid ecdsa stop_ptr

    // Public memory
    assert(publicInput[OFFSET_N_PUBLIC_MEMORY_PAGES] >= 1 && publicInput[OFFSET_N_PUBLIC_MEMORY_PAGES] < 100000); // Invalid number of memory pages
    ctx[MM_N_PUBLIC_MEM_PAGES] = publicInput[OFFSET_N_PUBLIC_MEMORY_PAGES]

    // Compute the total number of public memory entries.
    let n_public_memory_entries = 0;
    for page in 0..ctx[MM_N_PUBLIC_MEM_PAGES] {
        let n_page_entries = publicInput[getOffsetPageSize(page)];
        assert(n_page_entries < fpow(get_uint256("2"), get_uint256("30"));); // Too many public memory entries in one page
        n_public_memory_entries += n_page_entries;
    }
    ctx[MM_N_PUBLIC_MEM_ENTRIES] = n_public_memory_entries;

    let expectedPublicInputLength = getPublicInputLength(ctx[MM_N_PUBLIC_MEM_PAGES]);
    assert!(expectedPublicInputLength == publicInput.length); // Public input length mismatch

    let lmmPublicInputPtr = MM_PUBLIC_INPUT_IDX;
    //TODO: Somehow convert this
    //assembly {
        //mstore(add(ctx, mul(add(lmmPublicInputPtr, 1), 0x20)), add(publicInput, 0x20))
    //}
    // Set public input pointer to point at the first word of the public input
    // (skipping length word)
    //We don't need or can do a ptr to publicInput so we could wiher append it to the end of ctx and store start index or use publicInput array whenever we need it

    // Pedersen's shiftPoint values
    ctx[MM_PEDERSEN__SHIFT_POINT_X] = get_uint256("49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
    ctx[MM_PEDERSEN__SHIFT_POINT_Y] = get_uint256("3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");

    ctx[MM_RC16__PERM__PUBLIC_MEMORY_PROD] = get_uint256("1");
    ctx[MM_ECDSA__SIG_CONFIG_ALPHA] = get_uint256("1"0;
    ctx[MM_ECDSA__SIG_CONFIG_BETA] = get_uint256("6f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89");
    ctx[MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X] = get_uint256("49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
    ctx[MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y] = get_uint256("3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");
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
pub fn readLastFriLayer(ctx: & mut Vec<Uint256>) {
    let fri_last_layer_deg_bound = ctx[MM_FRI_LAST_LAYER_DEG_BOUND];
    let last_layer_idx: usize = uint256_ops::to_usize(&ctx[MM_CHANNEL]);
    let mut badInput = 0;

    let prime_minus_one = prime_field::get_k_modulus() - prime_field::get_one_val();
    let channel_idx = 1 + MM_CHANNEL;
    let last_layer_idx = ctx[channel_idx];

     // Make sure all the values are valid field elements.
    let last_layer_end = last_layer_idx + fri_last_layer_deg_bound;
    let mut coefs_idx = uint256_ops::to_usize( &last_layer_idx );
    while ( coefs_idx < last_layer_end ) {
        if ctx[coefs_idx] > prime_minus_one {
            badInput = 1;
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
    ctx[new_digest_idx] = ctx[digest_idx];

     // prng.digest := keccak256(digest||lastLayerCoefs).
     // Hash the sibling data
    let mut hasher = Keccak256::new();
    let combined_data = ctx[new_digest_idx..=last_layer_end+1];
    hasher.update(&combined_data);
    let result = hasher.finalize();
    ctx[digest_idx] = Uint256::from_bytes_le( result.as_slice() );

    // prng.counter := 0.
    ctx[channel_idx + 2] = 0;

    // Note: proof pointer is not incremented until this point.
    ctx[channel_idx] = last_layer_end;

    //Invalid field element
    assert!( badInput == 0 );
    ctx[MM_FRI_LAST_LAYER_IDX] = last_layer_idx;
}




/*
    Reads query responses for nColumns from the channel with the corresponding authentication
    paths. Verifies the consistency of the authentication paths with respect to the given
    merkleRoot, and stores the query values in proofDataPtr.

    nTotalColumns is the total number of columns represented in proofDataPtr (which should be
    an array of nUniqueQueries rows of size nTotalColumns). nColumns is the number of columns
    for which data will be read by this function.
    The change to the proofDataPtr array will be as follows:
    * The first nColumns cells will be set,
    * The next nTotalColumns - nColumns will be skipped,
    * The next nColumns cells will be set,
    * The next nTotalColumns - nColumns will be skipped,
    * ...

    To set the last columns for each query simply add an offset to proofDataPtr before calling the
    function.
*/
pub fn readQueryResponsesAndDecommit(
    ctx: & mut Vec<Uint256>, 
    nTotalColumns: u64, 
    nColumns: u64, 
    proof_data_idx: u64, 
    merkleRoot: Uint256
) {
    assert( nColumns <= getNColumnsInTrace() + getNColumnsInComposition() ); //Too many columns

    let nUniqueQueries = ctx[MM_N_UNIQUE_QUERIES];
    let channel_idx = MM_CHANNEL;
    let friQueue = MM_FRI_QUEUE;
    let friQueueEnd = friQueue + nUniqueQueries * 3;
    let merkle_ptr = MM_MERKLE_QUEUE;
    let lhashMask = verify_merkle::getHashMask()
    let proofDataSkipBytes = nTotalColumns - nColumns;
    let mut proof_data_i = proof_data_idx; 
    let mut proof_idx = ctx[channel_idx];

    while (friQueue < friQueueEnd) {
        let mut hasher = Keccak256::new();
        let combined_data = ctx[proof_idx..=proof_idx+nColumns];
        hasher.update(&combined_data);
        let hash_result = hasher.finalize();
        let mut merkleLeaf = uint256_ops::bitwise_and( &l_hash_mask, &Uint256::from_bytes_le( hash_result.as_slice() );

        // If a leaf contains only 1 field element we don't hash it.
        if nColumns == 1 {
            merkleLeaf = ctx[proof_idx];
        }

        // push(queryIdx, hash(row)) to merkleQueue.
        ctx[merkle_ptr] = ctx[friQueue];
        ctx[merkle_ptr+1] = merkleLeaf;
        merkle_ptr += 2;

        // Copy query responses to proofData array.
        // This array will used in OODS
        let end = proof_idx + nColumns;
        while proof_idx < end {
            ctx[proof_data_i] = ctx[proof_idx];
            proof_data_i += 1;
            proof_idx += 1;
        }

        proof_data_i += proofDataSkipBytes;
        friQueue += 3;
    }

    ctx[channel_idx] = proof_idx;

    verify_merkle::verify_merkle(channel_idx, ctx, merkle_ptr, merkleRoot, nUniqueQueries);
}




/*
    Computes the first FRI layer by reading the query responses and calling
    the OODS contract.

    The OODS contract will build and sum boundary constraints that check that
    the prover provided the proper evaluations for the Out of Domain Sampling.

    I.e. if the prover said that f(z) = c, the first FRI layer will include
    the term (f(x) - c)/(x-z).
*/
pub fn computeFirstFriLayer(ctx: & mut Vec<Uint256>) {

}



/*
    Checks that the trace and the compostion agree at oodsPoint, assuming the prover provided us
    with the proper evaluations.

    Later, we will use boundery constraints to check that those evaluations are actully consistent
    with the commited trace and composition ploynomials.
*/
pub fn oodsConsistencyCheck(ctx: & mut Vec<Uint256>) {
    //TODO: Figure out which oodsContract address is being used

}



pub fn validateFriParams(friSteps: & mut Vec<Uint256>, logTraceLength: Uint256, logFriLastLayerDegBound: Uint256) {
    assert!(friSteps[0] == 0); //Only eta0 == 0 is currently supported

    let expectedLogDegBound = logFriLastLayerDegBound;
    let nFriSteps = friSteps.len();
    for i in 1..nFriSteps {
        let friStep = friSteps[i];
        assert!(friStep > 0); // Only the first fri step can be 0
        assert!(friStep <= 4); //Max supported fri step is 4.
        expectedLogDegBound += friStep;
    }

    // FRI starts with a polynomial of degree 'traceLength'.
    // After applying all the FRI steps we expect to get a polynomial of degree less
    // than friLastLayerDegBound.
    assert!(expectedLogDegBound == logTraceLength); //Fri params do not match trace length
}


/*
    Main driver for Verifying proof:
        - Checks arithmetization
        - Checks low-degreeness using FRI protocol
*/
pub fn verify_proof(
    proofParams: Vec<Uint256>,
    proof: Vec<Uint256>,
    taskMetadata: Vec<Uint256>,
    cairoAuxInput: Vec<Uint256>,
    cairoVerifierId: Uint256,
) {

    /* ------------ GPS Statement Verifier ----------- */
    //TODO: Figure out when is the constructor called

    //TODO: Figure out if we need to add in memory pages or not

    assert!( cairoAuxInput.len() > get_offset_n_public_mem_pages() );

    //Transform cairoAuxInput -> cairoPublic input (- z, alpha)

    //TODO: Choose numSecurityBits and minProofOfWorkBits for Stark Verifier somehow?

    /* --------- StarkVerifier.verifyProof --------- */

    let mut verifier_state: Vec<Uint256> = !vec[get_uint256("0"); MM_CONTEXT_SIZE];//Blank init size MM_CONTEXT_SIZE
    initVerifierParams(publicInput, proofParams, & mut verifier_state);

    verifier_channel::initChannel( get_channel_offset(), get_proof_offset(), get_public_input_hash(publicInput), & verifier_state );


    //Read trace commitment
    verifier_state[get_mm_trace_commitment()] = 2; //uint256(readHash(channelPtr, true));

    verifier_channel::send_field_elements(channelIdx, get_n_coefficents(), mn_coefficents_index, & mut verifier_state);

    verifier_state[ get_mm_oods_commitment() ] = 2;  //uint256(readHash(channelPtr, true));
    
    //Send out domain sampling point
    verifier_channel::send_field_elements(channelIdx, 1, mm_oods_point_index, & mut verifier_state);

    //Read the answers to the Out of Domain Sampling
    let lmm_oods_vals: Uint256 = get_mn_oods_vals();
    for i in lmm_oods_vals..(lmm_oods_vals + get_n_oods_vals() ) {
        verifier_state[i] = verifier_channel::read_field_elements(channelIdx, true);
    }

    oods_consistency_check(&mut verifier_state);

    let nFriSteps = get_fri_steps(&verifier_state).len();
    for i in 1..(nFriSteps-1) {
        verifier_channel::send_field_elements(channelIdx, 1, fri_eval_index + i, & mut verifier_state);
        verifier_state[ get_mm_fri_commitments() + i ] = 2;  //uint256(VerifierChannel.readHash(channelPtr, true));
    }

    verifier_channel::send_field_elements(channelIdx, 1, mm_fri_eval_points + nFriSteps - 1, & mut verifier_state);

    verifier_channel::verify_PoW(channelIdx, verifier_state[get_mm_proof_of_work_bits(), &verifier_state]);

    verifier_state[MM_N_UNIQUE_QUERIES] = verifier_channel::send_random_queries(
        channelPtr, verifier_state[MM_N_UNIQUE_QUERIES], verifier_state[MM_EVAL_DOMAIN_SIZE] - 1, mm_fri_queueIndex
    );

    computeFirstFriLayer(&mut verifier_state);

    fri_verify_layers(&mut verifier_state); //Fri.sol.ref
}