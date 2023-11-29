mod utils;
mod sleep;

use reqwest;
use utils::{get_tarball, get_tarball_url};
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
    
    loop {
        match get_tarball_url(&npm_client, "libsql-stateless") {
            Ok(tar) => match get_tarball(&npm_client, tar) {
                Ok(_) => println!("[OK] libsql-stateless"),
                Err(er) => println!("[ERR] [TAR] [libsql-stateless] {}", er)
            },
            Err(er) => {
                println!("[ERR] [TAR_URL] [libsql-stateless] {}", er);
                continue
            }
        };

        match get_tarball_url(&npm_client, "libsql-stateless-easy") {
            Ok(tar) => match get_tarball(&npm_client, tar) {
                Ok(_) => println!("[OK] libsql-stateless-easy"),
                Err(er) => println!("[ERR] [TAR] [libsql-stateless-easy]{}", er)
            },
            Err(er) => {
                println!("[ERR] [TAR_URL] [libsql-stateless-easy] {}", er);
                continue
            }
        };
        
        wait_rand_sec();
    }
}
