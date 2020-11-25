//! Zkms
#![deny(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
use rand::Rng;
use std::sync::Arc;

use futures::{channel::oneshot, future::FutureExt};
use tokio::runtime::Runtime;
use tokio::time;

use zkms_common::{HandleRequest, KeystoreRequest, KeystoreResponse, RequestMethod};

/// Starts the jsonrpc server
/// # Arguments
///
/// * `handler` - An optional handler that implements the HandleRequest trait
/// If it is None a default handler is used.
pub fn start_service(handler: impl HandleRequest + 'static) {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();
    let mut rng = rand::thread_rng();

    let handler = Arc::new(handler);

    let run_server = async {
        let mut interval = time::interval(time::Duration::from_secs(2));

        for i in 0..10u8 {
            let value: u64 = rng.gen_range(10, 999);
            let (request_sender, receiver) = oneshot::channel::<KeystoreResponse>();

            let request = if i % 2 == 0 {
                RequestMethod::Inc(value)
            } else {
                RequestMethod::Dec(value)
            };

            let request = KeystoreRequest {
                sender: request_sender,
                method: request,
            };

            handler
                .process_request(request)
                .expect("This Should not fail");

            let result = receiver
                .map(|e| match e {
                    Ok(KeystoreResponse::Dec(value)) => value,
                    _ => 0,
                })
                .await;
            println!("CLIENT => {}\n TA => {}", value, result);
            interval.tick().await;
        }
    };

    // Execute the future, blocking the current thread until completion
    rt.block_on(run_server);
}
