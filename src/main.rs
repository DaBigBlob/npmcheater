mod utils;
mod sleep;
mod logs;

use reqwest;
use utils::fetch_pkg;
use sleep::wait_rand_sec;

fn main() {
    let npm_client = match reqwest::blocking::Client::builder()
    .user_agent("npm@10.2.0/node@v21.1.0+arm64 (darwin)")
    .brotli(true)
    .deflate(true)
    .gzip(true)
    .build() {
        Ok(c) => c,
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        }
    };

    let pkgs = ["libsql-stateless", "libsql-stateless-easy"];
    
    loop {
        fetch_pkg(&npm_client, pkgs[0]);
        fetch_pkg(&npm_client, pkgs[1]);

        wait_rand_sec(3560);
    }
}
