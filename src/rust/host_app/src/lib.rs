//! Zkms
#![deny(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
use rand::Rng;
use zkms_common::{HandleRequest, RequestMethod, RequestResponse};

pub fn start_service(handler: impl HandleRequest + 'static) {
    let mut rng = rand::thread_rng();
    for _ in 1..=10u8 {
        let value: u64 = rng.gen_range(1, 999);

        let request = if value % 2 == 0 {
            RequestMethod::Inc(value)
        } else {
            RequestMethod::Dec(value)
        };

        let response = handler
            .process_request(request)
            .expect("This Should not fail");

        let result = match response {
            RequestResponse::Inc(value) => value,
            RequestResponse::Dec(value) => value,
        };
        println!("CLIENT => {} - TA => {}", value, result);
    }
}
