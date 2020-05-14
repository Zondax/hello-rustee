use heapless::consts::U256;
use jsonrpc_core::{IoHandler, Params, Value};
use remote_signer::{Input, Output, UUID};
use warp::Filter;
use zondee::to_hex;
use zondee_teec::framework::send_msg;

#[tokio::main]
async fn main() {
    let mut io = IoHandler::<()>::default();
    io.add_method("sign", |params: Params| {
        let bytes: heapless::String<U256> = params.parse().unwrap();
        Ok(Value::String(sign(bytes.as_str()).unwrap()))
    });
    let sign = warp::post()
        .and(warp::path("sign"))
        .and(warp::path::param())
        .map(move |bytes: String| io.handle_request_sync(&bytes).unwrap());
    warp::serve(sign).run(([127, 0, 0, 1], 3030)).await;
}

fn sign(bytes_str: &str) -> zondee_teec::Result<String> {
    let input = Input::Sign({
        let mut sv = heapless::Vec::default();
        sv.extend(bytes_str.as_bytes().iter().copied());
        sv
    });
    let output: Output = send_msg(UUID.into(), input)?;
    match output {
        Output::Sign(bytes) => Ok(to_hex(&bytes).unwrap().as_str().into()),
    }
}
