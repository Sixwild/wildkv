use clap::{crate_authors, App, Arg};
use std::path::Path;
use wildkv::config::WildKvConfig;

fn main() {
    let matches = App::new("wildKV")
        .about("A distributed transactional key-value database powered by Rust and Redis")
        // 获取作者，但不知道怎么用
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .short("C")
                .long("config")
                .value_name("FILE")
                .help("Set the configuration file")
                .takes_value(true),
        )
        .get_matches();
    let config = matches
        .value_of("config")
        .map_or_else(WildKvConfig::default, |path| {
            let path = Path::new(path);
            WildKvConfig::from_file(path).unwrap_or_else(|e| {
                panic!(
                    "invalid auto generated configuration file {}, err {}",
                    path.display(),
                    e
                );
            })
        });
    server::server::run_wildkv(config);
}
