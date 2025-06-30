//! JetBrains产品代码管理模块
//!
//! 负责获取和管理JetBrains产品和插件的代码信息

use super::constant::{CODE_FILE_PATH, HTTP_CLIENT, PLUGIN_API_BASE, PRODUCT_API};
use anyhow::{Context, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;

/// JetBrains产品信息结构
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInfo {
    /// 产品代码
    pub(crate) code: String,
}

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

/// 加载JetBrains产品代码
///
/// 从JetBrains官方API获取所有产品的代码列表
///
/// # 返回值
/// * `Ok(String)` - 以逗号分隔的产品代码字符串
/// * `Err(CodeError)` - 获取失败
async fn load_product() -> Result<String, CodeError> {
    println!("    🔍 正在获取JetBrains产品代码...");

    let url = format!("{PRODUCT_API}?fields=code");
    let products: Vec<ProductInfo> = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .context("请求产品API失败")?
        .json()
        .await
        .context("解析产品数据失败")?;

    let product_codes: Vec<String> = products.into_iter().map(|p| p.code).collect();

    println!("    ✅ 获取到 {} 个产品代码", product_codes.len());
    Ok(product_codes.join(","))
}

/// 加载JetBrains插件代码
///
/// 并发获取付费和免费插件的产品代码
///
/// # 返回值
/// * `Ok(String)` - 以逗号分隔的插件产品代码字符串
/// * `Err(CodeError)` - 获取失败
async fn load_plugin() -> Result<String, CodeError> {
    println!("    🔍 正在获取JetBrains插件代码...");

    // 并发获取付费和免费插件列表
    let (paid_plugins, freemium_plugins) = tokio::try_join!(
        tokio::spawn(fetch_plugins("PAID")),
        tokio::spawn(fetch_plugins("FREEMIUM"))
    )?;

    // 合并插件列表
    let all_plugins = paid_plugins?
        .into_iter()
        .chain(freemium_plugins?.into_iter())
        .collect::<Vec<_>>();

    println!(
        "    📦 找到 {} 个插件，正在获取详细信息...",
        all_plugins.len()
    );

    // 并发获取插件详情，限制并发数为15
    let plugin_codes = futures::stream::iter(all_plugins)
        .filter_map(|plugin| async move { plugin["id"].as_i64().map(|id| id.to_string()) })
        .map(fetch_plugin_details)
        .buffer_unordered(15)
        .filter_map(|result| async move {
            match result {
                Ok(detail) => Some(detail.purchase_info.product_code),
                Err(_) => {
                    // 静默处理单个插件获取失败，避免输出过多错误信息
                    None
                }
            }
        })
        .collect::<Vec<_>>()
        .await;

    println!("    ✅ 成功获取 {} 个插件产品代码", plugin_codes.len());
    Ok(plugin_codes.join(","))
}

/// 获取指定定价模式的插件列表
///
/// # 参数
/// * `pricing_model` - 定价模式 ("PAID" 或 "FREEMIUM")
///
/// # 返回值
/// * `Ok(Vec<Value>)` - 插件列表
/// * `Err(CodeError)` - 获取失败
async fn fetch_plugins(pricing_model: &str) -> Result<Vec<Value>, CodeError> {
    let url =
        format!("{PLUGIN_API_BASE}/searchPlugins?max=10000&offset=0&pricingModels={pricing_model}");

    let response = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .context("请求插件列表失败")?;

    let data: Value = response.json().await.context("解析插件列表响应失败")?;

    data["plugins"]
        .as_array()
        .ok_or_else(|| CodeError::ApiError("插件列表格式无效".to_string()))
        .map(|arr| arr.to_vec())
}

/// 插件详情结构
#[derive(Deserialize)]
struct PluginDetail {
    #[serde(rename = "purchaseInfo")]
    purchase_info: PurchaseInfo,
}

/// 购买信息结构
#[derive(Deserialize)]
struct PurchaseInfo {
    #[serde(rename = "productCode")]
    product_code: String,
}

/// 获取插件详细信息
///
/// # 参数
/// * `id` - 插件ID
///
/// # 返回值
/// * `Ok(PluginDetail)` - 插件详情
/// * `Err(CodeError)` - 获取失败
async fn fetch_plugin_details(id: String) -> Result<PluginDetail, CodeError> {
    let url = format!("{PLUGIN_API_BASE}/plugins/{id}");

    let response = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .context("请求插件详情失败")?;

    let text = response.text().await.context("读取插件详情响应失败")?;

    let detail: PluginDetail = serde_json::from_str(&text).context("解析插件详情失败")?;

    Ok(detail)
}

/// 更新产品代码文件
///
/// 从JetBrains官方API获取最新的产品和插件代码，并保存到本地文件
///
/// # 返回值
/// * `Ok(())` - 更新成功
/// * `Err(CodeError)` - 更新失败
pub async fn update_code() -> Result<(), CodeError> {
    println!("  🔄 开始更新产品代码...");

    // 并发获取产品代码和插件代码
    let (product_code, plugin_code) = tokio::try_join!(load_product(), load_plugin())?;

    // 合并代码
    let combined_code = format!("{product_code},{plugin_code}");

    // 保存到文件
    println!("  💾 正在保存产品代码到文件...");
    let mut file = File::create(CODE_FILE_PATH).context("创建产品代码文件失败")?;

    file.write_all(combined_code.as_bytes())
        .context("写入产品代码失败")?;

    file.flush().context("刷新文件缓冲区失败")?;

    println!("  ✅ 产品代码更新完成，保存至: {}", CODE_FILE_PATH);
    Ok(())
}

/// 从文件读取产品代码
///
/// # 返回值
/// * `Ok(String)` - 产品代码字符串
/// * `Err(CodeError)` - 读取失败
pub fn get_code() -> Result<String, CodeError> {
    let code = fs::read_to_string(CODE_FILE_PATH).context("读取产品代码文件失败")?;

    if code.trim().is_empty() {
        return Err(CodeError::ApiError(
            "产品代码文件为空，请先运行更新命令".to_string(),
        ));
    }

    Ok(code.trim().to_string())
}
