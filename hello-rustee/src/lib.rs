use zondee::{wrapper::os, StackStr};

pub const UUID: os::Uuid = os::Uuid {
    timeLow: 2179707489,
    timeMid: 45827,
    timeHiAndVersion: 19152,
    clockSeqAndNode: [186, 244, 215, 41, 130, 228, 122, 139],
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
