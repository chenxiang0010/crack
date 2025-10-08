use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) const VARIANT_BASE64_TABLE: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

lazy_static! {
    pub(crate) static ref VARIANT_BASE64_DICT: HashMap<usize, char> =
        VARIANT_BASE64_TABLE.chars().enumerate().collect();
}
