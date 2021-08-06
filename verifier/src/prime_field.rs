use num256::uint256::Uint256 as Uint256;
use crate::uint256_ops::get_uint256;
use num_bigint::{BigInt, Sign, BigUint};

/* -------------
    Finite Field Paramters 
 ------------------ */
pub fn get_k_modulus() -> Uint256 {
   return get_uint256("800000000000011000000000000000000000000000000000000000000000001");
}

pub fn get_k_mod_mask() -> Uint256 {
    return get_uint256("0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
}

pub fn get_k_montgomery_r() -> Uint256 {
    return get_uint256("7fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
}

pub fn get_k_montgomery_r_inv() -> Uint256 {
    return get_uint256("40000000000001100000000000012100000000000000000000000000000000");
}

pub fn get_generator_val() -> Uint256 {
    return get_uint256("3");
}

pub fn get_one_val() -> Uint256 {
    return get_uint256("1");
}

pub fn get_gen_1024_val() -> Uint256 {
    return get_uint256("659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1");
}




/* ---------------
    Operations within the finite field
    defined by modulo PRIME, where PRIME is a large prime number
  ---------------- */
pub fn fmul(a: Uint256, b: Uint256) -> Uint256 {
    //Convert to BigUint and perform multiplication to avoid overflow
    let prod = BigUint::from_bytes_le( &a.to_bytes_le() ) * BigUint::from_bytes_le( &b.to_bytes_le() );
    let val_bytes = prod.modpow( &BigUint::new(vec![1]), &BigUint::from_bytes_le( &get_k_modulus().to_bytes_le() ) ).to_bytes_le(); // (a * b) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

// Using BigUint allows overflow of Uint256
pub fn fadd(a: Uint256, b: Uint256) -> Uint256 {
    let add = BigUint::from_bytes_le( &a.to_bytes_le() ) + BigUint::from_bytes_le( &b.to_bytes_le() );
    let val_bytes = add.modpow( &BigUint::new(vec![1]) , &BigUint::from_bytes_le( &get_k_modulus().to_bytes_le() ) ).to_bytes_le(); // (a + b) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn fsub(a: Uint256, b: Uint256) -> Uint256 {
    let res = fadd(
        a.clone(), get_k_modulus() - b.clone()
    );
    return res;
}

pub fn fpow(val_u: & Uint256, exp_u: & Uint256) -> Uint256 {
    let val = BigUint::from_bytes_le( &(*val_u).to_bytes_le() );
    let exp = BigUint::from_bytes_le( &(*exp_u).to_bytes_le() );
    let pow_bytes = val.modpow( &exp, &BigUint::from_bytes_le( &get_k_modulus().to_bytes_le() ) ).to_bytes_le();
    return  Uint256::from_bytes_le(&pow_bytes);
}

pub fn inverse(val: & Uint256) -> Uint256 {
    let base = get_k_modulus() - get_uint256("2");
    let val_bytes = val.modpow( &base, &get_k_modulus() ).to_bytes_le();
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn mod_prime(val: Uint256) -> Uint256 {
    let val_bigint = BigUint::from_bytes_le( &val.to_bytes_le() );
    return Uint256::from_bytes_le( 
        &val_bigint.modpow( 
            &BigUint::new(vec![1]), 
            &BigUint::from_bytes_le( &get_k_modulus().to_bytes_le() ) 
        ).to_bytes_le() 
    ); // val % K_MOD
}

pub fn from_montgomery(val: Uint256) -> Uint256 {
    let prod = fmul( val.clone(), get_k_montgomery_r_inv() );
    let val_bytes = prod.modpow( &get_uint256("1") , &get_k_modulus() ).to_bytes_le(); // (val * montgomery_inv_r) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn from_montgomery_bytes(bs: &[u8]) -> Uint256 {
    let val = Uint256::from_bytes_le(bs);
    return from_montgomery(val);
}


/* --------------
    Unit Testing
------------------ */
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::uint256_ops::get_uint256;

    // #[test]
    // fn test_fsub_underflow() {
    //     let val = fsub( get_uint256("0"), get_uint256("1")  );
    //     assert_eq!(val, get_k_modulus()-get_uint256("1") );

    //     let val2 = fsub( get_k_modulus(), get_k_modulus()+get_uint256("1")  );
    //     assert_eq!(val2, get_uint256("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe0") );
    // }

    #[test]
    fn test_fsub_normal() {
        let val = fsub( get_k_modulus(), get_uint256("10")  );
        assert_eq!(val, get_k_modulus()-get_uint256("10"), );
    }

    #[test]
    fn test_fadd_overflow() {
        let val1 = fadd( get_k_modulus(), get_uint256("1")  );
        assert_eq!(val1, get_uint256("1"), ); //Note: (Prime + 1) % Prime == 1

        let mut val2 = fadd( get_k_modulus(), get_k_modulus()  );
        val2 = fadd( val2, get_uint256("9")  );
        assert_eq!(val2, get_uint256("9"), ); //Note: (Prime + Prime + 9) % Prime == 9
    }

    #[test]
    fn test_fmul_overflow() {
        let val1 = fmul( get_k_modulus(), get_uint256("19230")  );
        assert_eq!( val1, get_uint256("0") );

        let val2 = fmul( get_k_modulus() - get_uint256("1"), get_uint256("2")  );
        assert_eq!( val2, get_k_modulus() - get_uint256("2") );
    }

    #[test]
    fn test_mod_prime() {
        let val1 = get_k_modulus() - get_uint256("1");
        assert_eq!( mod_prime(val1), get_k_modulus() - get_uint256("1") );

        let val2 = get_k_modulus() + get_uint256("90A");
        assert_eq!( mod_prime(val2), get_uint256("90A") );
    }
}