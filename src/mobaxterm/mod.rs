//! MobaXterm许可证生成模块
//!
//! 负责生成MobaXterm专业版许可证文件

mod constant;
mod encrypt;
pub mod util;

use crate::config::MobaXterm;
use anyhow::{Context, Result};

/// 运行MobaXterm许可证生成流程
///
/// # 参数
/// * `config` - MobaXterm配置信息
///
/// # 返回值
/// * `Ok(())` - 许可证生成成功
/// * `Err(anyhow::Error)` - 许可证生成失败
pub fn run(config: &MobaXterm) -> Result<()> {
    println!("🚀 开始MobaXterm许可证生成流程");
    println!("  👤 用户名: {}", config.username);
    println!("  📦 版本: {}", config.version);
    println!("  🏷️  许可证类型: {:?}", config.license_type);
    println!("  🔢 许可证数量: {}", config.count);

    println!("  🔐 正在生成许可证文件...");
    encrypt::encrypt(config).context("MobaXterm许可证生成失败")?;

    println!("\n🎉 MobaXterm许可证生成完成！");
    println!("📋 请将 'Custom.mxtpro' 文件移动到MobaXterm安装目录");

    Ok(())
}
