//! CLI命令行接口模块
//!
//! 提供命令行参数解析和命令执行功能

use crate::config::Config;
use crate::{jetbrains, mobaxterm};
use clap::{Parser, Subcommand};

/// CLI错误类型定义
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("🔧 MobaXterm处理失败: {0}")]
    Mobaxterm(anyhow::Error),
    #[error("💡 JetBrains处理失败: {0}")]
    Jetbrains(anyhow::Error),
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        // 根据错误信息判断来源，这里简化处理
        CliError::Jetbrains(err)
    }
}

/// 软件许可证生成工具
///
/// 支持生成MobaXterm和JetBrains系列软件的许可证文件
#[derive(Debug, Parser)]
#[command(
    name = "crack",
    about = "软件许可证生成工具",
    long_about = "一个用于生成MobaXterm和JetBrains软件许可证的命令行工具",
    disable_help_subcommand = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

/// 支持的命令类型
#[derive(Debug, Subcommand)]
pub enum Command {
    /// 生成MobaXterm专业版许可证
    ///
    /// 根据配置文件中的用户名、版本号等信息生成MobaXterm许可证文件
    #[command(name = "mobaxterm", alias = "moba")]
    Mobaxterm,

    /// 生成JetBrains系列IDE许可证
    ///
    /// 生成JetBrains全系列IDE的许可证文件和相关证书
    #[command(name = "jetbrains", alias = "jb")]
    Jetbrains,
}

/// 执行CLI命令
///
/// # 参数
/// * `config` - 应用程序配置
///
/// # 返回值
/// * `Ok(())` - 命令执行成功
/// * `Err(CliError)` - 命令执行失败
pub async fn run(config: &Config) -> Result<(), CliError> {
    let args = Cli::parse();

    println!("🚀 开始执行命令...\n");

    match args.command {
        Command::Mobaxterm => {
            println!("🔧 正在生成MobaXterm许可证...");
            mobaxterm::run(&config.mobaxterm).map_err(CliError::Mobaxterm)?;
        }
        Command::Jetbrains => {
            println!("💡 正在生成JetBrains许可证...");
            jetbrains::run(&config.jetbrains)
                .await
                .map_err(CliError::Jetbrains)?;
        }
    }

    println!("\n✅ 命令执行完成！");
    Ok(())
}
