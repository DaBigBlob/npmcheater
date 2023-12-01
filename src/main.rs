mod utils;
mod sleep;
mod logs;

use reqwest;
use clap::Parser;
use utils::fetch_pkg;
use sleep::wait_rand_sec;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClapCli {
    #[arg(short, long)]
    silent: Option<bool>,

    #[arg(short, long)]
    packages: Option<Vec<String>>,

    #[arg(short, long = "max-sleep-mili")]
    max_sleep_mili: Option<u64>,

    #[arg(short, long = "user-agent")]
    user_agent: Option<String>
}


fn main() {
    let args = ClapCli::parse();

    //user agent
    let user_agent = match args.user_agent {
        Some(ua) => ua,
        None => String::from("npm@10.2.0/node@v21.1.0+arm64 (darwin)")
    };

    let npm_client = match reqwest::blocking::Client::builder()
    .user_agent(user_agent)
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
