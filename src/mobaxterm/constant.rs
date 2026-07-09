/// 变体 Base64 字符表
///
/// 索引 0..=63 对应编码字符；按 6 位分组编码时索引恒在此范围内，
/// 末尾的 '=' 填充符不会被按索引取用。
pub(crate) const VARIANT_BASE64_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
