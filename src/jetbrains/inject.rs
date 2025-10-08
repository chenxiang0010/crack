use anyhow::{Context, Result};
use std::fs;

use super::cert_generator::generate_and_save_cert;
use super::{constant, xyzr};

pub fn inject() -> Result<()> {
    generate_and_save_cert().with_context(|| "Failed to generate and save certificate")?;
    let power_content =
        xyzr::load_power_conf().with_context(|| "Failed to load power configuration")?;
    fs::write(constant::POWER_FILE_PATH, power_content)
        .with_context(|| "Failed to write power configuration file")?;
    Ok(())
}
