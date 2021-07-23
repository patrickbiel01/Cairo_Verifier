// ---------- // Stark Verification constants. ----------
pub static N_COEFFICIENTS: usize =                      358;
pub static N_INTERACTION_ELEMENTS: usize =              3;
pub static MASK_SIZE: usize =                           200;
pub static N_ROWS_IN_MASK: usize =                      82;
pub static N_COLUMNS_IN_MASK: usize =                      22;
pub static N_COLUMNS_IN_TRACE0: usize =                      21;
pub static N_COLUMNS_IN_TRACE1: usize =                      1;
pub static CONSTRAINTS_DEGREE_BOUND: usize =                      2;
pub static N_OODS_VALUES: usize = MASK_SIZE + CONSTRAINTS_DEGREE_BOUND;
pub static N_OODS_COEFFICIENTS: usize = N_OODS_VALUES;
pub static MAX_FRI_STEP: usize = 3;
    
// ---------- // Air specific constants. ----------
pub static PUBLIC_MEMORY_STEP: usize = 8;
pub static PEDERSEN_BUILTIN_RATIO: usize = 8;
pub static PEDERSEN_BUILTIN_REPETITIONS: usize = 4;
pub static RC_BUILTIN_RATIO: usize = 8;
pub static RC_N_PARTS: usize = 8;
pub static ECDSA_BUILTIN_RATIO: usize = 512;
pub static ECDSA_BUILTIN_REPETITIONS: usize = 1;
pub static LAYOUT_CODE: usize = 6579576;
pub static LOG_CPU_COMPONENT_HEIGHT: usize = 4;