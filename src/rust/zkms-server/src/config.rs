//! Zkms Config

use serde::{Deserialize, Serialize};

/// Zkms Configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ZkmsConfig {
    /// Server section
    pub start: StartSection,
}

/// Server configurations
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StartSection {
    /// Connection address
    pub addr: String,
    /// Port number for stablishing a connection
    pub port: u16,
}

impl Default for StartSection {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".into(),
            port: 3000,
        }
    }
}
