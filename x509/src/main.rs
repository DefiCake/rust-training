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

  let path = &args.path.unwrap_or_else(|| { PathBuf::from("test_artifacts/cert.p12") });
  let password: &String = &args.pass.unwrap_or_else(|| { String::from("") });

  let file = std::fs::File
    ::open(&path)
    .with_context(|| format!("Could not read path {}", path.clone().into_os_string().into_string().unwrap()))?;

  let mut buf_reader = BufReader::new(file);
  let mut buf = Vec::new();
  buf_reader.read_to_end(&mut buf)?;

  let pkcs12 = PFX::parse(&buf).expect("P12.Error parsing file");

  let bags = &pkcs12.bags(password)?;

  for bag in bags {
    let cert: Option<&p12::CertBag> = match &bag.bag {
      p12::SafeBagKind::CertBag(cert) => { Some(cert) }
      _ => { None }
    };

    if let Some(p12::CertBag::X509(cert_bytes)) = cert {
      let hash = Sha256::new().chain_update(cert_bytes).finalize();
      let hash_hex = hex::encode(hash);
      dbg!(hash_hex);
      let (_, parsed) = x509_parser::parse_x509_certificate(&cert_bytes).unwrap();
      println!("  Subject: {} {}", &parsed.subject(), hex::encode(&parsed.subject().as_raw()));
      println!("  Key: {}", hex::encode(&parsed.public_key().raw));
      println!("  Expiry: {} {}", parsed.validity().not_after.timestamp(), parsed.validity().not_after);

      let pub_key_hex = match parsed.public_key().parsed() {
        Ok(PublicKey::RSA(rsa_key)) => Some(hex::encode(rsa_key.modulus)),
        Err(_) => None,
        _ => None,
      };

      if let Some(pub_key_hex) = pub_key_hex {
        println!("  Public key: {}", pub_key_hex);
      }

      break;
    }
  }

  let pkcs12 = Pkcs12::from_der(&buf)?;

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

  Ok(())
}

#[test]
fn test_info_extraction() {
  let path = PathBuf::from("test_artifacts/cert.p12");
  let password: &String = &String::from("");

  let file = std::fs::File
    ::open(&path)
    .with_context(|| format!("Could not read path {}", path.clone().into_os_string().into_string().unwrap()))
    .unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut buf = Vec::new();
  buf_reader.read_to_end(&mut buf).unwrap();

  let pkcs12 = PFX::parse(&buf).expect("P12.Error parsing file");

  let bags = &pkcs12.bags(password).unwrap();
  let mut cert: Option<&p12::CertBag> = None;

  for bag in bags {
    let _cert: Option<&p12::CertBag> = match &bag.bag {
      p12::SafeBagKind::CertBag(cert) => { Some(cert) }
      _ => { None }
    };

    if _cert.is_none() {
      continue;
    }

    cert = _cert.clone();
  }

  assert!(!cert.is_none(), "Could not find a certificate");

  if let Some(p12::CertBag::X509(cert_bytes)) = cert {
    let (hash_hex, pub_key_hex) = extract_hash_and_pubkey(cert_bytes);

    assert!(!pub_key_hex.is_none(), "Could not find a public key inside the X509 cert");

    assert_eq!(
      pub_key_hex.unwrap(),
      String::from(
        "00b0a24988ecfa102b501764941da50ad15286c6cc88c6be19ba61e3f5bb66fce5459e17e64be607135cf709ae44c6d30c4d418d78dc63232ddcfb46ab3600b51e580ca74e79f8c9fe482de14f5c2518c08c7accb46fd89f92dc4b1680588b27c6c070490914e73683429e169bbf4915d547fb80ae55f6e57f6ab74371aca7d225793d99faf041f7fbf64b20f16498a691ef8e0a1877bb4f42aa4edd3d04e56eff446d5889eaaed406aa76a122c045a9cf78e1b832043bf42c6d3d2c169895f29edd1732782205310ee2c26fe718e1a50e5d8df44fdd7190297068d090487d3a591513058f617dff6644357c6768d34f120121329d81964c6db17fc44f3af05015"
      )
    );

    let pkcs12 = Pkcs12::from_der(&buf).unwrap();
    let parsed = pkcs12.parse2(&password).unwrap();
    let cert: openssl::x509::X509 = parsed.cert.unwrap();
    let digest: openssl::hash::DigestBytes = cert.digest(MessageDigest::sha256()).unwrap();
    assert_eq!(hex::encode(digest), hash_hex);
  } else {
    assert!(false, "Could not find an X509 certificate");
  }
}

#[test]
fn test_chain_validity() {}
