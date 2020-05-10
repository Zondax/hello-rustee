use remote_signer::{Input, UUID, Output};
use warp::Filter;
use zondee::{to_hex, StackVec};
use zondee_teec::{framework::Client};

#[tokio::main]
async fn main() {
    let sign = warp::path!("sign" / String).map(|bytes| {
        let ctx = Default::default();
        let mut client = Client::new(ctx, "HOST", Default::default()).unwrap();
        client.open_session(UUID.into(), &mut Default::default()).unwrap();
        sign(&mut client, bytes).unwrap()
    });

    warp::serve(sign).run(([127, 0, 0, 1], 3030)).await;
}

fn sign(client: &mut Client, bytes_str: String) -> zondee_teec::Result<String> {
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
