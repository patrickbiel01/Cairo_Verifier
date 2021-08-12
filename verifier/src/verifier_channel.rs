use num256::uint256::Uint256 as Uint256;

use crate::uint256_ops;
use crate::prime_field;


/* Note:
	We store the state of the channel in uint256[3] as follows:
	[0] proof pointer.
	[1] prng digest.
	[2] prng counter.
*/


fn get_prng_ptr(channel_idx: usize) -> usize {
	return channel_idx + 1;
}

pub fn init_channel(channel_offset: usize, proof_offset: usize, public_input_hash: Uint256, ctx: &mut Vec<Uint256>) {
	ctx[channel_offset] = uint256_ops::from_usize(proof_offset);
	init_prng( get_prng_ptr(channel_offset), public_input_hash, ctx );
}

pub fn send_field_elements(channel_idx: usize, n_elements: usize, target_idx_input: usize, ctx: &mut Vec<Uint256>) {
	assert!(n_elements < 0x1000000); //Overflow protection failed

	let digest_idx = channel_idx + 1;
	let counter_idx = channel_idx + 2;
	let mut target_idx = target_idx_input;

	let mask = uint256_ops::get_uint256("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

	let end_idx = target_idx + n_elements;
	while target_idx < end_idx {

		let mut field_element = prime_field::get_k_modulus();
		while field_element >= prime_field::get_k_modulus() {

        	let mut combined_data: [u8; 64] = [0; 64];
			let digest_bytes = uint256_ops::to_fixed_bytes( &ctx[digest_idx] );
			let counter_bytes = uint256_ops::to_fixed_bytes( &ctx[digest_idx+1] );
        	for i in 0..32 {
            	combined_data[i] = digest_bytes[i];
            	combined_data[i + 32] = counter_bytes[i];
        	}

			field_element = uint256_ops::bitwise_and( &mask, &uint256_ops::keccak_256(&combined_data) );

			//println!("fieldElement: {}", field_element);

			ctx[counter_idx] += uint256_ops::get_uint256("1");
		}

		ctx[target_idx] = prime_field::from_montgomery(field_element);

		//println!("target_idx: {}, ctx[target_idx]: {}", target_idx, ctx[target_idx]);
		//println!("ctx[counter_idx]: {}", ctx[counter_idx]);

		target_idx += 1;
	}
}

fn read_bytes(channel_idx: usize, mix: bool, ctx: &mut Vec<Uint256>) -> Uint256 {

	let proof_idx = uint256_ops::to_usize( &ctx[channel_idx] );
	let val = ctx[proof_idx].clone(); 
	ctx[channel_idx] = uint256_ops::from_usize( proof_idx + 1 );
	println!("proof_ptr: {}", ctx[channel_idx]);

	if mix {
		 //Prng.mixSeedWithBytes(get_prng_ptr(channelPtr), abi.encodePacked(val));
		 let digest_idx = channel_idx + 1;
		 let counter_idx = channel_idx + 2;

		 ctx[counter_idx] = val.clone();

		let mut combined_data: [u8; 64] = [0; 64];
		let bytes_1 = uint256_ops::to_fixed_bytes( &ctx[digest_idx] );
		let bytes_2 = uint256_ops::to_fixed_bytes( &ctx[digest_idx+1] );
		for i in 0..32 {
			combined_data[i] = bytes_1[i];
			combined_data[i + 32] = bytes_2[i];
		}
		// prng.digest := keccak256(digest||val), nonce was written earlier.
		ctx[digest_idx] = uint256_ops::keccak_256(&combined_data);
		// prng.counter := 0.
		ctx[counter_idx] = uint256_ops::get_uint256("0");
	}

	return val;
}
pub fn read_hash(channel_idx: usize, mix: bool, ctx: &mut Vec<Uint256>) -> Uint256 {
	let val = read_bytes(channel_idx, mix, ctx);
	return val;
}




pub fn read_field_elements(channel_idx: usize, mix: bool, ctx: &mut Vec<Uint256>) -> Uint256 {
	let result = read_bytes(channel_idx, mix, ctx).to_bytes_le();
	return prime_field::from_montgomery( Uint256::from_bytes_le( &result ) );
}


pub fn verify_pow(channel_idx: usize, pow_bits: usize, ctx: &mut Vec<Uint256>, quarter_read_ptr: usize) {
	if pow_bits == 0 {
		return;
	}

	let mut bytes_bank: [u8; 41] = [0; 41];

	//Init bytes bank wih pow_val || digest || pow_bits 
	let pow_val_bytes: [u8; 32] = uint256_ops::to_fixed_bytes( &uint256_ops::get_uint256("0123456789abcded000000000000000000000000000000000000000000000000") ); //0x0123456789abcded
	let digest_bytes = uint256_ops::to_fixed_bytes(&ctx[channel_idx + 1]);
	for i in 0..8 {
		bytes_bank[i] = pow_val_bytes[i];
	}
	for i in 0..digest_bytes.len() {
		bytes_bank[i+8] = digest_bytes[i];
	}
	bytes_bank[40] = pow_bits as u8;
	//Do a Keccak on 42 bytes of 0-7: POW requirments, 8-46: digest, 41-42: pow_bits
	let hash_bytes = uint256_ops::keccak_256(&bytes_bank).to_bytes_be();
	//Write hash to bytes_bank
	for i in 0..32 {
		bytes_bank[i] = hash_bytes[i];
	}
	

	//Do a second hash of keccak256(keccak256(0123456789abcded || digest || workBits) || nonce)
	let proof_data = ctx[ uint256_ops::to_usize(&ctx[channel_idx]) ].clone();
	let proof_data_bytes = uint256_ops::to_fixed_bytes(&proof_data);
	//println!("proof_data: {}, proof_data_bytes: {:?}", proof_data, proof_data_bytes);
	for i in 0..8 {
		bytes_bank[i + 32] = proof_data_bytes[i]; //TODO: Make sure are we writing upper bytes of lower bytes
	}
	// Keccak of 0123456789abcded || digest || workBits) || nonce
	let pow_digest = uint256_ops::keccak_256(&bytes_bank[0..40]);



	for i in 0..digest_bytes.len() {
		bytes_bank[i] = digest_bytes[i];
	}
	// prng.digest := keccak256(digest||nonce), nonce was written earlier.
	ctx[channel_idx + 1] = uint256_ops::keccak_256(&bytes_bank[0..40]);
	// prng.counter := 0.
	ctx[channel_idx + 2] = uint256_ops::get_uint256("0");
	ctx[channel_idx] += uint256_ops::get_uint256("1"); //TODO: This is incorect since 0x8 is added to proofPtr, not 0x20
	ctx[quarter_read_ptr] = uint256_ops::get_uint256("1"); 
	//TODO: Maybe we could modify copy of proof to add 24 0 bytes infront of data read

	let pow_threshold = prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::from_usize(256 - pow_bits) ); // 1 << 256 - pow_bits

	println!("pow_threshold: {}, pow_digest: {}", pow_threshold, pow_digest);

	assert!(pow_digest < pow_threshold); //Proof of work check failed
}


/*
	Sends random queries and returns an array of queries sorted in ascending order.
	Generates count queries in the range [0, mask] and returns the number of unique queries.
	Note that mask is of the form 2^k-1 (for some k).
	Note that queriesOutPtr may be (and is) inteleaved with other arrays. The stride parameter
	is passed to indicate the distance between every two entries to the queries array, i.e.
	stride = 0x20*(number of interleaved arrays).
*/
pub fn send_random_queries(
	channel_idx: usize, count: usize, mask: Uint256, queries_out_idx: usize, stride: usize, ctx: &mut Vec<Uint256>
) -> Uint256 {

	let mut shift = 0;
	let mut end_idx = queries_out_idx;
	let mut val = uint256_ops::get_uint256("0");

	for _ in 0..count {
		if shift == 0 {
			val = get_random_bytes( get_prng_ptr(channel_idx), ctx );
			shift = 8;
		}
		
		shift -= 2;
		let r_shift = val.clone() / prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::from_usize(shift) ); // val >> shift
		let query_idx = uint256_ops::bitwise_and( &mask, &r_shift );
		
		// Insert new query_idx in the correct place like insertion sort.
		let mut idx_cpy = end_idx;
		let mut curr = uint256_ops::get_uint256("0");
		while idx_cpy > queries_out_idx {
			curr = ctx[idx_cpy - stride].clone();

			if query_idx >= curr {
				break;
			}

			ctx[idx_cpy] = curr.clone();
			idx_cpy -= stride;
		}

		if query_idx != curr {
			ctx[idx_cpy] = uint256_ops::make_copy( &query_idx );
			end_idx += stride;
		} else {
			// Revert right shuffling.
			while idx_cpy < end_idx {
				ctx[idx_cpy] = uint256_ops::make_copy( &ctx[idx_cpy + stride] );
				idx_cpy += stride;
			}
		}

	}

	return uint256_ops::from_usize( (end_idx - queries_out_idx) / stride );

}








/* --------------------
	PRNG (Randomness derived from public input) 
 --------------------- */

fn init_prng(prng_offset: usize, public_input_hash: Uint256, ctx: &mut Vec<Uint256>) {
	store_prng(prng_offset, public_input_hash, uint256_ops::get_uint256("0"), ctx );
}

fn store_prng(state_idx: usize, digest: Uint256, counter: Uint256, ctx: &mut Vec<Uint256>) {
	ctx[state_idx] = digest.clone();
	ctx[state_idx + 1] = counter.clone();
}

fn load_prng(state_idx: usize, ctx: & Vec<Uint256>) -> (Uint256, Uint256) {
	return ( uint256_ops::make_copy( &ctx[state_idx] ), uint256_ops::make_copy( &ctx[state_idx + 1] ) );
}

/* Auxiliary function for get_random_bytes */
fn get_random_bytes_inner(digest: Uint256, counter: Uint256, ctx: &mut Vec<Uint256>) -> (Uint256, Uint256, Uint256) {
	let prime_mask = uint256_ops::get_uint256("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
	
	// Do: Keccak( digest || counter)
    let mut combined_data: [u8; 64] = [0; 64];
	let digest_bytes = uint256_ops::to_fixed_bytes(&digest);
	let counter_bytes = uint256_ops::to_fixed_bytes(&counter);
	for i in 0..32 {
		combined_data[i] = digest_bytes[i];
		combined_data[i + 32] = counter_bytes[i];
	}
	let hash = uint256_ops::keccak_256(&combined_data);

	return ( digest, counter + uint256_ops::get_uint256("1"), uint256_ops::bitwise_and( &hash, &prime_mask ) );
}

fn get_random_bytes(prng_idx: usize, ctx: &mut Vec<Uint256>) -> Uint256 {
	let ( digest0, counter0) = load_prng(prng_idx, ctx);

	 // returns 32 bytes (for random field elements or four queries at a time).
	 let (digest, counter, random_bytes) = get_random_bytes_inner(digest0, counter0, ctx);

	 store_prng(prng_idx, digest, counter, ctx);
	
	 return random_bytes;
}

// fn mixSeedWithBytes(prng_idx: usize, data_bytes: &[u8], ctx: &mut Vec<Uint256>) {
// 	// let digest = uint256_ops::make_copy( &ctx[prng_idx] );
// 	// TODO: Implement init_prng(prngPtr, keccak256(abi.encodePacked(digest, dataBytes)));
// }

fn get_prng_digest(prng_idx: usize, ctx: & Vec<Uint256>) -> Uint256 {
	return uint256_ops::make_copy( &ctx[prng_idx] );
}

