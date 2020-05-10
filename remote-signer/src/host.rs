use jsonrpc_core::{IoHandler, Params, Value};
use remote_signer::{Input, Output, UUID};
use warp::Filter;
use zondee::{to_hex, StackStr, StackVec};
use zondee_teec::framework::Client;

#[tokio::main]
async fn main() {
    let mut io = IoHandler::<()>::default();
    io.add_method("sign", |params: Params| {
        let bytes: StackStr<[u8; 256]> = params.parse().unwrap();
        let ctx = Default::default();
        let mut client = Client::new(ctx, "HOST", Default::default()).unwrap();
        client
            .open_session(UUID.into(), &mut Default::default())
            .unwrap();
        Ok(Value::String(sign(&mut client, bytes.as_str()).unwrap()))
    });
    let sign = warp::post()
        .and(warp::path("sign"))
        .and(warp::path::param())
        .map(move |bytes: String| io.handle_request_sync(&bytes).unwrap());
    warp::serve(sign).run(([127, 0, 0, 1], 3030)).await;
}

fn sign(client: &mut Client, bytes_str: &str) -> zondee_teec::Result<String> {
    let input = Input::Sign({
        let mut sv = StackVec::default();
        sv.extend(bytes_str.as_bytes().iter().copied());
        sv
    });
    let rslt: Output = client.invoke_command(input)?;
    match rslt {
        Output::Sign(bytes) => Ok(to_hex(&bytes).unwrap().as_str().into()),
    }
}
