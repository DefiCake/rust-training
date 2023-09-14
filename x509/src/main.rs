use std::{ io::{ BufReader, Read }, path::PathBuf };

use anyhow::{ Context, Result };
use clap::Parser;
use openssl::{ pkcs12::Pkcs12, hash::MessageDigest };
use p12::PFX;
use sha2::{ Digest, Sha256 };
use x509_parser::public_key::PublicKey;

#[derive(Parser, Debug)]
struct Cli {
  path: Option<PathBuf>,
  pass: Option<String>,
}

#[allow(dead_code)]
fn read_p12_bytes(path: &PathBuf) -> Vec<u8> {
  let file = std::fs::File
    ::open(&path)
    .with_context(|| format!("Could not read path {}", path.clone().into_os_string().into_string().unwrap()))
    .unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut buf = Vec::new();
  buf_reader.read_to_end(&mut buf).expect("Could not read the file");

  buf
}

#[allow(dead_code)]
fn extract_x509(p12_bytes: &Vec<u8>, password: &String) -> Option<Vec<u8>> {
  let pkcs12 = PFX::parse(p12_bytes).expect("P12.Error parsing file");

  let bags = &pkcs12.bags(password).unwrap();

  for bag in bags {
    let cert: Option<p12::CertBag> = match &bag.bag {
      p12::SafeBagKind::CertBag(cert) => { Some(cert.clone()) }
      _ => { None }
    };

    if cert.is_none() {
      continue;
    }

    if let Some(p12::CertBag::X509(cert_bytes)) = cert {
      return Some(cert_bytes);
    }
  }

  None
}

#[allow(dead_code)]
fn extract_hash_and_pubkey(cert_bytes: &Vec<u8>) -> (String, Option<String>) {
  let hash = Sha256::new().chain_update(cert_bytes).finalize();
  let hash_hex = hex::encode(hash);

  let (_, parsed) = x509_parser::parse_x509_certificate(&cert_bytes).unwrap();
  let pub_key_hex = match parsed.public_key().parsed() {
    Ok(PublicKey::RSA(rsa_key)) => Some(hex::encode(rsa_key.modulus)),
    Err(_) => None,
    _ => None,
  };
  (hash_hex, pub_key_hex)
}

fn main() -> Result<()> {
  let args = Cli::parse();

  let path = &args.path.unwrap_or_else(|| { PathBuf::from("test_artifacts/member/member.p12") });
  let password: &String = &args.pass.unwrap_or_else(|| { String::from("") });

  let member_p12_bytes = read_p12_bytes(path);
  let ca_p12_bytes = read_p12_bytes(&PathBuf::from("test_artifacts/ca/ca.p12"));
  let member_x509_bytes = extract_x509(&member_p12_bytes, password).unwrap();
  let ca_x509_bytes = extract_x509(&ca_p12_bytes, &String::from("")).unwrap();

  let hash: Vec<u8> = Sha256::new().chain_update(&member_x509_bytes).finalize().to_vec();
  let hash_hex = hex::encode(&hash);
  println!("  [no-std] Hash: {}", &hash_hex);

  let (_, parsed) = x509_parser::parse_x509_certificate(&member_x509_bytes).unwrap();
  println!("  [no-std] Subject: {} {}", &parsed.subject(), hex::encode(&parsed.subject().as_raw()));
  println!("  [no-std] Key.raw: {}", hex::encode(&parsed.public_key().raw));
  println!("  [no-std] Expiry: {} {}", parsed.validity().not_after.timestamp(), parsed.validity().not_after);

  let pub_key_hex = match parsed.public_key().parsed() {
    Ok(PublicKey::RSA(rsa_key)) => Some(hex::encode(rsa_key.modulus)),
    Err(_) => None,
    _ => None,
  };

  if let Some(pub_key_hex) = pub_key_hex {
    println!("  [no-std] Parsed public key: {}", pub_key_hex);
  }

  let ca_x509 = x509_parser::parse_x509_certificate(&ca_x509_bytes).unwrap().1;
  let public_key = ca_x509.public_key();
  let res = parsed.verify_signature(Some(public_key));
  if res.is_err() {
    println!("  [no-std] Signature verification failed");
  } else {
    println!("  [no-std] Signature verification correct");
  }

  let pkcs12 = Pkcs12::from_der(&member_p12_bytes)?;

  // This gives Error: error:0308010C:digital envelope routines:inner_evp_generic_fetch:unsupported:crypto/evp/evp_fetch.c... it might have to do with OS dependencies,
  // need to enable legacy openssl
  // Update: solved by adding features = ["vendored"] to openssl crate
  let parsed = pkcs12.parse2(&password)?;

  let cert: openssl::x509::X509 = parsed.cert.unwrap();
  let digest: openssl::hash::DigestBytes = cert.digest(MessageDigest::sha256())?;
  let h = hex::encode(digest);
  let pub_key = hex::encode(cert.public_key()?.public_key_to_der()?);
  let validity = cert.not_after();

  dbg!(h);
  dbg!(cert.subject_name());
  dbg!(cert.subject_name_hash());
  dbg!(pub_key);
  dbg!(validity);

  // let t = String::from_utf8(cert.to_text().unwrap()).unwrap();
  // println!("{}", t);

  // let pkcs12 = Pkcs12::from_der(&ca_p12_bytes)?;
  // println!("{}", String::from_utf8(pkcs12.parse2(&password)?.cert.unwrap().to_text().unwrap()).unwrap());
  Ok(())
}

#[test]
fn test_info_extraction() {
  let path = PathBuf::from("test_artifacts/member/member.p12");
  let password: &String = &String::from("");

  let p12_bytes = read_p12_bytes(&path);
  let cert_bytes = extract_x509(&p12_bytes, password);

  assert!(!cert_bytes.is_none(), "Could not find a certificate");

  let cert_bytes = cert_bytes.unwrap();

  let (hash_hex, pub_key_hex) = extract_hash_and_pubkey(&cert_bytes);

  assert!(!pub_key_hex.is_none(), "Could not find a public key inside the X509 cert");

  assert_eq!(
    pub_key_hex.unwrap(),
    String::from(
      "0097c9cddd7da2c428bb11c2fdeae141ccfa0e965762f963946bf0ad2d7293092c6d5ffffc493aad5b3064d44277676e92615e203e3e39d16963d1f74fa9ed225b57600129404f0f3f1749434636b01dc703fd3f0cbec6c95f3fdbe7528887b55016aa6c9849575c3431e2be4a699f988cfe1e229806e8d6d90dc5e15389e9b3ea213169fdc04be92e262ba7335a0d69405883cd3e36e2791ae0d1a3239ee27f36a94f4e91e76905817fc0c72f08992ce4bb79efbc736781922664e5f4ec1b6b8376192d66e75d03bc295cd5bec53b4c222591b670c115f1d65164c89ae53e7eafa26016746ec148f5232b855e0d6b04e7c7bf2a3ad48416b0769a089162290bc3"
    )
  );

  let pkcs12 = Pkcs12::from_der(&p12_bytes).unwrap();
  let parsed = pkcs12.parse2(&password).unwrap();
  let cert: openssl::x509::X509 = parsed.cert.unwrap();
  let digest: openssl::hash::DigestBytes = cert.digest(MessageDigest::sha256()).unwrap();

  // Assert that the no-std impl throws the same data as the vendored impl
  assert_eq!(hex::encode(digest), hash_hex);
}

#[test]
fn test_chain_validity() {
  let path = PathBuf::from("test_artifacts/member/member.p12");
  let password: &String = &String::from("");

  let p12_bytes = read_p12_bytes(&path);
  let cert = extract_x509(&p12_bytes, password);

  assert!(!cert.is_none(), "Could not find a certificate");

  let cert = extract_x509(&p12_bytes, password);

  assert!(!cert.is_none(), "Could not find a certificate");
}
