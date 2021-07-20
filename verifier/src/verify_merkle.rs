use num256::uint256::Uint256 as Uint256;

use crate::uint256_ops;



pub fn get_hash_mask() -> Uint256 {
    return uint256_ops::get_uint256("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000");
}

/*
      Verifies a Merkle tree decommitment for n leaves in a Merkle tree with N leaves.
      The inputs data sits in the queue at queuePtr.
      Each slot in the queue contains a 32 bytes leaf index and a 32 byte leaf value.

      The indices need to be in the range [N..2*N-1] and strictly incrementing.
      Decommitments are read from the channel in the ctx.
      The input data is destroyed during verification.

      Queue Structure:
      0         1
      [Index, Hash/Value]
      [Index, Hash/Value]
*/
pub fn verify_merkle(
    channel_idx: usize,
    ctx: &mut Vec<Uint256>,
    queue_idx: usize,
    root: Uint256,
    unique_queries: usize
) -> Uint256 {

    let l_hash_mask = get_hash_mask();

    let max_merkle_verifier_queries: usize = 128;
    assert!(unique_queries <= max_merkle_verifier_queries); //TOO_MANY_MERKLE_QUERIES

    let hashes_index: usize = queue_idx + 1;
    let slot_size: usize = 2;
    let queue_size: usize = slot_size * unique_queries;

    // The items are in slots [0, n-1].
    let mut rd_idx: usize = 0;
    let mut wr_idx: usize = 0; 

    let mut index: Uint256 = ctx[queue_idx + rd_idx].clone(); // Should be 1028 for test data

    //println!("value of index (Should be greater than 1): {}. It is obtained at ctx[{}]", index, queue_idx + rd_idx);

    //Convert ctx[channel_idx] from uint256 -> usize
    let mut proof_idx = uint256_ops::to_usize( &ctx[channel_idx] );

    //Storing 2 Uint256's worth of data about sibling leaves
    let mut sibling_data: Vec<[u8; 32]> = vec![ [0; 32], [0; 32] ];

    // Iterate the queue until we hit the root.
    while index > uint256_ops::get_uint256("1") {
        //Bitwise not of a = xor(a, 1) = Left or right neighbour of a
        let sibling_index = uint256_ops::to_usize(&index) ^ 1;
        let sibling_offset = sibling_index % 2; //0 or 1 


        // Store the hash corresponding to index in the correct slot.
        // 0 if index is even and 0x20 if index is odd.
        // The hash of the sibling will be written to the other slot.
        //mstore(xor(0x20, sibblingOffset), mload(add(rd_idx, hashesPtr))) // TODO: check to_fixed_bytes
        sibling_data[1 ^ sibling_offset] = uint256_ops::to_fixed_bytes( &ctx[rd_idx + hashes_index] );
        rd_idx = ( rd_idx + slot_size ) % queue_size;

         // Inline channel operation:
        // Assume we are going to read a new hash from the proof.
        // If this is not the case proof += 1 will be reverted.
        let mut new_hash_index = proof_idx;
        proof_idx += 1;

        // Push index/2 into the queue, before reading the next index.
        // The order is important, as otherwise we may try to read from an empty queue (in
        // the case where we are working on one item).
        // wr_idx will be updated after writing the relevant hash to the queue.
        ctx[queue_idx + wr_idx] = index / uint256_ops::get_uint256("2");

        index = ctx[queue_idx + rd_idx].clone();

        //println!("index: {}. sibling_index: {}", index, sibling_index);

        if index == Uint256::from_bytes_le( &sibling_index.to_le_bytes() ) {
            new_hash_index = hashes_index + rd_idx;
            // Revert reading from proof.
            proof_idx -= 1;
            rd_idx = (rd_idx + slot_size) % queue_size;

            // Index was consumed, read the next one.
            // Note that the queue can't be empty at this point.
            // The index of the parent of the current node was already pushed into the
            // queue, and the parent is never the sibling.
            index = ctx[queue_idx + rd_idx].clone();
        }

        // Store the new hash at sibling offset
        //println!("new_hash_index: {}", new_hash_index);
        sibling_data[sibling_offset] = uint256_ops::to_fixed_bytes( &ctx[new_hash_index] );  //TODO: Decide wheather to use LE or BE bits in representation

        
        // Hash the sibling data
        let mut combined_data: [u8; 64] = [0; 64];
        for i in 0..31 {
            combined_data[i] = sibling_data[0][i];
            combined_data[i + 32] = sibling_data[1][i];
        }
        let sibling_hash = uint256_ops::keccak_256(&combined_data);

        // Push the new hash to the end of the queue.
        ctx[hashes_index + wr_idx] = uint256_ops::bitwise_and( &l_hash_mask, &sibling_hash );

        //println!("One value of calculated hash: {}", ctx[hashes_index + wr_idx].clone());

        wr_idx = (wr_idx + slot_size) % queue_size;

    }

    let hash = ctx[rd_idx + hashes_index].clone();

    //Store proof inde in verifier state at channel
    ctx[channel_idx] = Uint256::from_bytes_le( &proof_idx.to_le_bytes() );   

    println!("Calculated Hash: {} \n Expected Hash: {}", hash, root);

    //TODO: Compleltely wrong
    assert!(hash == root); // Possible causes
                            // Fixed - Misinput for test data (Check against Etherscan)
                            // Somewrong logic in verify_merkle
                            // Different hashing function - Not an issue
                            // EVM Keccak reads it differently from how I read it (LE or BE) (Test using remix)
                            // Copied from the queue wrong or reading wrong thing
                            // Logical error previously that cause bad reads from ctx

    return root;

}