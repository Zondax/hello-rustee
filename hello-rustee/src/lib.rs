use zondee::{Uuid, StackStr};

pub const UUID: Uuid = Uuid {
    time_low: 2179707489,
    time_mid: 45827,
    time_hi_and_version: 19152,
    clock_seq_and_node: [186, 244, 215, 41, 130, 228, 122, 139],
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Input {
    HelloFromRee(StackStr<[u8; 64]>),
    Version,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Output {
    HelloFromTee(StackStr<[u8; 64]>),
    Version(u32),
}
