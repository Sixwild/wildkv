use crate::setup::*;
use wildkv::config::WildKvConfig;

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
    // wait_for_signal();

    wildkv.stop();
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

    fn run_server(&mut self, server_config: ServerConfig) {
        // let server = self.servers.take().unwrap();
    }

    fn stop(self) {}
}

struct Server {}

struct ServerConfig {
    cluster_id: i64,
}
