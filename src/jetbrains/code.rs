use super::constant::{CODE_FILE_PATH, HTTP_CLIENT, PLUGIN_API_BASE, PRODUCT_API};
use anyhow::Result;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInfo {
    pub(crate) code: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeError {
    #[error("Failed to fetch product data: {0}")]
    ProductFetch(#[from] reqwest::Error),
    #[error("Failed to parse plugin data: {0}")]
    PluginParse(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Plugin API error: {0}")]
    ApiError(String),
    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

async fn load_product() -> Result<String, CodeError> {
    let url = format!("{}?fields=code", PRODUCT_API);
    let resp: Vec<String> = HTTP_CLIENT
        .get(&url)
        .send()
        .await?
        .json::<Vec<ProductInfo>>()
        .await?
        .iter()
        .map(|p| p.code.clone())
        .collect();
    Ok(resp.join(","))
}

async fn load_plugin() -> Result<String, CodeError> {
    let (paid, freemium) = tokio::try_join!(
        tokio::spawn(fetch_plugins("PAID")),
        tokio::spawn(fetch_plugins("FREEMIUM"))
    )?;
    let paid_stream = futures::stream::iter(paid?.into_iter());
    let freemium_stream = futures::stream::iter(freemium?.into_iter());
    let plugins_stream = paid_stream.chain(freemium_stream);
    let codes = plugins_stream
        .filter_map(|plugin| async move { plugin["id"].as_i64().map(|id| id.to_string()) })
        .map(fetch_plugin_details)
        .buffer_unordered(15)
        .filter_map(|result| async move {
            match result {
                Ok(detail) => Some(detail.purchase_info.product_code),
                Err(e) => {
                    eprintln!("获取插件详情失败: {:?}", e);
                    None
                }
            }
        })
        .collect::<Vec<_>>()
        .await;
    Ok(codes.join(","))
}

async fn fetch_plugins(pricing_model: &str) -> Result<Vec<Value>, CodeError> {
    let url = format!(
        "{}/searchPlugins?max=10000&offset=0&pricingModels={}",
        PLUGIN_API_BASE, pricing_model
    );
    let data: Value = HTTP_CLIENT.get(&url).send().await?.json().await?;
    data["plugins"]
        .as_array()
        .ok_or_else(|| CodeError::ApiError("Invalid plugin list format".to_string()))
        .map(|arr| arr.to_vec())
}

#[derive(Deserialize)]
struct PluginDetail {
    #[serde(rename = "purchaseInfo")]
    purchase_info: PurchaseInfo,
}

#[derive(Deserialize)]
struct PurchaseInfo {
    #[serde(rename = "productCode")]
    product_code: String,
}

async fn fetch_plugin_details(id: String) -> Result<PluginDetail, CodeError> {
    let url = format!("{}/plugins/{}", PLUGIN_API_BASE, id);
    let res = HTTP_CLIENT.get(&url).send().await?;
    let detail: PluginDetail = serde_json::from_str(&res.text().await?)?;
    Ok(detail)
}

pub async fn update_code() -> Result<(), CodeError> {
    let product_code = load_product().await?;
    let plugin_code = load_plugin().await?;
    let code = format!("{},{}", product_code, plugin_code);
    let mut file = File::create(CODE_FILE_PATH)?;
    file.write_all(code.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn get_code() -> Result<String, CodeError> {
    let code = fs::read_to_string(CODE_FILE_PATH)?;
    Ok(code)
}
