# Cairo Verifier

A STARK Verifier for Cairo programs, written in Rust. For use in the [Open Libra](https://github.com/OLSF/libra) Blockchain as a Zero-Knowledge Verification mechanism. Cairo Programs serve as a flexible, robust alternative to manually designing polynomial constraints, which are difficult and can be error-prone.

For a more detailed explanation check out my [Medium post](https://medium.com/myBlogPost).

Based off of STARKWare's Soldity Implementation available at: [github/starkware-libs/starkex-contracts/evm-verifier/](https://github.com/starkware-libs/starkex-contracts/tree/master/evm-verifier)

## Use

Include in your project's *cargo.toml*:
```
[dependencies]
cairo-verifier = { git = "https://github.com/patrickbiel01/Cairo_Verifier", branch = "main" }
```


Call:
```
use cairo_verifier;

cairo_verifier::verify_proof(
	proof_params, proof, task_meta_data,  cairo_aux_input, cairo_verifier_id
);
```

*verify_proof* Definition:
```
fn verify_proof(
    proof_params: Vec<Uint256>,
    proof: Vec<Uint256>,
    task_meta_data: Vec<Uint256>,
    cairo_aux_input: Vec<Uint256>,
    cairo_verifier_id: Uint256,
)
```

Note: Uint256 is the [num256::uint256::Uint256 crate](https://docs.rs/num256/0.1.3/i686-pc-windows-msvc/num256/uint256/struct.Uint256.html)

## Goals:
Currently this verifier only supports Cairo Programs that use Layout 1
 - Implement Layout 0, 2 (Example: [layout 0](https://github.com/starkware-libs/starkex-contracts/tree/master/evm-verifier/solidity/contracts/cpu/layout0))
 	* Change values in memory_map.rs, oods_check.rs, polynomial_contrainsts.rs, stark_params.rs
 	* Change all of above to accept multiple layout (Look at 4th element in Cairo Aux Input)

## Resources:

### Papers
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Scalable, transparent, and post-quantum secure computational integrity](https://eprint.iacr.org/2018/046.pdf)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[ethSTARK Documentation](https://eprint.iacr.org/2021/582.pdf)

### STARKWare Articles 
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[StarkDEX Deep Dive: the STARK Core Engine](https://medium.com/starkware/starkdex-deep-dive-the-stark-core-engine-497942d0f0ab)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[STARK Math Series](https://medium.com/starkware/tagged/stark-math)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Using SHARP (Shared Prover)](https://www.cairo-lang.org/docs/sharp.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Cairo for Blockchain Developers](https://www.cairo-lang.org/cairo-for-blockchain-developers/)

### Vitalik Buterin Articles 
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[STARKs, Part I: Proofs with Polynomials](https://vitalik.ca/general/2017/11/09/starks_part_1.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[STARKs, Part II: Thank Goodness It's FRI-day](https://vitalik.ca/general/2017/11/22/starks_part_2.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[STARKs, Part 3: Into the Weeds](https://vitalik.ca/general/2018/07/21/starks_part_3.html)

### Misc. Articles
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[A Hands-On Tutorial for Zero-Knowledge Proofs Series](http://www.shirpeled.com/2018/09/a-hands-on-tutorial-for-zero-knowledge.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Cryptography Stack Exchange Answer](https://crypto.stackexchange.com/questions/56327/what-are-zk-starks)

### Ropsten Addresses
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[0x2886D2A190f00aA324Ac5BF5a5b90217121D5756](https://ropsten.etherscan.io/address/0x2886d2a190f00aa324ac5bf5a5b90217121d5756)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[0xe74999fbc71455462c8143b56797d3bb84c1064b](https://etherscan.io/address/0xe74999fbc71455462c8143b56797d3bb84c1064b)
