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

//Returns 32 bytes corresponding to big endian of val
// Used in Keccak to emulate behaviour of EVM memory
pub fn to_fixed_bytes(val: & Uint256) -> [u8; 32] {
    let mut fixed_bytes: [u8; 32] = [0; 32];
    let val_bytes = val.to_bytes_be();
    for i in 0..val_bytes.len() {
        fixed_bytes[32 - val_bytes.len() + i] = val_bytes[i];
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
    let mut hasher = Keccak256::new();
    hasher.update(input_data);
    let result = hasher.finalize();
    let result_bytes = result.as_slice();

    return Uint256::from_bytes_be( &result_bytes );
}


/* --------------
    Unit Testing
------------------ */
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_copy() {
        let val = make_copy(&get_uint256("124238942389")); 
        assert_eq!(val , get_uint256("124238942389") );
    }

    #[test]
    fn test_to_usize() { //NOTE: Also tests LE or BE of get_uint256's hex string input
        let val = to_usize(&get_uint256("12345"));
        assert_eq!(val , 74565);
    }

    #[test]
    fn test_from_usize() {
        let val = from_usize(74565);
        assert_eq!(val , get_uint256("12345"));
    }

    #[test]
    fn test_and() {
        let val1 = get_uint256("1434");
        let val2 = get_uint256("124");
        assert_eq!( bitwise_and(&val1, &val2) , get_uint256("24") ); // == 36
    }

    #[test]
    fn test_not() {
        let val = get_uint256("3");
        assert_eq!( bitwise_not(val) , get_uint256("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC") );
    }

    #[test]
    fn test_xor() {
        let val1 = get_uint256("1434");
        let val2 = get_uint256("124");
        assert_eq!( bitwise_xor(&val1, &val2) , get_uint256("1510") ); // == 5392
    }

    //TODO: Finish writing this test and se this code in to_fixed_bytes representation
    #[test]
    fn test_keccak() {
        //Test 1 - Blank 0s
        let input_data: [u8; 64] = [0; 64];
        assert_eq!( keccak_256(&input_data), get_uint256("ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5") ); //hash obtained from remix - ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5

        // Test 2 - Determine to use le or be representation
        let mut combined_data: [u8; 64] = [0; 64];
        let bytes_val1 = get_uint256("12345").to_bytes_be();
        let bytes_val2 = get_uint256("6789A").to_bytes_be();

        for i in 0..bytes_val1.len() {
            combined_data[32 - bytes_val1.len() + i] = bytes_val1[i];
        }
        for i in 0..bytes_val2.len() {
            combined_data[64 - bytes_val2.len() + i] = bytes_val2[i];
        }

        assert_eq!( keccak_256(&combined_data), get_uint256("8c57df5cb87972af4fa804233ccf33fb470363cc9ffbf00055742fa617812c64") ); //hash obtained from remix - 0x8c57df5cb87972af4fa804233ccf33fb470363cc9ffbf00055742fa617812c64
    }

    // TODO: Have to_fixed_bytes return BE representation
    #[test]
    fn test_to_fixed_bytes() {
        let val = get_uint256("12345"); //74565 base 10
        let val_be = to_fixed_bytes(&val);
        assert_eq!( Uint256::from_bytes_be(&val_be), get_uint256("12345") );
    }

}