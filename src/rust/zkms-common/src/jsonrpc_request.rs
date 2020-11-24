use futures::channel::oneshot;

/// Request handler interface
pub trait HandleRequest: Send + Sync {
    /// process a request
    fn process_request(&self, request: KeystoreRequest) -> Result<(), String>;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum RequestMethod {
    Inc(u64),
    Dec(u64),
}

pub struct KeystoreRequest {
    pub sender: oneshot::Sender<KeystoreResponse>,
    pub method: RequestMethod,
}

pub enum KeystoreResponse {
    Inc(u64),
    Dec(u64),
}
