use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use hex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY};
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};

struct Algorithm{
	name: &'static str,
	help: &'static str,
	f: fn(data: Vec<u8>) -> Result<Vec<u8>, String>,
}

lazy_static!{
	static ref RAW_ALGORITHMS : Vec<Algorithm> = vec![
		Algorithm {
			name : "md5",
			help : "MD5",
			f: md5,
		},
		Algorithm {
			name : "sha1",
			help : "SHA-1",
			f: sha1,
		},
		Algorithm {
			name : "sha2_224",
			help : "SHA-2 224",
			f: sha2_224,
		},
		Algorithm {
			name : "sha2_256",
			help : "SHA-2 256",
			f: sha2_256,
		},
		Algorithm {
			name : "sha2_384",
			help : "SHA-2 384",
			f: sha2_384,
		},
		Algorithm {
			name : "sha2_512",
			help : "SHA-2 512",
			f: sha2_512,
		},
		Algorithm {
			name : "sha2_512_224",
			help : "SHA-2 512 truncate 224",
			f: sha2_512_224,
		},
		Algorithm {
			name : "sha2_512_256",
			help : "SHA-2 512 truncate 256",
			f: sha2_512_256,
		},
		Algorithm {
			name : "sha3_224",
			help : "SHA-3 224",
			f: sha3_224,
		},
		Algorithm {
			name : "sha3_256",
			help : "SHA-3 256",
			f: sha3_256,
		},
		Algorithm {
			name : "sha3_384",
			help : "SHA-3 384",
			f: sha3_384,
		},
		Algorithm {
			name : "sha3_512",
			help : "SHA-3 512",
			f: sha3_512,
		},
		Algorithm {
			name : "sha3_k_224",
			help : "SHA-3 keccak 224",
			f: sha3_k_224,
		},
		Algorithm {
			name : "sha3_k_256",
			help : "SHA-3 keccak 256",
			f: sha3_k_256,
		},
		Algorithm {
			name : "sha3_k_384",
			help : "SHA-3 keccak 384",
			f: sha3_k_384,
		},
		Algorithm {
			name : "sha3_k_512",
			help : "SHA-3 keccak 512",
			f: sha3_k_512,
		},
		Algorithm {
			name : "ripemd_160",
			help : "RIPEMD-160",
			f: ripemd_160,
		}
	];

	static ref ALGORITHMS : HashMap<&'static str, &'static Algorithm> = RAW_ALGORITHMS.iter().map(|x|(x.name, x)).collect();

	static ref ALGORITHM_HELP : String = "Hash algorithm\n".to_string() + &RAW_ALGORITHMS.iter().map(|a|{
		format!("{}: {}", a.name, a.help)
	}).collect::<Vec<String>>().join("\n");
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {

	vec![
		Command {
			app: SubCommand::with_name("hash").about("Hex to hash")
				.arg(
					Arg::with_name("a")
						.short("a").help(&ALGORITHM_HELP)
						.takes_value(true)
						.required(true))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: hash,
		},
	]
}

fn hash(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let a_name = matches.value_of("a").ok_or("Invalid algorithm")?;

	let result = match ALGORITHMS.get(a_name) {
		Some(a) => (a.f)(input)?,
		None => return Err("Invalid algorithm".to_string()),
	};

	let result = hex::encode(result);
	let result = "0x".to_string() + &result;

	Ok(vec![result])
}

fn md5(data: Vec<u8>) -> Result<Vec<u8>, String> {

	Ok(md5::compute(data).0.to_vec())
}

fn sha1(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
	context.update(&data);
	let result = context.finish().as_ref().to_vec();
	Ok(result)
}

fn sha2_224(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha224::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_256(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha256::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_384(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha384::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha512::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512_224(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha512Trunc224::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512_256(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = Sha512Trunc256::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_224(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Sha3_224::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_256(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Sha3_256::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_384(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Sha3_384::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_512(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Sha3_512::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_224(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Keccak224::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_256(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Keccak256::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_384(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Keccak384::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_512(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = sha3::Keccak512::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn ripemd_160(data: Vec<u8>) -> Result<Vec<u8>, String> {

	let mut hasher = ripemd160::Ripemd160::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_hash_md5() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "md5", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x900150983cd24fb0d6963f7d28e17f72".to_string()]));

	}

	#[test]
	fn test_hash_sha1() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha1", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xa9993e364706816aba3e25717850c26c9cd0d89d".to_string()]));

	}

	#[test]
	fn test_hash_sha2_224() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_224", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7".to_string()]));

	}

	#[test]
	fn test_hash_sha2_256() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_256", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string()]));

	}

	#[test]
	fn test_hash_sha2_384() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_384", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xcb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7".to_string()]));

	}

	#[test]
	fn test_hash_sha2_512() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_512", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f".to_string()]));

	}

	#[test]
	fn test_hash_sha2_512_224() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_512_224", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x4634270f707b6a54daae7530460842e20e37ed265ceee9a43e8924aa".to_string()]));

	}

	#[test]
	fn test_hash_sha2_512_256() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha2_512_256", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23".to_string()]));

	}

	#[test]
	fn test_hash_sha3_224() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_224", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xe642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf".to_string()]));

	}

	#[test]
	fn test_hash_sha3_256() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_256", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532".to_string()]));

	}

	#[test]
	fn test_hash_sha3_384() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_384", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25".to_string()]));

	}

	#[test]
	fn test_hash_sha3_512() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_512", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xb751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0".to_string()]));

	}

	#[test]
	fn test_hash_sha3_k_224() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_k_224", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xc30411768506ebe1c2871b1ee2e87d38df342317300a9b97a95ec6a8".to_string()]));

	}

	#[test]
	fn test_hash_sha3_k_256() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_k_256", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45".to_string()]));

	}

	#[test]
	fn test_hash_sha3_k_384() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_k_384", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0xf7df1165f033337be098e7d288ad6a2f74409d7a60b49c36642218de161b1f99f8c681e4afaf31a34db29fb763e3c28e".to_string()]));

	}

	#[test]
	fn test_hash_sha3_k_512() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "sha3_k_512", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x18587dc2ea106b9a1563e32b3312421ca164c7f1f07bc922a9c83d77cea3a1e5d0c69910739025372dc14ac9642629379540c17e2a65b19d77aa511a9d00bb96".to_string()]));

	}

	#[test]
	fn test_hash_ripemd_160() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["hash", "-a", "ripemd_160", "0x616263"]);
		assert_eq!(hash(&matches) , Ok(vec!["0x8eb208f7e05d987a9b044a8e98c6b087f15a0bfc".to_string()]));

	}

}