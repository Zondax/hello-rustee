use remote_signer::{Input, Output, UUID};
use zondee::{
    framework::{self, Client},
    wrapper, StackVec, to_hex
};
use warp::Filter;

// Theorically, these parameters shouldn't be needed by the host. Maybe a linking problem?
#[no_mangle]
pub static trace_ext_prefix: &[u8] = b"TA\0";
#[no_mangle]
pub static trace_level: libc::c_int = 0;

#[tokio::main]
async fn main() {
    let sign = warp::path!("sign" / String)
        .map(|bytes| {
            let ctx = Default::default();
            let uuid = wrapper::client::Uuid::from_os_uuid(UUID);
            let mut client = Client::new(ctx, "HOST", Default::default()).unwrap();
            client.open_session(uuid, &mut Default::default()).unwrap();
            sign(&mut client, bytes).unwrap()
        });

    warp::serve(sign)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn sign(client: &mut Client, bytes_str: String) -> framework::Result<String> {
    let input = Input::Sign({
        let mut sv = StackVec::default();
        sv.extend(bytes_str.as_bytes().iter().copied());
        sv
    });
    let rslt: Output = client.invoke_command(input)?;
    match rslt {
        Output::Sign(bytes) => Ok(to_hex(&bytes).unwrap().as_str().into())
    }
}
