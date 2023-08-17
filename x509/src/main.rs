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

  for bag in bags {
    let cert: Option<&p12::CertBag> = match &bag.bag {
      p12::SafeBagKind::CertBag(cert) => { Some(cert) }
      _ => { None }
    };

    if let Some(p12::CertBag::X509(cert_bytes)) = cert {
      let hash = Sha256::new().chain_update(cert_bytes).finalize();
      let hash_hex = hex::encode(hash);
      dbg!(hash_hex);

      let byte_pairs = cert_bytes.chunks(2);

      // Collect the byte pairs and convert them to u16 values
      let cert_as_u16_chunks: Vec<u16> = byte_pairs
        .map(|chunk| {
          let a = chunk[0];
          let b = if chunk.len() == 2 { chunk[1] } else { 0x00 };

          // println!("{}", hex::encode(vec![a, b]));
          u16::from_be_bytes([a, b])
        })
        .collect();

      // This might need a whole decoder utility for ASN1 / DER
      let tbs_cert_len: usize = cert_as_u16_chunks[3].into();
      let tbs_cert_start: usize = 8;
      let tbs_cert_end = tbs_cert_start + tbs_cert_len;

      let tbs_cert_bytes = &cert_bytes.as_slice()[tbs_cert_start..tbs_cert_end];

      dbg!(hex::encode(tbs_cert_bytes));
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
  dbg!(cert);
  dbg!(h);

  Ok(())
}
