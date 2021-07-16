#![allow(arithmetic_overflow)]

use num256::uint256::Uint256 as Uint256;
use crate::uint256_ops::get_uint256;
use num_bigint::BigUint;

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
  ---------------- */
pub fn fmul(a: Uint256, b: Uint256) -> Uint256 {
    //Convert to BigUint and perform multiplication to avoid overflow
    let prod = BigUint::from_bytes_le( &a.to_bytes_le() ) * BigUint::from_bytes_le( &b.to_bytes_le() );
    let val_bytes = prod.modpow( &BigUint::new(vec![1]), &BigUint::from_bytes_le( &get_k_modulus().to_bytes_le() ) ).to_bytes_le(); // (a * b) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

//TODO: allow overflow of Uint256? Do checked prod/div/add/sub of vals, if it returns None, subtract PRIME from total? and repeat
pub fn fadd(a: Uint256, b: Uint256) -> Uint256 {
    //(a + b) mod n = [ (a mod n) + (b mod n) ] mod n
    let add = a + b;
    let val_bytes = add.modpow( &get_uint256("1") , &get_k_modulus() ).to_bytes_le(); // (a + b) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn fsub(a: Uint256, b: Uint256) -> Uint256 {
    let sub = a - b;
    let val_bytes = sub.modpow( &get_uint256("1") , &get_k_modulus() ).to_bytes_le(); // (a - b) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn fpow(val: & Uint256, exp: & Uint256) -> Uint256 {
    let val_bytes = val.modpow( exp, &get_k_modulus() ).to_bytes_le();
    return  Uint256::from_bytes_le(&val_bytes);
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
    let prod = val * get_k_montgomery_r_inv();
    let val_bytes = prod.modpow( &get_uint256("1") , &get_k_modulus() ).to_bytes_le(); // (val * montgomery_inv_r) % K_MOD
    return Uint256::from_bytes_le(&val_bytes);
}

pub fn from_montgomery_bytes(bs: &[u8]) -> Uint256 {
    let val = Uint256::from_bytes_le(bs);
    return from_montgomery(val);
}