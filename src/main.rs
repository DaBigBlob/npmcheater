mod utils;
mod sleep;
mod logs;

use reqwest;
use utils::{get_tarball, get_tarball_url};
use sleep::wait_rand_sec;
use logs::{log_ok, LogRequestKing, log_err};

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
    
    loop {
        match get_tarball_url(&npm_client, "libsql-stateless") {
            Ok(tar) => match get_tarball(&npm_client, tar) {
                Ok(_) => log_ok(LogRequestKing::TarAndTarUrl, "(libsql-stateless)"),
                Err(er) => log_err(LogRequestKing::Tar, "(libsql-stateless)", er)
            },
            Err(er) => log_err(LogRequestKing::TarUrl, "(libsql-stateless)", er)
        };

        match get_tarball_url(&npm_client, "libsql-stateless-easy") {
            Ok(tar) => match get_tarball(&npm_client, tar) {
                Ok(_) => log_ok(LogRequestKing::TarAndTarUrl, "(libsql-stateless-easy)"),
                Err(er) => log_err(LogRequestKing::Tar, "(libsql-stateless-easy)", er)
            },
            Err(er) => log_err(LogRequestKing::TarUrl, "(libsql-stateless-easy)", er)
        };

        wait_rand_sec(3560);
    }
}
