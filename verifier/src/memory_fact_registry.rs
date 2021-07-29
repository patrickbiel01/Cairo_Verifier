use num256::uint256::Uint256 as Uint256;
use std::collections::HashMap;

use crate::prime_field;
use crate::uint256_ops;
use crate::cairo_bootloader;
use crate::verify_proof::{N_BUILTINS, N_MAIN_ARGS, N_MAIN_RETURN_VALUES};
use crate::public_input_offsets;
use crate::memory_map as map;

//TODO: Convert decimal to hex to get_uint256

// A page based on a list of pairs (address, value)
// In this case, memoryHash = hash(address, value, address, value, address, value, ...)
pub static REGULAR_PAGE: usize = 0;
// A page based on adjacent memory cells, starting from a given addresss
// In this case, memoryHash = hash(value, value, value, ...)
pub static CONTINUOUS_PAGE: usize = 1;


static METADATA_TASKS_OFFSET: usize = 1;
static METADATA_OFFSET_TASK_OUTPUT_SIZE: usize = 0;
static METADATA_OFFSET_TASK_PROGRAM_HASH: usize = 1;
static METADATA_OFFSET_TASK_N_TREE_PAIRS: usize = 2;
static METADATA_TASK_HEADER_SIZE: usize = 3;

static METADATA_OFFSET_TREE_PAIR_N_PAGES: usize = 0;
static METADATA_OFFSET_TREE_PAIR_N_NODES: usize = 1;

/*
	Registers the fact for memory page 0, which includes:
	1. The bootloader program,
	2. Arguments and return values of main()
	3. Some of the data required for computing the task facts. which is represented in
		taskMetadata.
	Returns information on the registered fact.
	Assumptions: cairo_aux_input is connected to the public input, which is verified by
	cairoVerifierContractAddresses.
	Guarantees: taskMetadata is consistent with the public memory, with some sanity checks.
*/
pub fn register_public_memory_main_page(
	task_meta_data: & Vec<Uint256>, cairo_aux_input: & Vec<Uint256>, registry: &mut HashMap<Uint256, bool>
) -> (usize, Uint256, Uint256) {
	let prime = prime_field::get_k_modulus();
	let n_tasks = uint256_ops::to_usize(&task_meta_data[0].clone());
	assert!( n_tasks < 2usize.pow(30) ); //Invalid number of tasks

	// Public memory length
	let pub_mem_len = 
		cairo_bootloader::PROGRAM_SIZE + /*return fp and pc*/2 + N_MAIN_ARGS + N_MAIN_RETURN_VALUES + /*Number of tasks cell*/1 + 2 * n_tasks;

	let mut public_memory: Vec<Uint256> = vec![uint256_ops::get_uint256("0"); pub_mem_len * public_input_offsets::N_WORDS_PER_PUBLIC_MEMORY_ENTRY];

	let mut offset = 0;

	// Write public memory, which is a list of pairs (address, value).
	// Copy program segment to public memory
	let bootloader_prog: Vec<Uint256> = cairo_bootloader::get_bootload_program();
	for i in 0..bootloader_prog.len() {
		// Force that memory[i + INITIAL_PC] = bootloaderProgram[i].
		public_memory[offset] = uint256_ops::from_usize(i + public_input_offsets::INITIAL_PC);
		public_memory[offset + 1] = bootloader_prog[i].clone();
		offset += 2;
	}


	// Execution segment - Make sure [initial_fp - 2] = initial_fp and .
	// This is required for the "safe call" feature (that is, all "call" instructions will
	// return, even if the called function is malicious).
	// It guarantees that it's not possible to create a cycle in the call stack.
	let initial_fp = cairo_aux_input[public_input_offsets::OFFSET_EXECUTION_BEGIN_ADDR].clone();
	assert!(initial_fp >= uint256_ops::get_uint256("2")); //Invalid execution begin address
	public_memory[offset + 0] = initial_fp.clone() - uint256_ops::get_uint256("2");
	public_memory[offset + 1] = initial_fp.clone();
	// Make sure [initial_fp - 1] = 0.
	public_memory[offset + 2] = initial_fp.clone() - uint256_ops::get_uint256("1");
	public_memory[offset + 3] = uint256_ops::get_uint256("0");
	offset += 4;


	// Execution segment - main's arguments.
	public_memory[offset + 0] = initial_fp.clone();
	public_memory[offset + 1] = cairo_aux_input[public_input_offsets::OFFSET_OUTPUT_BEGIN_ADDR].clone();
	public_memory[offset + 2] = initial_fp.clone() + uint256_ops::get_uint256("1");
	public_memory[offset + 3] = cairo_aux_input[public_input_offsets::OFFSET_PEDERSEN_BEGIN_ADDR].clone();
	public_memory[offset + 4] = initial_fp.clone() + uint256_ops::get_uint256("2");
	public_memory[offset + 5] = cairo_aux_input[public_input_offsets::OFFSET_RANGE_CHECK_BEGIN_ADDR].clone();
	public_memory[offset + 6] = initial_fp.clone() + uint256_ops::get_uint256("3");
	public_memory[offset + 7] = cairo_aux_input[public_input_offsets::OFFSET_ECDSA_BEGIN_ADDR].clone();
	offset += 8;


	// Execution segment - return values
	let execution_stop_ptr = cairo_aux_input[public_input_offsets::OFFSET_EXECUTION_STOP_PTR].clone();
	public_memory[offset + 0] = execution_stop_ptr.clone() - uint256_ops::get_uint256("4");
	public_memory[offset + 1] = cairo_aux_input[public_input_offsets::OFFSET_OUTPUT_STOP_PTR].clone();
	public_memory[offset + 2] = execution_stop_ptr.clone() - uint256_ops::get_uint256("3");
	public_memory[offset + 3] = cairo_aux_input[public_input_offsets::OFFSET_PEDERSEN_STOP_PTR].clone();
	public_memory[offset + 4] = execution_stop_ptr.clone() - uint256_ops::get_uint256("2");
	public_memory[offset + 5] = cairo_aux_input[public_input_offsets::OFFSET_RANGE_CHECK_STOP_PTR].clone();
	public_memory[offset + 6] = execution_stop_ptr.clone() - uint256_ops::get_uint256("1");
	public_memory[offset + 7] = cairo_aux_input[public_input_offsets::OFFSET_ECDSA_STOP_PTR].clone();
	offset += 8;


	// Program output.
	// Check that there are enough range checks for the bootloader builtin validation.
	// Each builtin is validated for each task and each validation uses one range check
	assert!(
		cairo_aux_input[public_input_offsets::OFFSET_RANGE_CHECK_STOP_PTR] >= cairo_aux_input[public_input_offsets::OFFSET_RANGE_CHECK_BEGIN_ADDR].clone() + uint256_ops::from_usize(N_BUILTINS) * uint256_ops::from_usize(n_tasks)
	); //Range-check stop pointer should be after all range checks used for validations

	let mut output_addr = cairo_aux_input[public_input_offsets::OFFSET_OUTPUT_BEGIN_ADDR].clone();
	// Force that memory[outputAddress] = nTasks.
	public_memory[offset + 0] = output_addr.clone();
	public_memory[offset + 1] = uint256_ops::from_usize(n_tasks);
	offset += 2;
	output_addr += uint256_ops::get_uint256("1");

	let mut task_meta_data_offset = METADATA_TASKS_OFFSET;
	for task in 0..n_tasks {
		let output_size = task_meta_data[task_meta_data_offset + METADATA_OFFSET_TASK_OUTPUT_SIZE].clone();
		assert!(uint256_ops::get_uint256("2") <= output_size && output_size < prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("30") )); //Invalid task output size

		let program_hash = task_meta_data[task_meta_data_offset + METADATA_OFFSET_TASK_PROGRAM_HASH].clone();
		let n_tree_pairs = task_meta_data[task_meta_data_offset + METADATA_OFFSET_TASK_N_TREE_PAIRS].clone();
		assert!(uint256_ops::get_uint256("1") <= n_tree_pairs && n_tree_pairs < prime_field::fpow( &uint256_ops::get_uint256("2"), &uint256_ops::get_uint256("20") ) ); //Invalid number of pairs in the Merkle tree structure
		
		// Force that memory[outputAddress] = outputSize.
		public_memory[offset + 0] = output_addr.clone();
		public_memory[offset + 1] = output_size.clone();
		// Force that memory[outputAddress + 1] = programHash.
		public_memory[offset + 2] = output_addr.clone() + uint256_ops::get_uint256("1");
		public_memory[offset + 3] = program_hash;
		offset += 4;

		output_addr += output_size.clone();
		task_meta_data_offset += METADATA_TASK_HEADER_SIZE + 2 * uint256_ops::to_usize(&n_tree_pairs);		
	}

	assert!( task_meta_data.len() == task_meta_data_offset ); //Invalid length of taskMetadata
	assert!( cairo_aux_input[public_input_offsets::OFFSET_OUTPUT_STOP_PTR] == output_addr ); //Inconsistent program output length
	assert!(public_memory.len() == offset); //Not all Cairo public inputs were written

	//Register a regular memory page
	assert!(public_memory.len() < 2usize.pow(20)); //Too many memory values
	assert!(public_memory.len() % 2 == 0); //Size of memoryPairs must be even
	assert!(cairo_aux_input[cairo_aux_input.len()-2] < prime); //Invalid value of z
	assert!(cairo_aux_input[cairo_aux_input.len()-1] < prime); //Invalid value of alpha

	//Compute hash of public memory
	let (fact_hash, mem_hash, prod) = compute_fact_hash(
		&public_memory, cairo_aux_input[cairo_aux_input.len()-2].clone(), cairo_aux_input[cairo_aux_input.len()-1].clone(), prime.clone()
	);

	//Write to the fact to the hashmap/registry
	registry.insert(
		fact_hash, true
	);


	return (pub_mem_len, mem_hash, prod );
}


/*
  A fact registry for the claim:
    I know n pairs (addr, value) for which the hash of the pairs is memoryHash, and the cumulative
    product: \prod_i( z - (addr_i + alpha * value_i) ) is prod.
  The exact format of the hash depends on the type of the page
  (see MemoryPageFactRegistryConstants).
  The fact consists of (pageType, prime, n, z, alpha, prod, memoryHash, address).
  Note that address is only available for CONTINUOUS_PAGE, and otherwise it is 0.
*/


/* Registers a fact based of the given memory (address, value) pairs (REGULAR_PAGE) */
fn compute_fact_hash(
	memory_pairs: &Vec<Uint256>, z: Uint256, alpha: Uint256, prime: Uint256
) -> (Uint256, Uint256, Uint256) {
	let mem_size = memory_pairs.len() / 2;

	let mut prod = uint256_ops::get_uint256("1");

	let mem_idx = 1;

	// Each value of memoryPairs is a pair: (address, value)
	let last_idx = memory_pairs.len();

	let mut idx = mem_idx;
	while idx < last_idx {
		// Compute address + alpha * value
		let addr_val_linear_comb = prime_field::fadd(
			memory_pairs[idx].clone(), prime_field::fmul(
				memory_pairs[idx+1].clone(), alpha.clone()
			)
		);
		prod = prime_field::fmul(
			prod.clone(), z.clone() + prime.clone() - addr_val_linear_comb.clone()
		);
		idx += 2;
	}

	let mut combined_data1: Vec<u8> = vec![0; 32*memory_pairs.len()];
    for i in 0..memory_pairs.len() {
        let bytes = uint256_ops::to_fixed_bytes( &memory_pairs[i] );
        for j in 0..bytes.len() {
            combined_data1[ 32*i + j] = bytes[j];
        }
    }
    let mem_hash = uint256_ops::keccak_256(&combined_data1);


	let mut combined_data2: Vec<u8> = vec![0; 32*8];
	let mut hashing_vals: Vec<Uint256> = vec![ uint256_ops::from_usize(REGULAR_PAGE), prime.clone(), uint256_ops::from_usize(mem_size), z.clone(), alpha.clone(), prod.clone(), mem_hash.clone(), uint256_ops::get_uint256("0") ];
    for i in 0..hashing_vals.len() {
        let bytes = uint256_ops::to_fixed_bytes( &hashing_vals[i] );
        for j in 0..bytes.len() {
            combined_data2[ 32*i + j] = bytes[j];
        }
    }
    let fact_hash = uint256_ops::keccak_256(&combined_data2);

	return (fact_hash, mem_hash, prod);

}




/*
    Verifies that all the information on each public memory page (size, hash, prod, and possibly
    address) is consistent with z and alpha, by checking that the corresponding facts were
    registered on memoryPageFactRegistry.
*/
pub fn verify_memory_page_facts(ctx: & Vec<Uint256>, registry: & HashMap<Uint256, bool>) {
    let n_pub_mem_pages = uint256_ops::to_usize(&ctx[map::MM_N_PUBLIC_MEM_PAGES]);

    for page in 0..n_pub_mem_pages {
        // Fetch page values from the public input (hash, product and size)
        let mem_hash_ptr = uint256_ops::to_usize(&ctx[map::MM_PUBLIC_INPUT_PTR]) + public_input_offsets::get_offset_page_hash(page);
        let prod_ptr = uint256_ops::to_usize(&ctx[map::MM_PUBLIC_INPUT_PTR]) + public_input_offsets::get_offset_page_prod(page, n_pub_mem_pages);
        let page_size_ptr = uint256_ops::to_usize(&ctx[map::MM_PUBLIC_INPUT_PTR]) + public_input_offsets::get_offset_page_size(page);

        let page_size = ctx[page_size_ptr].clone();
        let prod = ctx[prod_ptr].clone();
        let mem_hash = ctx[mem_hash_ptr].clone();

        let mut page_addr = 0;
        if page > 0 {
            page_addr = uint256_ops::to_usize(&ctx[ 
                uint256_ops::to_usize(&ctx[map::MM_PUBLIC_INPUT_PTR]) + public_input_offsets::get_offset_page_addr(page) 
            ]);
        }
        let page_type = if page == 0 { REGULAR_PAGE } else { CONTINUOUS_PAGE };

        // Verify that a corresponding fact is registered attesting to the consistency of the page
        // information with z and alpha
        let mut combined_data: Vec<u8> = vec![0; 32*8];
        let vals: Vec<Uint256> = vec![ 
            uint256_ops::from_usize(page_type), prime_field::get_k_modulus(), page_size, ctx[map::MM_INTERACTION_ELEMENTS].clone(), ctx[map::MM_INTERACTION_ELEMENTS + 1].clone(), prod, mem_hash, uint256_ops::from_usize(page_addr)
        ];
        for i in 0..8 {
            let bytes = uint256_ops::to_fixed_bytes( &vals[i] );
            for j in 0..bytes.len() {
                combined_data[ 32*i + j] = bytes[j];
            }
        }
        let fact_hash = uint256_ops::keccak_256(&combined_data);


        assert!( registry.contains_key(&fact_hash) ); //Memory page fact was not registered
    }
}

