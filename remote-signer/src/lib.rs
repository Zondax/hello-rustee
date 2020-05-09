use schnorrkel::SIGNATURE_LENGTH;
use zondee::{wrapper::os, StackVec};

pub const UUID: os::Uuid = os::Uuid {
    timeLow: 2179707489,
    timeMid: 45827,
    timeHiAndVersion: 19152,
    clockSeqAndNode: [186, 244, 215, 41, 130, 228, 122, 139],
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Input {
    Sign(StackVec<[u8; 128]>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Output {
    Sign(StackVec<[u8; SIGNATURE_LENGTH]>),
}
