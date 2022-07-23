use std::task::Poll;
use tonic::{transport::Server as tonic_server, Request, Response, Status};

use signal::{trap::Trap, Signal::*};

use wildkvpb::wildkv_server::{Wildkv, WildkvServer};
use kvrpcpb::{RawGetRequest, RawGetResponse};

use crate::setup::*;
use wildkv::config::WildKvConfig;

pub mod wildkvpb {
    tonic::include_proto!("wildkvpb");
}

pub mod kvrpcpb {
    tonic::include_proto!("kvrpcpb");
}

pub fn run_wildkv(config: WildKvConfig) {
    println!("run wildkv with config {:?}...", config);

    init_logger(&config);

    pre_start();

    run_impl(config);
}

pub fn pre_start() {}

fn run_impl(config: WildKvConfig) {
    let mut wildkv = WildKvServer::init(config);
    let server_config = wildkv.init_server();
    wildkv.run_server(server_config);

    // TODO 监听信号
    wait_for_signal();

    wildkv.stop();
}


#[allow(dead_code)]
pub fn wait_for_signal() {
    let trap = Trap::trap(&[SIGTERM, SIGINT, SIGHUP, SIGUSR1, SIGUSR2]);
    for sig in trap {
        match sig {
            SIGTERM | SIGINT | SIGHUP => {
                break;
            }
            SIGUSR1 => {
                // Use SIGUSR1 to log metrics.
            }
            // TODO: handle more signal
            _ => unreachable!(),
        }
    }
}

struct WildKvServer {
    servers: Option<Server>,
}

impl WildKvServer {
    fn init(mut config: WildKvConfig) -> WildKvServer {
        Self { servers: None }
    }

    fn init_server(&mut self) -> ServerConfig {
        ServerConfig { cluster_id: 0 }
    }

    #[tokio::main]
    async fn run_server(&mut self, server_config: ServerConfig)
        -> Result<(), Box<dyn std::error::Error>> {
        let address = "[::1]:50052".parse().unwrap();
        let greeter = Server::default();

        tonic_server::builder()
            .add_service(WildkvServer::new(greeter))
            .serve(address)
            .await.unwrap();

        Ok(())
    }

    fn stop(self) {}
}

#[derive(Default)]
pub struct Server {}

#[tonic::async_trait]
impl Wildkv for Server {
    async fn raw_get(
        &self,
        request: Request<RawGetRequest>
    ) -> Result<Response<RawGetResponse>, Status> {
        let val = String::from("ok");
        let resp = RawGetResponse {
           value: val.into_bytes(),
            error: String::from(""),
            not_found: false,
        };
        let request_msg = request.into_inner();
        let ctx = request_msg.context.unwrap();
        println!("{}????", request_msg.cf);
        println!("{:?}????", request_msg.key);
        println!("{:?}????", ctx.api_version);
        Ok(Response::new(resp))
    }
}


struct ServerConfig {
    cluster_id: i64,
}
