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
pub static FINAL_PC = INITIAL_PC + 4;

// The format of the public input, starting at OFFSET_PUBLIC_MEMORY is as follows:
//   * For each page:
//     * First address in the page (this field is not included for the first page).
//     * Page size.
//     * Page hash.
//   # All data above this line, appears in the initial seed of the proof.
//   * For each page:
//     * Cumulative product.

