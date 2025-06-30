//! 不可描述的功能 - 软件许可证生成工具
//!
//! 这是一个用于生成MobaXterm和JetBrains软件许可证的命令行工具。
//!
//! # 功能模块
//! - `mobaxterm`: MobaXterm许可证生成
//! - `jetbrains`: JetBrains IDE许可证和证书生成
//! - `config`: 配置文件管理
//! - `utils`: 通用工具函数

mod cli;
mod config;
mod jetbrains;
mod mobaxterm;
mod utils;

use config::Config;
use std::process;

/// 程序错误退出处理函数
///
/// # 参数
/// * `error` - 实现了Display trait的错误类型
fn exit_with_error(error: impl std::fmt::Display) -> ! {
    eprintln!("❌ 错误: {error}");
    process::exit(1);
}

/// 程序主入口点
///
/// 初始化环境、加载配置并运行CLI命令
#[tokio::main]
async fn main() {
    // 初始化输出目录和配置文件
    if let Err(e) = utils::init() {
        exit_with_error(format!("初始化失败: {e}"));
    }

    // 加载配置文件
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => exit_with_error(format!("配置加载失败: {e}")),
    };

    // 运行CLI命令
    if let Err(e) = cli::run(&config).await {
        exit_with_error(format!("命令执行失败: {e}"));
    }
}
