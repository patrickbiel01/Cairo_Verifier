use num256::uint256::Uint256 as Uint256;
use sha3::Keccak256;
use sha3::Digest;

// Returns a Uint256 from string containg appropriate hex value
pub fn get_uint256(str: &str) -> Uint256 {
    let mut string_even = String::from(str);
    if str.len() % 2 != 0 { //If length is odd, prepend a 0
        let mut zero_string = String::from("0");
        zero_string.push_str(str);
        string_even = zero_string.clone();
    }

    let val_bytes = hex::decode(string_even).expect("Whoops problem encoding str to hex: ");
    return Uint256::from_bytes_be(&val_bytes);
}


/* Takes lower 8 bits of val in little endian form and return a usize value*/
pub fn to_usize(val: & Uint256) -> usize {
    let mut bytes: [u8; 8] = [0; 8];
    let val_bytes = val.to_bytes_le();
    for i in 0..7 {
        let mut byte = 0;
        if i < val_bytes.len() { byte = val_bytes[i]; }
        bytes[i] = byte;
    }
    return usize::from_le_bytes( bytes );
}

pub fn from_usize(val: usize) -> Uint256 {
    let val_bytes = val.to_le_bytes();
    return Uint256::from_bytes_le( &val_bytes );
}

//Returns 32 bytes corresponding to little endian of val
pub fn to_fixed_bytes(val: & Uint256) -> [u8; 32] {
    let mut fixed_bytes: [u8; 32] = [0; 32];
    let val_bytes = val.to_bytes_be();
    for i in 0..31 {
        let mut byte = 0;
        if i < val_bytes.len() { byte = val_bytes[i]; }
        fixed_bytes[i] =  val_bytes[i];
    }
    return fixed_bytes;
}

// Performs bitwise and between 2 vals
pub fn bitwise_and(val1: & Uint256, val2: & Uint256) -> Uint256 {
    let val1_bytes = val1.to_bytes_le();
    let val2_bytes = val2.to_bytes_le();

    let mut result_bytes: [u8; 32] = [0; 32]; 
    for i in 0..32 {
        let mut val1 = 0;
        let mut val2 = 0;
        if i < val1_bytes.len() { val1 = val1_bytes[i]; }
        if i < val2_bytes.len() { val2 = val2_bytes[i]; }
        result_bytes[i] = val1 & val2;
    }

    return Uint256::from_bytes_le(&result_bytes);
}

// Performs bitwise xor between 2 vals
pub fn bitwise_xor(val1: & Uint256, val2: & Uint256) -> Uint256 {
    let val1_bytes = val1.to_bytes_le();
    let val2_bytes = val2.to_bytes_le();

    let mut result_bytes: [u8; 32] = [0; 32]; 
    for i in 0..32 {
        let mut val1 = 0;
        let mut val2 = 0;
        if i < val1_bytes.len() { val1 = val1_bytes[i]; }
        if i < val2_bytes.len() { val2 = val2_bytes[i]; }
        result_bytes[i] = val1 ^ val2;
    }

    return Uint256::from_bytes_le(&result_bytes);
}


pub fn bitwise_not(val: Uint256) -> Uint256 {
    let val_bytes = val.to_bytes_le();

    let mut result_bytes: [u8; 32] = [0; 32]; 
    for i in 0..32 {
        let mut val = 0;
        if i < val_bytes.len() { val = val_bytes[i]; }
        result_bytes[i] = !val;
    }

    return Uint256::from_bytes_le(&result_bytes);
}


//Returns copy of Uint256 from reference
pub fn make_copy(val: & Uint256) -> Uint256 {
    let val_bytes = val.to_bytes_le();
    return Uint256::from_bytes_le( &val_bytes );
}

//Performs a keckkack256 hash on the input bytes and return a Uint256
pub fn keccak_256(input_data: &[u8]) -> Uint256 {
    //TODO: Probably prepend 0s if less than disred length
    let mut hasher = Keccak256::new();
    hasher.update(input_data);
    let result = hasher.finalize();
    let result_bytes = result.as_slice();

    return Uint256::from_bytes_le( &result_bytes );
}


/* --------------
    Unit Testing
------------------ */
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(3, 3);
    }

}