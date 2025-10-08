//! JetBrains许可证生成模块
//!
//! 负责生成JetBrains系列IDE的许可证文件和相关证书

mod cert_generator;
mod code;
mod constant;
mod inject;
mod product_license_generator;
mod xyzr;

use crate::config::JetBrains;
use anyhow::{Context, Result};
use product_license_generator::LicenseInfoReq;
use std::fs;

/// 生成JetBrains许可证文件
///
/// # 参数
/// * `config` - JetBrains配置信息
///
/// # 返回值
/// * `Ok(())` - 许可证生成成功
/// * `Err(anyhow::Error)` - 许可证生成失败
async fn generate_license(config: &JetBrains) -> Result<()> {
    println!("  📝 正在生成许可证文件...");

    let product_code = code::get_code().with_context(|| "获取产品代码失败")?;

    let license_info = LicenseInfoReq {
        licensee_name: config.licensee_name.clone(),
        assignee_name: config.assignee_name.clone(),
        expire_at: config.expire_at.clone(),
        product_code,
    };

    let license_code = product_license_generator::generate_license_code(license_info)
        .with_context(|| "生成许可证代码失败")?;

    fs::write(constant::LICENSE_FILE_PATH, license_code).with_context(|| "写入许可证文件失败")?;

    println!("  ✅ 许可证文件生成完成: {}", constant::LICENSE_FILE_PATH);
    Ok(())
}

/// 运行JetBrains许可证生成流程
///
/// # 参数
/// * `config` - JetBrains配置信息
///
/// # 返回值
/// * `Ok(())` - 生成流程执行成功
/// * `Err(anyhow::Error)` - 生成流程执行失败
pub async fn run(config: &JetBrains) -> Result<()> {
    println!("🚀 开始JetBrains许可证生成流程");

    // 更新产品代码（如果需要）
    if config.update_code {
        println!("  🔄 正在更新产品代码...");
        code::update_code()
            .await
            .with_context(|| "更新产品代码失败")?;
        println!("  ✅ 产品代码更新完成");
    } else {
        println!("  ℹ️  跳过产品代码更新");
    }

    // 注入证书和配置
    println!("  🔐 正在生成证书和配置文件...");
    inject::inject().with_context(|| "证书和配置注入失败")?;
    println!("  ✅ 证书和配置文件生成完成");

    // 生成许可证
    generate_license(config)
        .await
        .with_context(|| "许可证生成失败")?;

    println!("\n🎉 JetBrains许可证生成完成！");
    println!("📋 请将以下文件复制到您的IDE安装目录：");
    println!("   • license.txt - 许可证文件");
    println!("   • power.conf - 配置文件");

    Ok(())
}
