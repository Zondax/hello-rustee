// https://github.com/paritytech/substrate/blob/ben-remote-signer/client/jsonrpc-remote-signer/src/lib.rs

use serde;
use std::mem;

use sp_keystore::vrf::{VRFTranscriptData, VRFTranscriptValue};

fn make_static_u8<'a>(x: &'a [u8]) -> &'static [u8] {
    unsafe { mem::transmute(x) }
}

fn make_static_str<'a>(x: &'a str) -> &'static str {
    unsafe { mem::transmute(x) }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransferableVRFTranscriptData {
    /// The transcript's label
    pub label: Vec<u8>,
    /// Additional data to be registered into the transcript
    pub items: Vec<(String, VRFTranscriptValue)>,
}

impl From<TransferableVRFTranscriptData> for VRFTranscriptData {
    fn from(t: TransferableVRFTranscriptData) -> VRFTranscriptData {
        VRFTranscriptData {
            label: make_static_u8(&t.label),
            items: t
                .items
                .into_iter()
                .map(|(s, v)| (make_static_str(&s), v))
                .collect(),
        }
    }
}

impl From<VRFTranscriptData> for TransferableVRFTranscriptData {
    fn from(d: VRFTranscriptData) -> TransferableVRFTranscriptData {
        TransferableVRFTranscriptData {
            label: d.label.to_vec(),
            items: d
                .items
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }
}
