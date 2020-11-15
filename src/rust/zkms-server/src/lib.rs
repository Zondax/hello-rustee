//! Zkms

#![deny(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use jsonrpc_core::IoHandler;

use tokio::runtime::Runtime;
use tokio::task;

use jsonrpc_tcp_server::ServerBuilder;

use zkms_common::HandlerRequest;

mod wasm_executor;

pub mod config;
pub(crate) mod local_handler;

pub mod server;
pub mod types;
pub use config::ZkmsConfig;
pub(crate) use server::ServerHandler;
pub(crate) use types::RemoteSignerApi;
pub(crate) use wasm_executor::Executor;

pub(crate) const WASM_BINARY: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/wasm_runtime.wasm"));

/// Starts the jsonrpc server
/// # Arguments
///
/// * `handler` - An optional handler that implements the HandlerRequest trait
/// If it is None a default handler is used.
pub fn start_server(handler: impl HandlerRequest + 'static) {
    // TODO: later we may want to get the config settings from outside
    let config = ZkmsConfig::default();

    // Create the runtime
    let mut rt = Runtime::new().unwrap();

    let run_server = async {
        let server_ip = &config.start.addr;
        let server_port = &config.start.port;

        let server_addr = format!("{}:{}", server_ip, server_port);
        println!("conntection server to: {}", server_addr);

        let server = ServerHandler::new(handler);

        let mut io = IoHandler::new();

        io.extend_with(server.to_delegate());

        let server = ServerBuilder::new(io)
            .start(&server_addr.parse().unwrap())
            .expect("Unable to start RPC server");

        // This presents a problem y the executor is turned off, it will wait
        // until this task ends,
        task::spawn_blocking(|| {
            server.wait();
        })
        .await
        .unwrap();
    };

    // Execute the future, blocking the current thread until completion
    rt.block_on(run_server);
}