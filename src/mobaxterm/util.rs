use serde::{Deserialize, Serialize};
use std::fmt;

pub(crate) fn encrypt_decrypt_bytes(key: &mut u16, bytes: &[u8], encrypt: bool) -> Vec<u8> {
    bytes
        .iter()
        .map(|&byte| {
            let result = byte ^ ((*key >> 8) as u8);
            *key = if encrypt {
                (result as u16 & *key) | 0x482D
            } else {
                (byte as u16 & *key) | 0x482D
            };
            result
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LicenseType {
    Professional,
    Educational,
    Personal,
}

impl LicenseType {
    pub fn to_id(&self) -> u8 {
        match self {
            LicenseType::Professional => 1,
            LicenseType::Educational => 3,
            LicenseType::Personal => 4,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LicenseType::Professional => "Professional",
            LicenseType::Educational => "Educational",
            LicenseType::Personal => "Personal",
        }
    }
}

impl fmt::Display for LicenseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
