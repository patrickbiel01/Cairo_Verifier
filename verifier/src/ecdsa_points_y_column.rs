use num256::Uint256;
use crate::uint256_ops;
use crate::prime_field;



/* -----------
 Unit Testing 		
 ------------- */
 #[cfg(test)]
mod tests {
    use super::*;
    use crate::uint256_ops::get_uint256;

    #[test]
    fn test_compute_ecdsa_y_col() {
        let val = get_uint256("29A66415"); //Input: 698770453 base 10 = 29A66415 base 16
		assert_eq!( compute(val), get_uint256("07363f694442d93630659f43cb714a5a317a50998275674476542685ea4f3b6d"));
		// Expected Result: 0x07363f694442d93630659f43cb714a5a317a50998275674476542685ea4f3b6d
    }
}




/* -----------------------
	ECDSA Points Y Column Implementation:
		Use Horner's method to compute f(x).
		The idea is that
			a_0 + a_1 * x + a_2 * x^2 + ... + a_n * x^n =
			(...(((a_n * x) + a_{n-1}) * x + a_{n-2}) * x + ...) + a_0.
		Consequently we need to do deg(f) horner iterations that consist of:
			1. Multiply the last result by x
			2. Add the next coefficient (starting from the highest coefficient)
		
		We slightly diverge from the algorithm above by updating the result only once
		every 7 horner iterations.
		We do this because variable assignment in solidity's functional-style assembly results in
		a swap followed by a pop.
		7 is the highest batch we can do due to the 16 slots limit in evm.
 ------------------------- */
pub fn compute(x: Uint256) -> Uint256 {
	let mut result = uint256_ops::get_uint256("0");

	result = uint256_ops::get_uint256("f524ffcb160c3dfcc72d40b12754e2dc26433a37b8207934f489a203628137") + prime_field::fmul(
			uint256_ops::get_uint256("23b940cd5c4f2e13c6df782f88cce6294315a1b406fda6137ed4a330bd80e37") +  prime_field::fmul(
				uint256_ops::get_uint256("62e62fafc55013ee6450e33e81f6ba8524e37558ea7df7c06785f3784a3d9a8") + prime_field::fmul(
					uint256_ops::get_uint256("347dfb13aea22cacbef33972ad3017a5a9bab04c296295d5d372bad5e076a80") + prime_field::fmul(
						uint256_ops::get_uint256("6c930134c99ac7200d41939eb29fb4f4e380b3f2a11437dd01d12fd9ebe8909") + prime_field::fmul(
							uint256_ops::get_uint256("49d16d6e3720b63f7d1e74ed7fd8ea759132735c094c112c0e9dd8cc4653820") + prime_field::fmul(
								uint256_ops::get_uint256("23a2994e807cd40717d68f37e1d765f4354a81b12374c82f481f09f9faff31a") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("4eac8ffa98cdea2259f5c8ad87a797b29c9dccc28996aed0b545c075c17ebe1") + prime_field::fmul(
			uint256_ops::get_uint256("1058ff85f121d7902521abfa5f3f5c953fee83e0f58e069545f2fc0f4eda1ba") +  prime_field::fmul(
				uint256_ops::get_uint256("76b4883fd523dff46e4e330a3dd140c3eded71524a67a56a75bd51d01d6b6ca") + prime_field::fmul(
					uint256_ops::get_uint256("5057b804cff6566354ca744df3686abec58eda846cafdc361a7757f58bd336e") + prime_field::fmul(
						uint256_ops::get_uint256("37d720cf4c846de254d76df8b6f92e93b839ee34bf528d059c3112d87080a38") + prime_field::fmul(
							uint256_ops::get_uint256("a401d8071183f0c7b4801d57de9ba6cda7bd67d7941b4507eab5a851a51b09") + prime_field::fmul(
								uint256_ops::get_uint256("603e3a8698c5c3a0b0b40a79ba0fdff25e5971f0ef0d3242ead1d1a413e443b") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("4b74b468c4ef808ddcc6e582393940111941abece8a285da201171dc50525c7") + prime_field::fmul(
			uint256_ops::get_uint256("761717d47600662a250116e2403b5115f4071de6e26e8dc231840eeb4484ec3") +  prime_field::fmul(
				uint256_ops::get_uint256("5a593d928542a100c16f3dc5344734c9ef474609bd7099257675cef0392fab8") + prime_field::fmul(
					uint256_ops::get_uint256("7d2292c8660492e8a1ce3db5c80b743d60cdaac7f438b6feab02f8e2aade260") + prime_field::fmul(
						uint256_ops::get_uint256("480d06bb4222e222e39ab600b8aadf591db4c70bae30fe756b61564eec6c7e") + prime_field::fmul(
							uint256_ops::get_uint256("59fef071cf1eeff5303f28f4fe10b16471a2230766915d70b525d62871f6bc6") + prime_field::fmul(
								uint256_ops::get_uint256("6e7240c4a94fa3e10de72070fd2bf611af5429b7e83d53cfe1a758dee7d2a79") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("247573f2f3fbd5386eac2d26851f9512cd57ad19773b8ca119d20852b9b6538") + prime_field::fmul(
			uint256_ops::get_uint256("739edb8cdd16692deaba7fb1bb03f55dd417891bacb39c7927969551f29cb37") +  prime_field::fmul(
				uint256_ops::get_uint256("6e0bed1b41ee1cf8667c2924ebd460772a0cd97d68eaea63c6fa77bf73f9a9e") + prime_field::fmul(
					uint256_ops::get_uint256("3ede75d46d49ceb580d53f8f0553a2e370138eb76ac5e734b39a55b958c847d") + prime_field::fmul(
						uint256_ops::get_uint256("59bd7fe1c9553495b493f875799d79fc86d0c26e794cce09c659c397c5c4778") + prime_field::fmul(
							uint256_ops::get_uint256("47b2a5ef58d331c30cfcd098ee011aaeae87781fd8ce2d7427c6b859229c523") + prime_field::fmul(
								uint256_ops::get_uint256("14ef999212f88ca277747cc57dca607a1e7049232becedf47e98aca47c1d3fe") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("38db61aa2a2b03053f5c51b155bc757b0634ce89baace113391369682fc1f74") + prime_field::fmul(
			uint256_ops::get_uint256("43545892bb5a364c0b9acd28e36371bede7fd05e59a9dcd875c44ff68275b2b") +  prime_field::fmul(
				uint256_ops::get_uint256("5599e790bd325b322395d63d96cd0bd1494d4648e3d1991d54c23d24a714342") + prime_field::fmul(
					uint256_ops::get_uint256("675532b80f5aaa605219de7fe8650e24fee1c3b0d36cdf4fb605f6215afacee") + prime_field::fmul(
						uint256_ops::get_uint256("278a7c68986adbe634d44c882a1242147e276fee7962d4c69ca4c8747b3e497") + prime_field::fmul(
							uint256_ops::get_uint256("75a0f99a4dec1988f19db3f8b29eeef87836eb0c3d8493913b7502cfedcef28") + prime_field::fmul(
								uint256_ops::get_uint256("2f6efb89f27d2c0a86ec1e6f231b225caf2af9be01aca173a15fa02b11fdf24") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("10f236430f20aafda49d1c3e3759c510fdf0c0c19f89df6d5d71deac88b547b") + prime_field::fmul(
			uint256_ops::get_uint256("7b16c33c4a8ffcecbd83f382469e1d00a340ceab5e7d9c0bd4fd010b83f4310") +  prime_field::fmul(
				uint256_ops::get_uint256("6ae3ee97ea5dcfbb7c36cffd89665baf114fae391c0367be688db09861a8ca1") + prime_field::fmul(
					uint256_ops::get_uint256("cb3335374cc2a2350fe53d2389f04952c4d634f489031742dfccca17be2e09") + prime_field::fmul(
						uint256_ops::get_uint256("1030d58878296e14b1c5bcafe7e817ebe4aa1039aa96b9d0dd7fc915b23f42a") + prime_field::fmul(
							uint256_ops::get_uint256("3a663fc27ec3ad56da89d407089bcec0971cebcb3edf0c393112501919643d7") + prime_field::fmul(
								uint256_ops::get_uint256("71b2b6b03e8cc0365ac26c4dbf71e8d426167d79f8bd1af44738890c563062a") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);



	result = uint256_ops::get_uint256("4f63db02e10fbe428a5dda8d9093feef46cc19568a3c8ad2fce7e7519004095") + prime_field::fmul(
			uint256_ops::get_uint256("2bfd1294f111a5a90842d19cffb97481aefbc09ab6c47d7dcf91ba228019c07") +  prime_field::fmul(
				uint256_ops::get_uint256("daee1c7b34ecb34717b7313dc4a299dd1a161447e2e0249426a6fc33a72289") + prime_field::fmul(
					uint256_ops::get_uint256("76323f8567119897f10d58e1552c98f5a62f03a16d3737e20fc2b0a31a3a843") + prime_field::fmul(
						uint256_ops::get_uint256("65d50aa3c1d84a3deee14057eec98656a1296cdcbe32250bfdaa50ffac4c5dc") + prime_field::fmul(
							uint256_ops::get_uint256("253bf2869135f4bda4029cae2819b2f468ae88530f3ea771090b2727814c494") + prime_field::fmul(
								uint256_ops::get_uint256("104b04e96151f5103118c4eb556cd79899148fd6656e73cb62f41b41d65e4d8") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);




	result = uint256_ops::get_uint256("4e0a5dd802deed7cb8d06527beb15dad32547bae77141c32473f4c8148912e3") + prime_field::fmul(
			uint256_ops::get_uint256("33ff2d848bf237f536524da818598ae0f2516ebee526b77957448973eefacd3") +  prime_field::fmul(
				uint256_ops::get_uint256("5a00feeb391114d7b976654ab16ddf8360f05671b34d4a97da278c0aef34d76") + prime_field::fmul(
					uint256_ops::get_uint256("7e8659c39d7a102a198f0e7c3814060926ec0410330dd1a13dfadeab4e74593") + prime_field::fmul(
						uint256_ops::get_uint256("5ba89e0eb3830039d0f8a9ca00acef15db22374c965b01abc49dee46270a7d") + prime_field::fmul(
							uint256_ops::get_uint256("30a2e8ac9e6605fd722dffb4caca8c06dd4a8968a7bf41a5371cb1a07d11c00") + prime_field::fmul(
								uint256_ops::get_uint256("761a240cd8aa2f135daf0760bfc2c9d5e896e93a45426571cdad9118722e2b0") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);




	result = uint256_ops::get_uint256("1b0fa36439192f135c239918bf47ad14b55ced699f4582d929a60dd227b34ff") + prime_field::fmul(
			uint256_ops::get_uint256("472d99d1a6e1a6aef339eab1af3d53af7a8326e4d0a6bac73c3a159031c3686") +  prime_field::fmul(
				uint256_ops::get_uint256("2046e1b4fd4c108e8f832f5bcc4dd46abf0d19ef0237beaec29d6c12fb9832e") + prime_field::fmul(
					uint256_ops::get_uint256("a758a70ba6a0cbcbc65abfeca51359904f790752c3df55d42707253d8dea70") + prime_field::fmul(
						uint256_ops::get_uint256("6eb66d366da57e4ae717307dfc3351579fe857c51aa82b95044473c9ed14377") + prime_field::fmul(
							uint256_ops::get_uint256("59d0d8ca9ecda81081dfcae7580ab3c08a72195438c1556000c0c1dbdc08174") + prime_field::fmul(
								uint256_ops::get_uint256("776459dfedbbdfcef7a31e0f60c6480fc0676b280fdb6290859fe586d6e6106") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);



	result = uint256_ops::get_uint256("23590dabe53e4ef12cba4a89b4741fcfaa232b7713d89df162031c8a627011e") + prime_field::fmul(
			uint256_ops::get_uint256("339b405bffb6dbb25bc0432e9c726b7f94e18cf1332ec7adfeb613345e935ab") +  prime_field::fmul(
				uint256_ops::get_uint256("25c5f348c260177cd57b483694290574a936a4d585ea7cf55d114a8005b17d0") + prime_field::fmul(
					uint256_ops::get_uint256("68a8c6f86a8c1ebaeb6aa72acef7fb5357b40700af043ce66d3dccee116510a") + prime_field::fmul(
						uint256_ops::get_uint256("1ea9bd78c80641dbf20eddd35786028691180ddcf8df7c87552dee1525368ba") + prime_field::fmul(
							uint256_ops::get_uint256("4e42531395d8b35bf28ccc6fab19ea1f63c635e5a3683ac9147306c1640e887") + prime_field::fmul(
								uint256_ops::get_uint256("728dd423dbf134972cbc7c934407424743843dd438e0f229afbcca6ce34d07d") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("30b11c32e8aab0c5908651a8d445395de52d5ce6a1efe75f2ad5e2c8c854a30") + prime_field::fmul(
			uint256_ops::get_uint256("44938959c2e944eb6e5c52fc4ee40b34df37905fa348fa109f6875c1aa18000") +  prime_field::fmul(
				uint256_ops::get_uint256("655038ca08eba87484bc562e7fd50ce0584363278f9d716e31c650ee6989a2b") + prime_field::fmul(
					uint256_ops::get_uint256("4f81a946bb92416d212e4d54f2be5fa8043be6fa482b417d772bfa90be4e273") + prime_field::fmul(
						uint256_ops::get_uint256("605a244f646a825602891bf9ddffef80525010517b32625759b0bf5a7f2c386") + prime_field::fmul(
							uint256_ops::get_uint256("2e1b2a3c32aebc0be30addd8929c01714783aaf01be8a1d35e830646e8a54f0") + prime_field::fmul(
								uint256_ops::get_uint256("534a4f3cf71c93023e473f12e407558b6c24b712204fd59ddc18c7bcddd571e") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("3e850e31c0345726c1ace38537dd88a50c85d6819ae98add1bbd62b618f7a1c") + prime_field::fmul(
			uint256_ops::get_uint256("d77a8e8eed7ce4931a6d2a4774c21864e2c9f468d080af9aba6756433a1a8d") +  prime_field::fmul(
				uint256_ops::get_uint256("62be425458d26cfedf8ec23961cdfd9f4abeb21f1debbe87bd51469013358fe") + prime_field::fmul(
					uint256_ops::get_uint256("7d7faca17be1da74cf132dda889a05fce6e710af72897a941625ea07caa8b01") + prime_field::fmul(
						uint256_ops::get_uint256("580550e76557c8ff3368e6578a0e3bed0bac53b88fefdde88f00d7089bc175d") + prime_field::fmul(
							uint256_ops::get_uint256("1345876a6ab567477c15bf37cc95b4ec39ac287887b4407593203d76f853334") + prime_field::fmul(
								uint256_ops::get_uint256("4a92733a733f225226a3d7f69297e7ff378b62c8a369e1bbf0accfd7fb0977e") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("2833391a62030808228d14437d6f91b31c0038c14988a23742b45e16f9b84b5") + prime_field::fmul(
			uint256_ops::get_uint256("a737d6916aa6a869252d8ff294a55706e95e0844e6b047755704e37d978e09") +  prime_field::fmul(
				uint256_ops::get_uint256("2652523cbbec2f84fae1a17397dac1965127650479e1d5ccfc6bfbfcbb67996") + prime_field::fmul(
					uint256_ops::get_uint256("6dcfc3a99563a5ba4368ac4f11f43e830c5b620a7273330e841bedec0bfb5a") + prime_field::fmul(
						uint256_ops::get_uint256("5428ff423f2bbabcb5f54aafa03d99a320b4b255115351f50b229eae5522178") + prime_field::fmul(
							uint256_ops::get_uint256("76640613af9ed1a125624e0c38252bee457ce87badb24fc4f961e55883d9077") + prime_field::fmul(
								uint256_ops::get_uint256("375a5d9b11c83d06a04dc9f1908b8183adc6f04e5b2ceeaa23d3b68c973ee77") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("327319fcc0d34a0d64f5acab00244b43674a60bef754844fb2920c87c90cff0") + prime_field::fmul(
			uint256_ops::get_uint256("573b13b32161c11c9b16eff7cf93fa770a3ef667547a27503e39092aeabf73e") +  prime_field::fmul(
				uint256_ops::get_uint256("41776c662b44a36c7075097c14b6010cb321591a4eca2866d58252eaf9471ac") + prime_field::fmul(
					uint256_ops::get_uint256("7f2abefac9e7f8109b0a2d25d0bd297059e45dd66798ac8b299f0a3e442dd2c") + prime_field::fmul(
						uint256_ops::get_uint256("60bdb98c079bd5cef216803b056afce03f6ea41934275c965d6e196240fb953") + prime_field::fmul(
							uint256_ops::get_uint256("1e141c5429a369996563573bf61d7f713cb7d25baadff636ba2756c65a910ee") + prime_field::fmul(
								uint256_ops::get_uint256("284f7815a7eabc1dcf56da511f7d739f1a199f8ffaf3474f645d2fc93327dc") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("70930735d913d54915fba20c97f07cba8f33eb8f4f81fd869699a10e83264cd") + prime_field::fmul(
			uint256_ops::get_uint256("1e3b6498f0daba2fd99c2ac65461c3fa519cb738b53cd6f002e97199fa4161c") +  prime_field::fmul(
				uint256_ops::get_uint256("3d8506e792fa9ac86ac9739d3d5bf63cfc13c456a99c8581adf590c8d9b72eb") + prime_field::fmul(
					uint256_ops::get_uint256("5e4b0ecc6a6c15ed16c1c04e96538880785ff9b5bff350f37e83b6fed446f14") + prime_field::fmul(
						uint256_ops::get_uint256("21f5ea8660d290f28b9300e02ed84e110d7338a74503b369ad144a11cf79f63") + prime_field::fmul(
							uint256_ops::get_uint256("7b9cd3b277f00a75a17961d2d8e46e6a1838c8500c569cdcad08bd4e0cbae84") + prime_field::fmul(
								uint256_ops::get_uint256("755f0e4c374e2fa4aa7eda10041e2139a4a7793eea44f415c73ad4fcba1758") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("3678de28b6896959edf5c9dc0caec59b02dfbbf54811f87939b32d0523f58bb") + prime_field::fmul(
			uint256_ops::get_uint256("5820792f23a13d58ddef0607950d422598bb1f21888dace88929fbe7d4828c4") +  prime_field::fmul(
				uint256_ops::get_uint256("26a4b2a61f40c1ad77737b99cb27d2f3118622be64f0120907e2589d2f25ebf") + prime_field::fmul(
					uint256_ops::get_uint256("4b2222d0aee638c7e5efd8ada791638ac155a01b78f3b532283574653998bb2") + prime_field::fmul(
						uint256_ops::get_uint256("5db8c52b6adb520496f9edd7105c92df67e8605ff4e0cc59992c3eb651ac7a4") + prime_field::fmul(
							uint256_ops::get_uint256("3aa748723229eb8b33354e0901f50ad052b6c1006916790c979133c4442be90") + prime_field::fmul(
								uint256_ops::get_uint256("16a36769ee50227c564bebce3d9cd7c4ca55702a7c7ccf403075f68f05a0c2") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("171f0638dedf0b69655fa9930bcbc91b257e299a6717bd8ea23ef550c8faff5") + prime_field::fmul(
			uint256_ops::get_uint256("29889daac66c404d6491ec3a435d810a2877d885df1a3a193697b79b4af39c4") +  prime_field::fmul(
				uint256_ops::get_uint256("229d7fc2a1bcfbe00d5773f8dadd70a2641d8578fa73e66263b3512d3e40491") + prime_field::fmul(
					uint256_ops::get_uint256("73200d12e733294b5cbb8ffe7fb3977088135d0b0e335135f9076d04a653c58") + prime_field::fmul(
						uint256_ops::get_uint256("6d7af6524127a117184a0c12a6ff30d28b14933a4e96bb3b738d2a36db72e84") + prime_field::fmul(
							uint256_ops::get_uint256("7af8995e2ceed8841e34d44365c7ca14f5980a6a5c67b9813fa7bfd74a9c1b1") + prime_field::fmul(
								uint256_ops::get_uint256("3cd13f84bb7ae6eeccc1012837d2f3e017f069e66cf047172bc70371f5aed38") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("658160ea7b654d786dc624b258c691f594e080610c2d41d6ebea0d8e3396849") + prime_field::fmul(
			uint256_ops::get_uint256("56cbe248ebbc2f57ca8b943b219ba245791592f687815293a4499ef598fa9b7") +  prime_field::fmul(
				uint256_ops::get_uint256("2a48058c77edcd75dd4323d9bb9eccb854009b1184fd716a8202f8627bb5447") + prime_field::fmul(
					uint256_ops::get_uint256("3444c0f008988c8f600270b365ff926f016e49a54ab35bac4f3b3a42a5879b1") + prime_field::fmul(
						uint256_ops::get_uint256("6d1c3edcf1de16a4e0ad7d8aa099a31fa2cfbf81f6d1a5798bd1ef93ff906af") + prime_field::fmul(
							uint256_ops::get_uint256("7fc7d854c9d0b3bfbf826c384b3521af0f29f975613e8ea6dc14f37d8beb54c") + prime_field::fmul(
								uint256_ops::get_uint256("ded0f75cd0a6a5401a954d26880eaf12050ce6458d3254c9dd6354bf66278") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("54ab13ae1984dcc7d38c867a47f4a8cf786079ee07cc94ab5ec1962c21f638b") + prime_field::fmul(
			uint256_ops::get_uint256("688c61ee887c1497ffcef82163f1a81bf7778f2c314ffbd325627bf0b25dc5a") +  prime_field::fmul(
				uint256_ops::get_uint256("657060a10db73c4a9b6aa6288dd6164e0b50a4e6efbc2ee599a0cf4fda33b81") + prime_field::fmul(
					uint256_ops::get_uint256("4c05a7abaaf08f21d93b2257d4f4a3ab2b44f4ac44ce0444418c864ca18470b") + prime_field::fmul(
						uint256_ops::get_uint256("19637a12aa8b822c4a3f3551ef6c538043371a12a962de1dc25d67e0a5ee561") + prime_field::fmul(
							uint256_ops::get_uint256("7b74edd15d97b289da4040272cfc573f69a8c9a8b36d05e3e50b598508b7f9d") + prime_field::fmul(
								uint256_ops::get_uint256("6fcc261ded0ba97b4defc7c9bcd32b5dac89e4c08cb55cef98c6b50f5a3a289") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("601a139ed75acbecf557cd6513171385a119087585111c30bbc1b65cd6d30d") + prime_field::fmul(
			uint256_ops::get_uint256("199d80ad30b4b330fc8a063d1e87307993e1d98822a1729488ba8a586045691") +  prime_field::fmul(
				uint256_ops::get_uint256("17ab90241b58bd3bd90b8a5c7f30aa9e5afeedbe1c31f21ca86c46c497b573c") + prime_field::fmul(
					uint256_ops::get_uint256("7d92a463e2aec09eb86f4647dc9ec241904135b5eb53ea272e809e58c0a271e") + prime_field::fmul(
						uint256_ops::get_uint256("51d6322f7d582892421e977464b49c4e6e64af2438da9a7f21a061c77712dc") + prime_field::fmul(
							uint256_ops::get_uint256("610bf9b7ea4557d72411ec90fb677f9a2ccb84c76f003954da4e7f439c9a84c") + prime_field::fmul(
								uint256_ops::get_uint256("ccee381472bb7dcae008316038c87a44fd9295f730e389eff14e86442c41b8") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("79fd6f5f9b042ece36af6b10eae2eef9de9c9dd18752eb66868a0c301015dd9") + prime_field::fmul(
			uint256_ops::get_uint256("f1f93c3d919653f02fba06fcba1ab89497fff53eceff6a7d129887d5a9e3b") +  prime_field::fmul(
				uint256_ops::get_uint256("43f51dfe0f1cf290c9a522e2a5e734f79d220be80348438c676295c3d429e") + prime_field::fmul(
					uint256_ops::get_uint256("27e76848780aba5b12061bffefff1710995586618a2f32792d62771d31ed519") + prime_field::fmul(
						uint256_ops::get_uint256("7e176a66dcfd58e240c4546cd760b7e5ad02e4f0265c6a2f38d710bbdf99d55") + prime_field::fmul(
							uint256_ops::get_uint256("2a17a5c34f9f598deb5bec334fde606eaa5601df908eb5825ecf70f9cecec3f") + prime_field::fmul(
								uint256_ops::get_uint256("77b10e23b08892ab18cc6b14dfda6f4be5c2fec94a12e3622622376edd0d6a8") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);



	result = uint256_ops::get_uint256("78aafbe80fa5ee9a846e991bf35b81567a6dcbb1b190e7ee47e53fc66422e84") + prime_field::fmul(
			uint256_ops::get_uint256("69d95f3c7892a1cf65b45c324be2294c4c5459e05e0feaa0b8bb98cd8bc958f") +  prime_field::fmul(
				uint256_ops::get_uint256("201019c76d9aa29a00e6b18a4eeac7b1322b44285c57cf4c0b68a87120b1d31") + prime_field::fmul(
					uint256_ops::get_uint256("7238f034b8c57c8b59b0f744ababf9da8229152a051d4f3b3c4995233ac1111") + prime_field::fmul(
						uint256_ops::get_uint256("219557f1604be8622e697e986c03d2a49e40cce558a264bf4f1ebe06493eceb") + prime_field::fmul(
							uint256_ops::get_uint256("329230075f64ffbf631eb0c40b97d71b4dc38a08bd18b638f57e5644680068c") + prime_field::fmul(
								uint256_ops::get_uint256("1958435eb08883bd69b6a56a8f3103c22f8ae206a3d4deaf4a04118b4dd6a6c") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("b8dd33ef8726747fb368aedf80c2f4a720bc1b5220f4a3f0e56e2fafb7e243") + prime_field::fmul(
			uint256_ops::get_uint256("6eba866251e1dca38a21c8b3fad0aa3c22a45dd89884c4c68bd7ef67de64f52") +  prime_field::fmul(
				uint256_ops::get_uint256("90b2b18b3fc2919a55b71ad6d6fa67dda752bd02c985b59e6554f557fe4a2e") + prime_field::fmul(
					uint256_ops::get_uint256("2f47cde744314dc0502faffb0387a2e765e4354b0516ee9ab0b97a1b6c33ec2") + prime_field::fmul(
						uint256_ops::get_uint256("4adaabee9ab3c6ee7fc67a2ddc09c5185755dcc76cc3b814a6b71aa7ae542ea") + prime_field::fmul(
							uint256_ops::get_uint256("1a4bdaf2bff969eff8cef73e762b6346492b8d0f17b2e42956c526f625241ea") + prime_field::fmul(
								uint256_ops::get_uint256("15ba3c5a882d4dfe3e23db18368ade6b2d10ef52e34f12ce0d62e7183c10f7e") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("38e5702bb10256e1856a5bfb03a06b231b89a36e2f84af80bcd2d027153d847") + prime_field::fmul(
			uint256_ops::get_uint256("7f71cb5526600d15d3413ec971ee3b133718224b3cbdc68171a53d7c8684382") +  prime_field::fmul(
				uint256_ops::get_uint256("64d672ca00300ddd5e9c9d2db433d7623bb54c8eb2db51b235a07616f1517e5") + prime_field::fmul(
					uint256_ops::get_uint256("84add7269e2e41ea57aaed996f4c012ba7003ea2b994670cc0d554b7a8bd2a") + prime_field::fmul(
						uint256_ops::get_uint256("28b38e0334fc06af4c94ec4f9434923d4149cc51817526597423fd4692c59ad") + prime_field::fmul(
							uint256_ops::get_uint256("6d28879c6f75c4ede18e1b94ffff964d08c79038fd9ba2e7873cbefb5f323db") + prime_field::fmul(
								uint256_ops::get_uint256("1fac2f441d05a3b483675200cb1ebc6f4ca6ecc5ae60118fe8745f95217bf8b") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("45b4e74f19b293bc3d3d172a101e344558fcf4ccfe5eecefe31f45a45614df7") + prime_field::fmul(
			uint256_ops::get_uint256("e505592d606917f898c54a7afc45b328be3cd48121aee2e8f05185a3e23e5f") +  prime_field::fmul(
				uint256_ops::get_uint256("2a427d70a34b6b5237894f065ef5d60a9872ba444d47d98648b080b8ddb2a68") + prime_field::fmul(
					uint256_ops::get_uint256("40a9cea0394d15ef057c2923d4185f290fe2347e00529d92f927ef506e3b5e7") + prime_field::fmul(
						uint256_ops::get_uint256("31a77aa370bb597dbdd0422612a7dd947aae09a5b0b17d1996f13a85103d150") + prime_field::fmul(
							uint256_ops::get_uint256("68384718bd3bb23f32999f1edcb2dbddd8136259e676c4492d0cafe80ffd856") + prime_field::fmul(
								uint256_ops::get_uint256("1a8d4b2044b8e03b325c353f3f92283013920b92f479064b6e93159d2ed3ba0") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("3238aeb8f6bea8bcaaa1bdd5b4f917ccfad8eab031785ccdc648b47d7ea4be8") + prime_field::fmul(
			uint256_ops::get_uint256("399c00b8ebb398248bb1f52528d5241e7366b73c2d89f57a11dc82c530cc57c") +  prime_field::fmul(
				uint256_ops::get_uint256("68c5830832f6270a189b074d7675fcbc1d1c5cc06ce9c478bf8f4d5ac1bf40") + prime_field::fmul(
					uint256_ops::get_uint256("4387edee6899d4a85883d2f8524978a4634ff82779f150b7b0c861bb315ed3f") + prime_field::fmul(
						uint256_ops::get_uint256("3159144c85f2c515eb806e5aedd908553057b69c556d226adc6e4511a35423c") + prime_field::fmul(
							uint256_ops::get_uint256("2868a08eae382c069047152ee964ac5ebd242b44267e97e578802440ef764f5") + prime_field::fmul(
								uint256_ops::get_uint256("68486394265c9dc8fae42c8fd39605d3179c981cb44cbe33740a3deb907bc59") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("47d21828025d0cbab84084965a49dd14c7833aac562b55de808a94777df2ea3") + prime_field::fmul(
			uint256_ops::get_uint256("50c92b3e6848a21001be2a268615e1e26cb4918ecb09640efaaf1d8b71568fb") +  prime_field::fmul(
				uint256_ops::get_uint256("3c4ad04a5a057e4411487858dbe16af8e3fc065ef7400749ffdc248bdb25bc5") + prime_field::fmul(
					uint256_ops::get_uint256("3924324af1994280f87f289fdae0b9a2d8cb9914ec37d319c18daf029211815") + prime_field::fmul(
						uint256_ops::get_uint256("1cb6e2fba23730f5bf9d8e726569b6e8bf6b5ffe8520339503c5469cc3713a2") + prime_field::fmul(
							uint256_ops::get_uint256("360274f27df6eeec0b7b65fbb227a8214ac3e55cb37b1970e18489ef5b574e1") + prime_field::fmul(
								uint256_ops::get_uint256("357bf5d87c973292381fa4320114551a837a1d6cb6e2bb0eeba534fb2e01742") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("77dee5f03389585fad0d1f2a8accfa4cb985344891b8befaee42f3462cb48a") + prime_field::fmul(
			uint256_ops::get_uint256("5ac4bcdb9c14634ab83c13a30822ddbabc54248cf1177b11cc2aed24d2d32f5") +  prime_field::fmul(
				uint256_ops::get_uint256("5dd2e0680c7eff25211f31d3c30a9f454500d6eb09d46d87a75a42b190203cb") + prime_field::fmul(
					uint256_ops::get_uint256("22aa8c5c5ff26f9a0edc768ae32ff4f71a71205b4e83cfa0cc687a1e02566ba") + prime_field::fmul(
						uint256_ops::get_uint256("78f49c214872b5cce18ead0207a165fb741ea818a69cfe9647737323f70f4f5") + prime_field::fmul(
							uint256_ops::get_uint256("2d4acebd804035257147ad8d8419a5f5762b4b543c4846ef9acf41856e672ee") + prime_field::fmul(
								uint256_ops::get_uint256("6207c6a2fd70c19a10430566c9efaad95eab8cbddf308f0057c81f3155a25a0") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("264a535ae10091157ed59b04955dff66897af74cae20456bb830336b803ae47") + prime_field::fmul(
			uint256_ops::get_uint256("160abeb38bc4f22af5fe618c19c77c39903007900722bdbdeaee059f31544c8") +  prime_field::fmul(
				uint256_ops::get_uint256("4846d310812d81ffda3731e8289005e2f0e05411e76b1c84332c3ee9e831afb") + prime_field::fmul(
					uint256_ops::get_uint256("2e14e83be58cde3ed5f3fec8ba6462493a4a2f0f7d6c846006220eccd49ef25") + prime_field::fmul(
						uint256_ops::get_uint256("73724274fdd351c378e597da1615dc51058e14994464cb7b318766199ac2a35") + prime_field::fmul(
							uint256_ops::get_uint256("23bf372b0b59abf250463697ef4b2096eb1c9674613918b4d0c79aa10d9fd59") + prime_field::fmul(
								uint256_ops::get_uint256("737dba18eb055a12d842bfae32fd146dcd2d7bb932a2591aa864458d6d652") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("7616cfc6834643d4b95ed1cfec036f816a7c3d3b9800f301f98ddf341712ebf") + prime_field::fmul(
			uint256_ops::get_uint256("318e5a52d685eaa06e0f39159a344b3d97b52688b671d133954aeff0bc17707") +  prime_field::fmul(
				uint256_ops::get_uint256("7ff76956e0cd2b490b47a0a0497df5f874cf47f54c45f08101256429b48460") + prime_field::fmul(
					uint256_ops::get_uint256("181ef9cde124459dc0e2aaf93512abd49a10328fb93dfc4d49ab671db64bbc4") + prime_field::fmul(
						uint256_ops::get_uint256("2353c4a418bdc1e461be162140cc69c26eb9d99f08924991f85058f87f6df41") + prime_field::fmul(
							uint256_ops::get_uint256("775d95a0beb287c98663a3f9a9c577ffc67c1fe6fbe2db5b08829a2c3eac922") + prime_field::fmul(
								uint256_ops::get_uint256("316ce6b23e720b8302e2d4bd968c0f140f69930e46a54784a7cee7e0b8a0c8") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("4ce0a14a5a9c30a38062eb8870eeb4ff3562db743c0f3eede2e3d3862a2eb7c") + prime_field::fmul(
			uint256_ops::get_uint256("47f02fc512b153462379f4f793c7cab9e659bfdb07d3439d29039f566b7236d") +  prime_field::fmul(
				uint256_ops::get_uint256("6f617dce150ea148cb8c7488fe4caa920b2000bc8122cce1891e4b76cddc9d4") + prime_field::fmul(
					uint256_ops::get_uint256("685af2d7bbf30cd0c5c3d41c430a8657eeafeeb4596165faaa73d802087ad80") + prime_field::fmul(
						uint256_ops::get_uint256("4fb0c93fe30da048576fe5e839483636218dfdda3d05f1d68847a4c0167597f") + prime_field::fmul(
							uint256_ops::get_uint256("b806f4e19770279fab5427b8eaf5bc68bf984d6ccea1e878a7aaf32c9975d9") + prime_field::fmul(
								uint256_ops::get_uint256("59869515fb57ea7733567e5d849bcaa00c00e0f86f4ebbd2c7a6f4c0c77692b") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);


	result = uint256_ops::get_uint256("175a904681c7a91856bf7fcf8410d2c19eb8705267914489664a1ea2af5b8fe") + prime_field::fmul(
			uint256_ops::get_uint256("c61c74cc988663ee09f4c725d5b1f04549bd342d3550ce17427ac75592b637") +  prime_field::fmul(
				uint256_ops::get_uint256("206d7f23d0fe1b1c0967486ebb792d7fdf5b1691d2c2f9306e211d3b849526b") + prime_field::fmul(
					uint256_ops::get_uint256("4255a568f4597862e1dfe0c391b97059d179d7eb4d868f61364835e5028f9dd") + prime_field::fmul(
						uint256_ops::get_uint256("5fcfeb78685abb1ce610e516ab7e2aa210fd90844c8d1c89cd798f3d71bbcb3") + prime_field::fmul(
							uint256_ops::get_uint256("50f5f6adbf0b9abc6e231b855018f4ec806a4f199cc511bed5c423ebef298e4") + prime_field::fmul(
								uint256_ops::get_uint256("7b077d27c7007656025224fa4e528b4c4261f43c3da1e42bd1349403af55cbb") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("30632b3865a272a1a00270430744ee90b40ff16e1fc44515876ce8e36215ca0") + prime_field::fmul(
			uint256_ops::get_uint256("728771890334d0c9b0f400543bdc13ea6890497bc87c509a04f8014916c13a5") +  prime_field::fmul(
				uint256_ops::get_uint256("72c0dd24a576b47a84cdd1a20227773b5621f85b781c288625e3368e1cf738a") + prime_field::fmul(
					uint256_ops::get_uint256("6dff267c3bbce68474294da908df4f5cf2a4160c638f7cb45c098057e968f44") + prime_field::fmul(
						uint256_ops::get_uint256("842955243a56778a332ba9be0b22b2af62efaa50068d3078675fb76c225e76") + prime_field::fmul(
							uint256_ops::get_uint256("14899e0f97aac917d46ce5e9ddf11194fb846d2c52726af4085f27c570a98a9") + prime_field::fmul(
								uint256_ops::get_uint256("1bd842a4ec97e1489ceb542bd3161e5a00ce431547bfadfbced954d993b0a11") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("4e23809ce49747990e43b2d976083dc84d67e75cf22e5a76ad5b7a2dca50b3d") + prime_field::fmul(
			uint256_ops::get_uint256("40f019a18b8097235264cb8efee7d149321a199ccd32ffac43b5a778dfadda1") +  prime_field::fmul(
				uint256_ops::get_uint256("1495d40cf3f13c5fc90653c2b2f02e0b833790c07576286d3127f745ea920ae") + prime_field::fmul(
					uint256_ops::get_uint256("7c3234094dff9a45064a5b9abd0667c04dd76c62722984f7f8475e7cc344c06") + prime_field::fmul(
						uint256_ops::get_uint256("119bcf6402ad9953851bac8e318d50af699b0cc75e2597aff0a2cc521975aa4") + prime_field::fmul(
							uint256_ops::get_uint256("1dbdc2ea2e555309578eeb2352fbc47c8fd5ed77cc09903b577700f9a4d1be1") + prime_field::fmul(
								uint256_ops::get_uint256("76d656560dac569683063278ea2dee47d935501c2195ff53b741efe81509892") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("1cdf0446663046f35c26d51e45a5233a93c51f4f7f1985dfe130dd67addefa3") + prime_field::fmul(
			uint256_ops::get_uint256("6df73a948c95439f3230282814ba7e26203cfdc725901e4971ad9cff4db4396") +  prime_field::fmul(
				uint256_ops::get_uint256("9969a08d753e885857a5696d1cafd39f62bb193acc99089df76c240acd2fc0") + prime_field::fmul(
					uint256_ops::get_uint256("2065bc7a4aa38d5fe86f9b593ccd060f8d4a5a19a9ca8b182c32199a4bd27be") + prime_field::fmul(
						uint256_ops::get_uint256("611384709c407d85c93256b6aff04c4ac515450c70cf507994165abfe2347b") + prime_field::fmul(
							uint256_ops::get_uint256("9460aa25f77fc10cfcc4579e2011e39ce477a32a768aa553201e556ed2bbe1") + prime_field::fmul(
								uint256_ops::get_uint256("7f0a3bec1d34f2fd632993a3d9c6432401cec25ad9d6196b909f3672980bd05") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("47dc0e209ee8d0b67f63d9e63837ff2ab462c4839bc14a1a3e802327ff0e31f") + prime_field::fmul(
			uint256_ops::get_uint256("35ca7fa56aa38486833a976804899ba3c97fdaa0a23056cd2dc9bfdbcdd2e31") +  prime_field::fmul(
				uint256_ops::get_uint256("575531b404cdba72a63dbbd17aef7d9ae00f73eca7c6dcdaf5e0778c921be41") + prime_field::fmul(
					uint256_ops::get_uint256("319c68159cdf104c2543486ff784860f302187d77effb9a5fefe4e16f0ddc2c") + prime_field::fmul(
						uint256_ops::get_uint256("49aadcf98ef59c0e5d2097845949988862b96194abc8c5453f056f232482892") + prime_field::fmul(
							uint256_ops::get_uint256("5030fda0c29a929e6cd634b9f3d1bf975c363012cfb439cae13495f8ce10225") + prime_field::fmul(
								uint256_ops::get_uint256("59cbe680183d1dc3161ee7f945f38ab9461a5293748b2b7be84899e62c9860b") + prime_field::fmul(
									result.clone(), x.clone()
								), x.clone()
							), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			), x.clone()
		);

	result = uint256_ops::get_uint256("562f636b49796e469dfe9e6748c4468f340e8f69e3f79cfe6925a261198dbb3") + prime_field::fmul(
				uint256_ops::get_uint256("7dd14b0299ff6064a96fe97e086df3f64a4c7e8b4a58a5bd5fe1b9cf7c61e7c") + prime_field::fmul(
					uint256_ops::get_uint256("73c57ecea0c64a9bc087e50a97a28df974b294c52a0ef5854f53f69ef6773af") + prime_field::fmul(
						uint256_ops::get_uint256("744bdf0c2894072564f6eca2d26efc03ef001bc6e78b34bf6be3a1a91fd90fc") + prime_field::fmul(
							result.clone(), x.clone()
						), x.clone()
					), x.clone()
				), x.clone()
			);

	

	return prime_field::mod_prime(result);
}