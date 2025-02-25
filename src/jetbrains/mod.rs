mod cert_generator;
mod code;
mod constant;
mod inject;
mod product_license_generator;
mod xyzr;

use crate::config::JetBrains;
use anyhow::Context;
use product_license_generator::LicenseInfoReq;
use std::fs;

async fn generate_license(config: &JetBrains) -> anyhow::Result<()> {
    let product_code = code::get_code().context("Failed to get product code")?;
    let license_info = LicenseInfoReq {
        licensee_name: config.licensee_name.clone(),
        assignee_name: config.assignee_name.clone(),
        expire_at: config.expire_at.clone(),
        product_code,
    };
    let license_code = product_license_generator::generate_license_code(license_info)?;
    fs::write(constant::LICENSE_FILE_PATH, license_code).context("Failed to write license file")?;
    Ok(())
}

pub async fn run(config: &JetBrains) -> anyhow::Result<()> {
    if config.update_code {
        code::update_code().await?;
    }
    inject::inject()?;
    generate_license(config).await?;
    Ok(())
}
