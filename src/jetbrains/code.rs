//! JetBrains产品代码管理模块
//!
//! 负责获取和管理JetBrains产品和插件的代码信息

use anyhow::{Context, Result};
use std::fs::{self};

use super::constant::CODE_FILE_PATH;

/// 代码获取相关错误类型
#[derive(Debug, thiserror::Error)]
pub enum CodeError {
    #[error("🌐 获取产品数据失败: {0}")]
    ProductFetch(#[from] reqwest::Error),
    #[error("📄 解析插件数据失败: {0}")]
    PluginParse(#[from] serde_json::Error),
    #[error("💾 文件操作失败: {0}")]
    Io(#[from] std::io::Error),
    #[error("🔌 插件API错误: {0}")]
    ApiError(String),
    #[error("⚡ 任务执行失败: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("🔧 通用错误: {0}")]
    General(#[from] anyhow::Error),
}

/// 从文件读取产品代码
///
/// # 返回值
/// * `Ok(String)` - 产品代码字符串
/// * `Err(CodeError)` - 读取失败
pub fn get_code() -> Result<String, CodeError> {
    let code = fs::read_to_string(CODE_FILE_PATH).with_context(|| "读取产品代码文件失败")?;

    if code.trim().is_empty() {
        return Err(CodeError::ApiError(
            "产品代码文件为空，请先运行更新命令".to_string(),
        ));
    }

    Ok(code.trim().to_string())
}
