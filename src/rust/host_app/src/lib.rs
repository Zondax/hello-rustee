//! Zkms
#![deny(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
use rand::Rng;
use zkms_common::{HandleRequest, KeystoreResponse, RequestMethod};

pub fn start_service(handler: impl HandleRequest + 'static) {
    let mut rng = rand::thread_rng();
    for _ in 0..10u8 {
        let value: u64 = rng.gen_range(10, 999);

        let request = if value % 2 == 0 {
            RequestMethod::Inc(value)
        } else {
            RequestMethod::Dec(value)
        };

        let response = handler
            .process_request(request)
            .expect("This Should not fail");

        let result = match response {
            KeystoreResponse::Inc(value) => value,
            KeystoreResponse::Dec(value) => value,
            _ => 0,
        };
        println!("CLIENT => {}\n TA => {}", value, result);
    }
}
