use ethers::prelude::rand::thread_rng;
use sha256::{ digest };
use ethabi::{ encode, FixedBytes, Token };
use ethabi::token::{ LenientTokenizer, Tokenizer };
use ethers::prelude::*;

use ethers::{
  core::{ types::TransactionRequest, utils::Anvil },
  middleware::SignerMiddleware,
  providers::{ Http, Middleware, Provider },
  signers::{ LocalWallet, Signer },
};
use rand::Rng;
use ethabi::Token;

#[tokio::main]
async fn main() {
  let selector = hex::decode("1626ba7e").unwrap();
  let hash = hex::decode("265f1ae422b537c21d4f660ba0a1a4d703fc8eba732561832f2070e0a1ecb35e").unwrap();
  let private_key = hex::decode("b5c6d3977d2beba1c1cfc61bddd446786db3db7445a8cdf7747115eea578b1f9").unwrap();
  //   let hash_token = Tokenizer::tokenize_fixed_bytes(hash, 32).unwrap();
  let hash_token = LenientTokenizer::tokenize_fixed_bytes(
    "265f1ae422b537c21d4f660ba0a1a4d703fc8eba732561832f2070e0a1ecb35e",
    32
  );

  let wallet = LocalWallet::from_bytes(&private_key).unwrap();

  println!("Wallet address is {}", &wallet.address());

  let mut rng = rand::thread_rng();
  let mut nonce = 0u32;

  nonce = rng.gen::<u32>();

  let signature = wallet.sign_message(&format!("{}", nonce)).await.unwrap();

  let signature_token: Token = ethabi::Token(LenientTokenizer::tokenize_bytes(&signature.to_string()).unwrap());

  let encoded = encode([signature_token]);

  // loop {
  //   nonce = rng.gen::<u32>();

  //   let signature = wallet.sign_message(&format!("{}", nonce)).await.unwrap();

  //   let signature_token = LenientTokenizer::tokenize_bytes(&hex::encode(&signature)).unwrap();
  //   let encoded = encode(hash_token);
  // }

  // loop {
  //   let signature = wallet.sign_message(&format!("{}", nonce)).await.unwrap();

  //   let signature_token = Tokenizer::tokenize_bytes(signature).unwrap();

  //   let encoded = encode([&hash_token, &signature_token]);
  //   // let encoded = arrayify(coder.encode(&[("bytes32", "bytes")], &[(&hash, &signature)])).unwrap();

  //   let mut data: Vec<u8> = [selector, encoded].concat();

  //   let result = digest(&data);

  //   if result[..10] == *selector {
  //     println!("Found {} {} {} {} {}", result, wallet.private_key().unwrap(), wallet.address(), nonce, signature);
  //     break;
  //   }

  //   nonce += 1;
  // }
}
