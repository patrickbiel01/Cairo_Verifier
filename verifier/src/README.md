# Module Functionality

## verify_proof.rs
Puts all the peices of verification together into one function + Some verifier state initialization code
<br>
Note: ctx / verifier state is a big vector that contains all information about the verifier at a given time
<br>
<br>
All of 4 modules below are efficent evaluations of their namesake
### ecdsa_points_x_column.rs

### ecdsa_points_y_column.rs

### penderson_hash_x_column.rs

### penderson_hash_y_column.rs

## fri.rs
FRI Protocol Code: Verifies that the compositional polynomial sent by the prover is low-degree (bounded by degree, d) in poly-log(degree) time

### horner_eval.rs
Compute f(a) quickly, where f is a polynomial and 'a' is a real value
<br>
Note: Only used in the fri module

## verifier_channel.rs

## uint256_ops.rs
Commonly used operations for unsigned, 256-bit numbers are defined here

## prime_field.rs
Operations within a finite/galois field are defined here. Defined using: integer mod p, where p is a prime number = 0x800000000000011000000000000000000000000000000000000000000000001

## memory_fact_registry.rs
Stores facts of "memory pages" (Pairs of data either in cairoAuxInput of taskMetaData)
Guarantees taskMetadata is consistent with the public memory, with some sanity checks.

### cairo_bootloader.rs
The program is stored in a 'regular' mempry page alongside arguments and return values of main() and some of the data required for computing the task facts

## oods_check.rs

Out Of Domain Sampling on the composition polynomial sent by the prover. Serves as a consistency check to make sure the polynomial is valid

## verify_merkle.rs
Verifies a root and a sibling path of a merkle tree commitment sent by the prover during the FRI phase by hashing input values. 
The input data (i.e. merkle hashes) are expressed in the form of a queue 



## Mapping

### memory_map.rs
Defines the mapping of the verifier state (depends on layout)
<br>
Note: only layout 1 is defined currently

### stark_params.rs
Defines Verifier parameters associated with a certain layout
<br>
Note: only layout 1 is defined currently

### public_input_offsets.rs
Defines offsets for certain values within cairoAuxInput
