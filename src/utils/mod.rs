//! 通用工具模块
//!
//! 提供应用程序初始化和通用工具函数

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// 输出目录列表
const OUTPUT_DIRS: &[&str] = &[
    "output/jetbrains",
    "output/jetbrains/cert",
    "output/mobaxterm",
];

/// 默认配置文件内容
const DEFAULT_CONFIG_CONTENT: &str = r#"{
  "mobaxterm": {
    "username": "Developer",
    "version": "23.1",
    "licenseType": "Professional",
    "count": 1
  },
  "jetbrains": {
    "licenseeName": "Developer",
    "assigneeName": "Developer",
    "expireAt": "2029-12-31",
    "updateCode": false
  }
}"#;

/// 初始化应用程序环境
///
/// 创建必要的输出目录和默认配置文件
///
/// # 返回值
/// * `Ok(())` - 初始化成功
/// * `Err(anyhow::Error)` - 初始化失败
pub fn init() -> Result<()> {
    println!("🔧 正在初始化应用程序环境...");

    init_output_directories().context("创建输出目录失败")?;

    init_config_file().context("初始化配置文件失败")?;

    println!("✅ 环境初始化完成");
    Ok(())
}

/// 初始化输出目录
///
/// 创建所有必要的输出目录
fn init_output_directories() -> Result<()> {
    for dir_path in OUTPUT_DIRS {
        create_directory_if_not_exists(dir_path)
            .with_context(|| format!("创建目录 '{}' 失败", dir_path))?;
    }
    Ok(())
}

/// 创建目录（如果不存在）
///
/// # 参数
/// * `path` - 目录路径
fn create_directory_if_not_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path).with_context(|| format!("无法创建目录: {}", path.display()))?;
        println!("📁 创建目录: {}", path.display());
    }
    Ok(())
}

/// 初始化配置文件
///
/// 如果配置文件不存在，则创建默认配置文件
fn init_config_file() -> Result<()> {
    let config_file = Path::new("config.json");

    if !config_file.exists() {
        fs::write(config_file, DEFAULT_CONFIG_CONTENT).context("无法写入默认配置文件")?;

        println!("📄 创建默认配置文件: {}", config_file.display());
        println!("💡 请根据需要修改配置文件中的参数");
    }

    Ok(())
}
