//! Server module
use jsonrpc_core::BoxFuture;

use futures::{
    channel::oneshot,
    future::{FutureExt, TryFutureExt},
};

use crate::types::RemoteSignerApi;
use zkms_common::{HandleRequest, KeystoreRequest, KeystoreResponse, RequestMethod};

/// Handler for requests
pub struct ServerHandler<Handler> {
    handler: Handler,
}

impl<Handler: HandleRequest + Default + 'static> Default for ServerHandler<Handler> {
    fn default() -> ServerHandler<Handler> {
        let handler = Handler::default();
        Self { handler }
    }
}

impl<Handler: HandleRequest + 'static> ServerHandler<Handler> {
    /// Creates a new server handler instance
    /// # Arguments
    ///
    /// * `store` - Any type that implements the HandleRequest trait
    pub fn new(store: Handler) -> ServerHandler<Handler> {
        Self { handler: store }
    }

    // TODO: A real implementation might require this function to be async
    // It would depend on the inner TEE interface and other interfaces at hand at that moment
    fn process_request(&self, request: RequestMethod) -> oneshot::Receiver<KeystoreResponse> {
        let (request_sender, receiver) = oneshot::channel::<KeystoreResponse>();

        let request = KeystoreRequest {
            sender: request_sender,
            method: request,
        };

        self.handler
            .process_request(request)
            .expect("This Should not fail");

        receiver
    }
}

impl<Handler: HandleRequest + 'static> RemoteSignerApi for ServerHandler<Handler> {
    fn encode(&self, msg: String) -> BoxFuture<String> {
        let receiver = self.process_request(RequestMethod::Encode(msg));
        Box::new(
            receiver
                .map(|e| match e {
                    Ok(KeystoreResponse::Encode(msg)) => Ok(msg),
                    _ => Ok(String::new()),
                })
                .boxed()
                .compat(),
        )
    }

    fn decode(&self, msg: String) -> BoxFuture<String> {
        let receiver = self.process_request(RequestMethod::Decode(msg));
        Box::new(
            receiver
                .map(|e| match e {
                    Ok(KeystoreResponse::Decode(msg)) => Ok(msg),
                    _ => Ok(String::new()),
                })
                .boxed()
                .compat(),
        )
    }
}

#[cfg(test)]
mod tests {
    use jsonrpc_test;
    use serde_json;
    use tokio;

    use super::*;
    use crate::types::RemoteSignerApi;

    #[derive(Default)]
    struct TestHandler {}

    impl HandleRequest for TestHandler {
        fn process_request(&self, request: KeystoreRequest) -> Result<(), String> {
            let sender = request.sender;
            match request.method {
                RequestMethod::Encode(msg) => {
                    let _ = sender.send(KeystoreResponse::Encode(msg));
                }
                RequestMethod::Decode(msg) => {
                    let _ = sender.send(KeystoreResponse::Decode(msg));
                }
            }
            Ok(())
        }
    }

    async fn setup() -> jsonrpc_test::Rpc {
        let server = ServerHandler::new(TestHandler::default());
        jsonrpc_test::Rpc::new(RemoteSignerApi::to_delegate(server))
    }

    #[tokio::test(core_threads = 4)]
    async fn test_keys() {
        let rpc = setup().await;

        let test_str = "Alice".to_string();

        let r = rpc.request("encode", &[test_str.clone()]);

        let res: String = serde_json::from_str(&r).unwrap();

        assert_eq!(res.len(), test_str.len());
    }
}
