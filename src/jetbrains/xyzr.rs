use anyhow::{Context, Result};
use num_bigint::{BigUint, ToBigUint};
use openssl::bn::{BigNum, BigNumContext};
use openssl::x509::X509;
use std::fs::File;
use std::io::Read;

use super::constant::{CA_CERT_FILE_PATH, ROOT_CERTIFICATE, Y};

pub fn load_power_conf() -> Result<String> {
    let mut file =
        File::open(CA_CERT_FILE_PATH).with_context(|| "Failed to open certificate file")?;
    let mut cert_buffer = Vec::new();
    file.read_to_end(&mut cert_buffer)
        .with_context(|| "Failed to read certificate file")?;
    let cert = X509::from_pem(&cert_buffer).with_context(|| "Failed to parse certificate")?;
    let signature_bytes = cert.signature().as_slice();
    let x = BigUint::from_bytes_be(signature_bytes);
    let y: BigUint = Y.to_biguint().unwrap();
    let root_cert = X509::from_pem(ROOT_CERTIFICATE.as_bytes())
        .with_context(|| "Failed to parse root certificate")?;
    let root_public_key = root_cert.public_key()?;
    let root_rsa = root_public_key.rsa()?;
    let z = BigUint::from_bytes_be(&root_rsa.n().to_vec());
    let public_key = cert.public_key()?;
    let rsa = public_key.rsa()?;
    let signature = BigNum::from_slice(cert.signature().as_slice())?;
    let mut ctx = BigNumContext::new()?;
    let mut r = BigNum::new()?;
    r.mod_exp(&signature, rsa.e(), rsa.n(), &mut ctx)?;
    Ok(format!("[Result]\nEQUAL,{x},{y},{z}->{r}"))
}
