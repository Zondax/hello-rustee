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

    let mut prev = 1u64;

    for _ in 1..=10u8 {
        let value: u64 = rng.gen_range(1, 999);

        let request = if value % 2 == 0 {
            RequestMethod::Inc(value)
        } else {
            RequestMethod::Dec(value)
        };

        let response = handler
            .process_request(request.clone())
            .expect("This Should not fail");

        let result = match response {
            RequestResponse::Inc(value) => value,
            RequestResponse::Dec(value) => value,
            _ => unreachable!("got mul out of inc and dec, TA misbehaving?"),
        };
        println!("CLIENT ({:?}) => {} ;; TA => {}", request, value, result);

        let request = RequestMethod::Mul(prev, value);

        let mul_response = handler
            .process_request(request.clone())
            .expect("shouldn't fail");

        let result = match mul_response {
            RequestResponse::Mul(val) => val,
            _ => unreachable!("got inc/dec out of mul, TA misbehaving?"),
        };
        prev = result;

        println!("CLIENT ({:?}) => {} ;; TA => {}", request, value, result);
    }
}
