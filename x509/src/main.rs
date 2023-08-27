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

      let key_hex = match parsed.public_key().parsed() {
        Ok(PublicKey::RSA(rsa_key)) => Some(hex::encode(rsa_key.modulus)),
        Err(_) => None,
        _ => None,
      };

      if let Some(key_hex) = key_hex {
        println!("  Public key: {}", key_hex);
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
