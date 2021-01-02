//use futures::channel::oneshot;

/// Request handler interface
pub trait HandleRequest: Send + Sync {
    /// process a request
    //fn process_request(&self, request: KeystoreRequest) -> Result<(), String>;
    fn process_request(&self, request: RequestMethod) -> Result<RequestResponse, String>;
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum RequestMethod {
    Inc(u64),
    Dec(u64),
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum RequestResponse {
    Inc(u64),
    Dec(u64),
}
