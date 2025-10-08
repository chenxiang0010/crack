//! MobaXterm许可证加密模块
//!
//! 负责生成和加密MobaXterm许可证文件

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use super::constant::VARIANT_BASE64_DICT;
use super::util::encrypt_decrypt_bytes;
use crate::config::MobaXterm;

/// 构建MobaXterm许可证ZIP文件
///
/// # 参数
/// * `license` - 许可证数据
///
/// # 返回值
/// * `Ok(())` - ZIP文件创建成功
/// * `Err(anyhow::Error)` - ZIP文件创建失败
fn build_zip(license: &[u8]) -> Result<()> {
    let output_path = Path::new("output/mobaxterm/Custom.mxtpro");

    println!("    📦 正在创建许可证文件: {}", output_path.display());

    let file = File::create(output_path).with_context(|| "创建ZIP文件失败")?;

    let mut zip_file = ZipWriter::new(file);
    let options = FileOptions::<()>::default().compression_method(CompressionMethod::Stored);

    zip_file
        .start_file("Pro.key", options)
        .with_context(|| "添加许可证到ZIP失败")?;

    zip_file
        .write_all(license)
        .with_context(|| "写入许可证数据失败")?;

    zip_file.finish().with_context(|| "完成ZIP文件创建失败")?;

    println!("    ✅ 许可证文件创建完成");
    Ok(())
}

/// 解析版本号字符串
///
/// # 参数
/// * `version` - 版本号字符串 (格式: "主版本.次版本")
///
/// # 返回值
/// * `Ok((主版本, 次版本))` - 解析成功
/// * `Err(anyhow::Error)` - 版本格式无效
fn parse_version(version: &str) -> Result<(&str, &str)> {
    let version_regex = Regex::new(r"^\d+\.\d+$").with_context(|| "创建版本号正则表达式失败")?;

    if !version_regex.is_match(version) {
        return Err(anyhow!(
            "版本号格式无效，应为 '主版本.次版本' 格式，如 '23.1'"
        ));
    }

    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 2 {
        return Err(anyhow!("版本号格式错误"));
    }

    Ok((parts[0], parts[1]))
}

/// 处理单个数据块的Base64编码
///
/// # 参数
/// * `start_index` - 起始索引
/// * `byte_count` - 字节数量
/// * `base64_dict` - Base64字符映射表
/// * `bytes` - 原始字节数据
///
/// # 返回值
/// * `Vec<u8>` - 编码后的字节数据
fn process_block_encode(
    start_index: usize,
    byte_count: usize,
    base64_dict: &HashMap<usize, char>,
    bytes: &[u8],
) -> Vec<u8> {
    // 将字节数据转换为32位整数
    let coding_int = {
        let mut buffer = [0u8; 4];
        buffer[..byte_count].copy_from_slice(&bytes[start_index..start_index + byte_count]);
        i32::from_le_bytes(buffer)
    };

    // 根据字节数量确定编码步数
    let step_count = match byte_count {
        1 => 2,
        2 => 3,
        3 => 4,
        _ => unreachable!("字节数量无效: {}", byte_count),
    };

    // 生成编码块
    let mut block = String::with_capacity(step_count);
    for i in 0..step_count {
        let index = ((coding_int >> (i * 6)) & 0x3F) as usize;
        block.push(base64_dict[&index]);
    }

    block.into_bytes()
}

/// 变体Base64编码
///
/// 使用自定义的Base64字符表进行编码
///
/// # 参数
/// * `bytes` - 待编码的字节数据
///
/// # 返回值
/// * `Vec<u8>` - 编码后的字节数据
fn variant_base64_encode(bytes: &[u8]) -> Vec<u8> {
    if bytes.is_empty() {
        return Vec::new();
    }

    // 预计算结果长度
    let result_len = (bytes.len() * 4).div_ceil(3);
    let mut result = Vec::with_capacity(result_len);

    // 按3字节为一组进行编码
    for i in (0..bytes.len()).step_by(3) {
        let chunk_size = std::cmp::min(3, bytes.len() - i);
        let block = process_block_encode(i, chunk_size, &VARIANT_BASE64_DICT, bytes);
        result.extend_from_slice(&block);
    }

    result
}

/// 构建MobaXterm许可证代码
///
/// # 参数
/// * `config` - MobaXterm配置信息
///
/// # 返回值
/// * `Ok(Vec<u8>)` - 许可证代码字节数据
/// * `Err(anyhow::Error)` - 构建失败
fn build_license_code(config: &MobaXterm) -> Result<Vec<u8>> {
    println!("    🔧 正在构建许可证代码...");

    let MobaXterm {
        username,
        version,
        license_type,
        count,
    } = config;

    // 解析版本号
    let (major, minor) = parse_version(version).with_context(|| "版本号解析失败")?;

    // 获取许可证类型数值
    let license_type_int = license_type.to_int();

    // 构建许可证字符串
    // 格式: 许可证类型#用户名|主版本次版本#数量#主版本3次版本6次版本#0#0#0#
    let license_string = format!(
        "{license_type_int}#{username}|{major}{minor}#{count}#{major}3{minor}6{minor}#0#0#0#"
    );

    println!("    🔐 正在加密许可证数据...");

    // 加密许可证字符串
    let mut encryption_key = 0x787u16;
    let encrypted_code =
        encrypt_decrypt_bytes(&mut encryption_key, license_string.as_bytes(), true);

    // 使用变体Base64编码
    let license_code = variant_base64_encode(&encrypted_code);

    println!("    ✅ 许可证代码构建完成");
    Ok(license_code)
}

/// 加密并生成MobaXterm许可证文件
///
/// # 参数
/// * `config` - MobaXterm配置信息
///
/// # 返回值
/// * `Ok(())` - 加密成功
/// * `Err(anyhow::Error)` - 加密失败
pub fn entry(config: &MobaXterm) -> Result<()> {
    println!("  🔐 开始许可证加密流程...");

    // 构建许可证代码
    let license_code = build_license_code(config).with_context(|| "许可证代码构建失败")?;

    // 创建ZIP文件
    build_zip(&license_code).with_context(|| "许可证文件创建失败")?;

    println!("  ✅ 许可证加密完成");
    Ok(())
}
