use std::{ io::{ BufReader, Read }, path::PathBuf };

use anyhow::{ Context, Result };
use clap::Parser;
use openssl::{ pkcs12::Pkcs12, hash::MessageDigest };
use p12::PFX;
use sha2::{ Digest, Sha256 };

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

  // // TODO:
  // // - Check why some certs give ASN1 error for the pure lib
  // // - Get a digest of the certificate from the pure lib
  // // - Cannot obtain it without parsing, so parsing is FUNDAMENTAL
  for bag in bags {
    match &bag.bag {
      p12::SafeBagKind::Pkcs8ShroudedKeyBag(_) => println!("Shrouded"),
      p12::SafeBagKind::CertBag(cert) => {
        match cert {
          p12::CertBag::X509(bytes) => {
            let hash = Sha256::new().chain_update(bytes).finalize();
            let hash_hex = hex::encode(hash);
            dbg!(hash_hex);
          }
          p12::CertBag::SDSI(_) => println!("CertBag -> SDSI"),
        }
      }
      p12::SafeBagKind::OtherBagKind(_) => println!("Other"),
    };
  }

  let pkcs12 = Pkcs12::from_der(&buf)?;

  // This gives Error: error:0308010C:digital envelope routines:inner_evp_generic_fetch:unsupported:crypto/evp/evp_fetch.c... it might have to do with OS dependencies,
  // need to enable legacy openssl
  // Update: solved by adding features = ["vendored"] to openssl crate
  let parsed = pkcs12.parse2(&password)?;

  let cert: openssl::x509::X509 = parsed.cert.unwrap();
  let digest: openssl::hash::DigestBytes = cert.digest(MessageDigest::sha256())?;

  let h = hex::encode(digest);
  dbg!(cert);
  dbg!(h);

  Ok(())
}
