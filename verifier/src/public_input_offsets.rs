use num256::uint256::Uint256 as Uint256;
use crate::uint256_ops;

/* ----------------
	The following constants are offsets of data expected in the public input.
 ----------------- */
pub static OFFSET_LOG_N_STEPS: usize =                      0;
pub static OFFSET_RC_MIN: usize =              				1;
pub static OFFSET_RC_MAX: usize =                           2;
pub static OFFSET_LAYOUT_CODE: usize =                      3;
pub static OFFSET_PROGRAM_BEGIN_ADDR: usize =               4;
pub static OFFSET_PROGRAM_STOP_PTR: usize =                 5;
pub static OFFSET_EXECUTION_BEGIN_ADDR: usize =             6;
pub static OFFSET_EXECUTION_STOP_PTR: usize =               7;
pub static OFFSET_OUTPUT_BEGIN_ADDR: usize = 				8;
pub static OFFSET_OUTPUT_STOP_PTR: usize = 					9;
pub static OFFSET_PEDERSEN_BEGIN_ADDR: usize = 				10;
pub static OFFSET_PEDERSEN_STOP_PTR: usize = 				11;
pub static OFFSET_RANGE_CHECK_BEGIN_ADDR: usize = 			12;
pub static OFFSET_RANGE_CHECK_STOP_PTR: usize = 			13;
pub static OFFSET_ECDSA_BEGIN_ADDR: usize = 				14;
pub static OFFSET_ECDSA_STOP_PTR: usize = 					15;
pub static OFFSET_PUBLIC_MEMORY_PADDING_ADDR: usize = 		16;
pub static OFFSET_PUBLIC_MEMORY_PADDING_VALUE: usize = 		17;
pub static OFFSET_N_PUBLIC_MEMORY_PAGES: usize = 			18;
pub static OFFSET_PUBLIC_MEMORY: usize = 					19;

// --- Memory Page Constants ---

pub static N_WORDS_PER_PUBLIC_MEMORY_ENTRY: usize = 2;

 // The program segment starts from 1, so that memory address 0 is kept for the null pointer.
pub static INITIAL_PC: usize = 1;

// The first Cairo instructions are:
//   ap += n_args; call main; jmp rel 0.
// As the first two instructions occupy 2 cells each, the "jmp rel 0" instruction is at
// offset 4 relative to INITIAL_PC.
pub static FINAL_PC: usize = INITIAL_PC + 4;


//Calculates hash asscoiated with public input
pub fn get_public_input_hash(public_input: &Vec<Uint256>) -> Uint256{
	// The initial seed consists of the first part of public_input. Specifically, it does not
	// include the page products (which are only known later in the process, as they depend on
	// the values of z and alpha)
	let n_pages = uint256_ops::to_usize( &public_input[OFFSET_N_PUBLIC_MEMORY_PAGES] );
	let pub_input_size_for_hash = get_offset_page_prod(0, n_pages);

	let mut combined_data: Vec<u8> = vec![0; 32*pub_input_size_for_hash];
	for i in 0..public_input.len() { //TODO: is it starting from 1 or 0?
		let bytes = uint256_ops::to_fixed_bytes( &public_input[i] );
		for j in 0..bytes.len() {
			combined_data[32*i + j] = bytes[j];
		}
	}
	return uint256_ops::keccak_256( &combined_data );

}


// The format of the public input, starting at OFFSET_PUBLIC_MEMORY is as follows:
//   * For each page:
//     * First address in the page (this field is not included for the first page).
//     * Page size.
//     * Page hash.
//   # All data above this line, appears in the initial seed of the proof.
//   * For each page:
//     * Cumulative product.
pub fn get_offset_page_size(page_id: usize) -> usize {
	return OFFSET_PUBLIC_MEMORY + 3 * page_id;
}

pub fn get_offset_page_hash(page_id: usize) -> usize {
	return OFFSET_PUBLIC_MEMORY + 3 * page_id + 1;
}

pub fn get_offset_page_addr(page_id: usize) -> usize {
	assert!(page_id >= 1); //Address of page 0 is not part of the public input
	return OFFSET_PUBLIC_MEMORY + 3 * page_id - 1;
}

pub fn get_offset_page_prod(page_id: usize, n_pages: usize) -> usize {
	return OFFSET_PUBLIC_MEMORY + 3 * n_pages - 1 + page_id;
}

pub fn get_public_input_length(n_pages: usize) -> usize {
	return OFFSET_PUBLIC_MEMORY + 4 * n_pages - 1;
}