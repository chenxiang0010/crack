use super::constant::VARIANT_BASE64_DICT;
use super::util::encrypt_decrypt_bytes;
use crate::config::MobaXterm;
use anyhow::{Context, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

fn build_zip(license: &[u8]) -> anyhow::Result<()> {
    let output_path = Path::new("output/mobaxterm/Custom.mxtpro");
    let file = File::create(output_path).context("Failed to create ZIP file")?;
    let mut zip_file = ZipWriter::new(file);
    let options = FileOptions::<()>::default().compression_method(CompressionMethod::Stored);
    zip_file
        .start_file("Pro.key", options)
        .context("Failed to add license to ZIP")?;
    zip_file
        .write_all(license)
        .context("Failed to write license to ZIP")?;
    zip_file.finish().context("Failed to finalize ZIP file")?;
    Ok(())
}

fn parse_version(version: &str) -> anyhow::Result<(&str, &str)> {
    let reg = Regex::new(r"^\d+\.\d+$").context("Failed to create version regex.")?;
    if reg.is_match(version) {
        let version_parts: Vec<&str> = version.split('.').collect();
        Ok((version_parts[0], version_parts[1]))
    } else {
        Err(anyhow!("Invalid version format."))
    }
}

fn process_block_encode(
    start_index: usize,
    byte_count: usize,
    base64_dict: &HashMap<usize, char>,
    bytes: &[u8],
) -> Vec<u8> {
    let coding_int = {
        let mut buffer = [0u8; 4];
        buffer[..byte_count].copy_from_slice(&bytes[start_index..start_index + byte_count]);
        i32::from_le_bytes(buffer)
    };
    let step_count = match byte_count {
        1 => 2,
        2 => 3,
        3 => 4,
        _ => unreachable!("Invalid byte count"),
    };
    let mut block = String::with_capacity(step_count);
    (0..step_count).for_each(|i| {
        let index = ((coding_int >> (i * 6)) & 0x3F) as usize;
        block.push(base64_dict[&index]);
    });
    block.into_bytes()
}

fn variant_base64_encode(bytes: &[u8]) -> Vec<u8> {
    let result_len = (bytes.len() * 4 + 2) / 3;
    let mut result = Vec::with_capacity(result_len);

    for i in (0..bytes.len()).step_by(3) {
        let chunk_size = std::cmp::min(3, bytes.len() - i);
        let block = process_block_encode(i, chunk_size, &VARIANT_BASE64_DICT, bytes);
        result.extend_from_slice(&block);
    }

    result
}

fn build_license_code(config: &MobaXterm) -> anyhow::Result<Vec<u8>> {
    let MobaXterm {
        username,
        version,
        license_type,
        count,
    } = config;
    let (major, minor) = parse_version(version)?;
    let license_type = license_type.to_int();
    let license_string =
        format!("{license_type}#{username}|{major}{minor}#{count}#{major}3{minor}6{minor}#0#0#0#");
    let encrypted_code = encrypt_decrypt_bytes(&mut 0x787, &license_string.into_bytes(), true);
    let license_code = variant_base64_encode(&encrypted_code);
    Ok(license_code)
}

pub fn encrypt(config: &MobaXterm) -> anyhow::Result<()> {
    let license_code = build_license_code(config)?;
    build_zip(&license_code)?;
    println!(
        "Generation successful! Please move 'Custom.mxtpro' to MobaXterm installation directory."
    );
    Ok(())
}
