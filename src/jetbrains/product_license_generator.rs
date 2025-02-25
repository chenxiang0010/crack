use super::constant::{CA_CERT_FILE_PATH, CA_KEY_FILE_PATH};
use anyhow::{Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::x509::X509;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub(crate) code: String,
    fallback_date: String,
    paid_up_to: String,
    extended: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LicenseInfo {
    license_id: String,
    licensee_name: String,
    assignee_name: String,
    assignee_email: String,
    license_restriction: String,
    check_concurrent_use: bool,
    pub(crate) products: Vec<Product>,
    metadata: String,
    hash: String,
    grace_period_days: u8,
    auto_prolongated: bool,
    is_auto_prolongated: bool,
}

impl LicenseInfo {
    fn new(license_info_req: &LicenseInfoReq) -> Self {
        let mut products = Vec::new();
        for code in license_info_req.product_code.split(',') {
            if !code.trim().is_empty() {
                products.push(Product {
                    code: code.trim().to_string(),
                    fallback_date: license_info_req.expire_at.clone(),
                    paid_up_to: license_info_req.expire_at.clone(),
                    extended: false,
                });
            }
        }

        Self {
            license_id: "6E26WZCE14".to_string(),
            licensee_name: license_info_req.licensee_name.clone(),
            assignee_name: license_info_req.assignee_name.clone(),
            assignee_email: "".to_string(),
            license_restriction: "".to_string(),
            check_concurrent_use: false,
            products,
            metadata: "0120230914PSAX000005".to_string(),
            hash: "".to_string(),
            grace_period_days: 7,
            auto_prolongated: false,
            is_auto_prolongated: false,
        }
    }
}

pub struct LicenseInfoReq {
    pub licensee_name: String,
    pub assignee_name: String,
    pub expire_at: String,
    pub product_code: String,
}

pub fn generate_license_code(license_info_req: LicenseInfoReq) -> Result<String> {
    let cert = fs::read_to_string(CA_CERT_FILE_PATH).context("Failed to read certificate file")?;
    let cert = X509::from_pem(cert.as_bytes()).context("Failed to parse certificate")?;
    let license_info = LicenseInfo::new(&license_info_req);
    let license_part =
        serde_json::to_string(&license_info).context("Failed to serialize license info")?;
    let license_part_base64 = STANDARD.encode(license_part.as_bytes());
    let key_data = fs::read(CA_KEY_FILE_PATH)?;
    let private_key = PKey::private_key_from_pem(&key_data)?;
    let mut signer = Signer::new(MessageDigest::sha1(), &private_key)?;
    signer.update(license_part.as_bytes())?;
    let signature = signer.sign_to_vec()?;
    let signature_base64 = STANDARD.encode(&signature);
    let cert_base64 = STANDARD.encode(&cert.to_der()?);
    Ok(format!(
        "{}-{}-{}-{}",
        license_info.license_id, license_part_base64, signature_base64, cert_base64
    ))
}
