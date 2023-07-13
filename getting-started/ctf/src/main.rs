use ethabi::{ encode, ParamType };
use ethabi::token::{ LenientTokenizer, Tokenizer };
use ethers::prelude::*;
use rayon::prelude::*;
use rand::Rng;

use sha2::{ Sha256, Digest };

fn main() {
  let selector_str = "1626ba7e";
  let selector = hex::decode(&selector_str).unwrap();
  // let hash = hex::decode("265f1ae422b537c21d4f660ba0a1a4d703fc8eba732561832f2070e0a1ecb35e").unwrap();
  let private_key = hex::decode("b5c6d3977d2beba1c1cfc61bddd446786db3db7445a8cdf7747115eea578b1f9").unwrap();

  let hash_token = LenientTokenizer::tokenize(
    &ParamType::FixedBytes(32),
    "265f1ae422b537c21d4f660ba0a1a4d703fc8eba732561832f2070e0a1ecb35e"
  ).unwrap();

  let wallet = LocalWallet::from_bytes(&private_key).unwrap();

  println!("Wallet address is {}", &wallet.address());

  loop {
    let results: Vec<String> = (1i32..1_000_000)
      .into_par_iter()
      .map_init(
        || rand::thread_rng(), // get the thread-local RNG
        |rng, _x| {
          // let mut nonce = rng.gen::<u32>();
          // let signature = wallet.sign_message(&format!("{}", nonce)).await.unwrap();
          let signature = hex::encode(rng.gen::<[u8; 32]>());
          let signature_token = LenientTokenizer::tokenize(&ParamType::Bytes, &signature).unwrap();
          let encoded = encode(&[hash_token.clone(), signature_token.clone()]);
          let data = [selector.clone(), encoded].concat();

          let mut hasher: Sha256 = Sha256::new();
          hasher.update(&data);

          let result = hex::encode(hasher.finalize());

          if result.starts_with(selector_str) {
            println!("Found result {} with signature", result);
            println!("Signature {} {}", 32, &signature);
          }

          result
        }
      )
      .collect();

    println!("Ended round with a result {}", results.last().unwrap());
  }
}
