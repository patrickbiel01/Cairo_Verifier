/*
    We store the state of the verifer in a contiguous chunk of memory (often reffered to as 'ctx' in the code).
    The offsets of the different fields are listed below.
    E.g. The offset of the i'th hash is [mm_hashes + i].
*/

pub static CHANNEL_STATE_SIZE: usize = 3;
pub static MAX_N_QUERIES: usize =  48;
pub static FRI_QUEUE_SIZE: usize = MAX_N_QUERIES;

pub static MAX_SUPPORTED_MAX_FRI_STEP: usize = 4;

pub static MM_EVAL_DOMAIN_SIZE: usize =                          0x0;
pub static MM_BLOW_UP_FACTOR: usize =                            0x1;
pub static MM_LOG_EVAL_DOMAIN_SIZE: usize =                      0x2;
pub static MM_PROOF_OF_WORK_BITS: usize =                        0x3;
pub static MM_EVAL_DOMAIN_GENERATOR: usize =                     0x4;
pub static MM_PUBLIC_INPUT_PTR: usize =                          0x5;
pub static MM_TRACE_COMMITMENT: usize =                          0x6; // uint256[2]
pub static MM_OODS_COMMITMENT: usize =                           0x8;
pub static MM_N_UNIQUE_QUERIES: usize =                          0x9;
pub static MM_CHANNEL: usize =                                   0xa; // uint256[3]
pub static MM_MERKLE_QUEUE: usize =                              0xd; // uint256[96]
pub static MM_FRI_QUEUE: usize =                                0x6d; // uint256[144]
pub static MM_FRI_QUERIES_DELIMITER: usize =                    0xfd;
pub static MM_FRI_CTX: usize =                                  0xfe; // uint256[40]
pub static MM_FRI_STEPS_PTR: usize =                           0x126;
pub static MM_FRI_EVAL_POINTS: usize =                         0x127; // uint256[10]
pub static MM_FRI_COMMITMENTS: usize =                         0x131; // uint256[10]
pub static MM_FRI_LAST_LAYER_DEG_BOUND: usize =                0x13b;
pub static MM_FRI_LAST_LAYER_PTR: usize =                      0x13c;
pub static MM_CONSTRAINT_POLY_ARGS_START: usize =              0x13d;
pub static MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X: usize =    0x13d;
pub static MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y: usize =    0x13e;
pub static MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X: usize = 0x13f;
pub static MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__Y: usize = 0x140;
pub static MM_TRACE_LENGTH: usize =                            0x141;
pub static MM_OFFSET_SIZE: usize =                             0x142;
pub static MM_HALF_OFFSET_SIZE: usize =                        0x143;
pub static MM_INITIAL_AP: usize =                              0x144;
pub static MM_INITIAL_PC: usize =                              0x145;
pub static MM_FINAL_AP: usize =                                0x146;
pub static MM_FINAL_PC: usize =                                0x147;
pub static MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM: usize = 0x148;
pub static MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0: usize = 0x149;
pub static MM_MEMORY__MULTI_COLUMN_PERM__PERM__PUBLIC_MEMORY_PROD: usize = 0x14a;
pub static MM_RC16__PERM__INTERACTION_ELM: usize =             0x14b;
pub static MM_RC16__PERM__PUBLIC_MEMORY_PROD: usize =          0x14c;
pub static MM_RC_MIN: usize =                                  0x14d;
pub static MM_RC_MAX: usize =                                  0x14e;
pub static MM_PEDERSEN__SHIFT_POINT_X: usize =                 0x14f;
pub static MM_PEDERSEN__SHIFT_POINT_Y: usize =                 0x150;
pub static MM_INITIAL_PEDERSEN_ADDR: usize =                   0x151;
pub static MM_INITIAL_RC_ADDR: usize =                         0x152;
pub static MM_ECDSA__SIG_CONFIG_ALPHA: usize =                 0x153;
pub static MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X: usize =         0x154;
pub static MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y: usize =         0x155;
pub static MM_ECDSA__SIG_CONFIG_BETA: usize =                  0x156;
pub static MM_INITIAL_ECDSA_ADDR: usize =                      0x157;
pub static MM_TRACE_GENERATOR: usize =                         0x158;
pub static MM_OODS_POINT: usize =                              0x159;
pub static MM_INTERACTION_ELEMENTS: usize =                    0x15a; // uint256[3]
pub static MM_COEFFICIENTS: usize =                            0x15d; // uint256[358]
pub static MM_OODS_VALUES: usize =                             0x2c3; // uint256[200]
pub static MM_CONSTRAINT_POLY_ARGS_END: usize =                0x38b;
pub static MM_COMPOSITION_OODS_VALUES: usize =                 0x38b; // uint256[2]
pub static MM_OODS_EVAL_POINTS: usize =                        0x38d; // uint256[48]
pub static MM_OODS_COEFFICIENTS: usize =                       0x3bd; // uint256[202]
pub static MM_TRACE_QUERY_RESPONSES: usize =                   0x487; // uint256[1056]
pub static MM_COMPOSITION_QUERY_RESPONSES: usize =             0x8a7; // uint256[96]
pub static MM_LOG_N_STEPS: usize =                             0x907;
pub static MM_N_PUBLIC_MEM_ENTRIES: usize =                    0x908;
pub static MM_N_PUBLIC_MEM_PAGES: usize =                      0x909;
pub static MM_CONTEXT_SIZE: usize =                            0x90c; //Note: Modified from original Layout 1 to include QUARTER_READ_NUM
pub static QUARTER_READ_NUM: usize =                           0x90c; //QUARTER_READ_NUM is needed since verify_pow in verifier channel only reads from a quarter of a Uint256