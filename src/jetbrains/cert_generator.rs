use super::constant::{CA_CERT_FILE_PATH, CA_KEY_FILE_PATH};
use anyhow::Result;
use openssl::asn1::Asn1Time;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::{X509NameBuilder, X509ReqBuilder, X509};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum CertError {
    #[error("Failed to generate RSA key: {0}")]
    RsaGeneration(#[from] openssl::error::ErrorStack),
    #[error("Failed to create certificate file: {0}")]
    FileCreation(#[from] std::io::Error),
    #[error("Failed to convert certificate: {0}")]
    CertConversion(#[from] anyhow::Error),
}

pub fn generate_and_save_cert() -> Result<(), CertError> {
    if Path::new(CA_CERT_FILE_PATH).exists() && Path::new(CA_KEY_FILE_PATH).exists() {
        eprintln!("Certificate already exists");
        return Ok(());
    }
    let (cert, priv_key_pem) = generate_self_signed_cert()?;
    save_cert_and_key(cert, priv_key_pem)?;
    Ok(())
}

fn generate_self_signed_cert() -> Result<(X509, Vec<u8>), CertError> {
    let rsa = Rsa::generate(4096)?;
    let pkey = PKey::from_rsa(rsa)?;
    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_pubkey(&pkey)?;
    let mut name_builder = X509NameBuilder::new()?;
    name_builder.append_entry_by_nid(Nid::COMMONNAME, "JetProfile CA")?;
    let name = name_builder.build();
    req_builder.set_subject_name(&name)?;
    let _req = req_builder.build();
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = openssl::bn::BigNum::from_u32(1)?.to_asn1_integer()?;
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&name)?;
    cert_builder.set_issuer_name(&name)?;
    cert_builder.set_pubkey(&pkey)?;
    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(365 * 10)?;
    cert_builder.set_not_after(&not_after)?;
    cert_builder.append_extension(
        openssl::x509::extension::BasicConstraints::new()
            .critical()
            .ca()
            .build()?,
    )?;
    cert_builder.append_extension(
        openssl::x509::extension::SubjectKeyIdentifier::new()
            .build(&cert_builder.x509v3_context(None, None))?,
    )?;
    cert_builder.sign(&pkey, MessageDigest::sha256())?;
    let cert = cert_builder.build();
    Ok((cert, pkey.private_key_to_pem_pkcs8()?))
}

fn save_cert_and_key(cert: X509, priv_key_pem: Vec<u8>) -> Result<(), CertError> {
    if let Some(cert_dir) = Path::new(CA_CERT_FILE_PATH).parent() {
        std::fs::create_dir_all(cert_dir)?;
    }
    if let Some(key_dir) = Path::new(CA_KEY_FILE_PATH).parent() {
        std::fs::create_dir_all(key_dir)?;
    }
    let mut file = File::create(CA_CERT_FILE_PATH)?;
    file.write_all(&cert.to_pem().map_err(CertError::RsaGeneration)?)?;
    let mut file = File::create(CA_KEY_FILE_PATH)?;
    file.write_all(&priv_key_pem)?;
    Ok(())
}
