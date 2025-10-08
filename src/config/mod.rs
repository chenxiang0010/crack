//! 配置管理模块
//!
//! 负责加载和验证应用程序配置文件

use crate::mobaxterm::util::LicenseType;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 应用程序主配置结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// MobaXterm相关配置
    pub mobaxterm: MobaXterm,
    /// JetBrains相关配置
    pub jetbrains: JetBrains,
}

/// MobaXterm配置结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobaXterm {
    /// 用户名
    pub username: String,
    /// 软件版本号
    pub version: String,
    /// 许可证类型
    pub license_type: LicenseType,
    /// 许可证数量
    pub count: usize,
}

/// JetBrains配置结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JetBrains {
    /// 许可证持有人姓名
    pub licensee_name: String,
    /// 被授权人姓名
    pub assignee_name: String,
    /// 过期时间 (格式: YYYY-MM-DD)
    pub expire_at: String,
}

impl Config {
    /// 配置文件路径
    const CONFIG_FILE_PATH: &'static str = "config.json";

    /// 从配置文件加载配置
    ///
    /// # 返回值
    /// * `Ok(Config)` - 配置加载成功
    /// * `Err(anyhow::Error)` - 配置加载失败
    pub fn new() -> Result<Self> {
        let config_path = Path::new(Self::CONFIG_FILE_PATH);

        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "配置文件 '{}' 不存在，请先运行程序进行初始化",
                Self::CONFIG_FILE_PATH
            ));
        }

        let config_content = fs::read_to_string(config_path)
            .with_context(|| format!("无法读取配置文件 '{}'", Self::CONFIG_FILE_PATH))?;

        let config: Config = serde_json::from_str(&config_content)
            .with_context(|| format!("配置文件 '{}' 格式错误", Self::CONFIG_FILE_PATH))?;

        config.validate()?;
        Ok(config)
    }

    /// 验证配置的有效性
    ///
    /// # 返回值
    /// * `Ok(())` - 配置验证通过
    /// * `Err(anyhow::Error)` - 配置验证失败
    fn validate(&self) -> Result<()> {
        // 验证MobaXterm配置
        if self.mobaxterm.username.trim().is_empty() {
            return Err(anyhow::anyhow!("MobaXterm用户名不能为空"));
        }

        if self.mobaxterm.version.trim().is_empty() {
            return Err(anyhow::anyhow!("MobaXterm版本号不能为空"));
        }

        if self.mobaxterm.count == 0 {
            return Err(anyhow::anyhow!("MobaXterm许可证数量必须大于0"));
        }

        // 验证JetBrains配置
        if self.jetbrains.licensee_name.trim().is_empty() {
            return Err(anyhow::anyhow!("JetBrains许可证持有人姓名不能为空"));
        }

        if self.jetbrains.assignee_name.trim().is_empty() {
            return Err(anyhow::anyhow!("JetBrains被授权人姓名不能为空"));
        }

        if self.jetbrains.expire_at.trim().is_empty() {
            return Err(anyhow::anyhow!("JetBrains过期时间不能为空"));
        }

        // 验证日期格式 (简单验证)
        if !self.jetbrains.expire_at.matches('-').count() == 2 {
            return Err(anyhow::anyhow!(
                "JetBrains过期时间格式错误，应为 YYYY-MM-DD 格式"
            ));
        }

        Ok(())
    }
}
