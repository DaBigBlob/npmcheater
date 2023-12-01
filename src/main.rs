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
    // #[arg(short, long)]
    // silent: Option<bool>,

    #[arg(short, long)]
    packages: Option<Vec<String>>,

    #[arg(short, long = "max-sleep-mili")]
    max_sleep_mili: Option<u64>,

    #[arg(short, long = "user-agent")]
    user_agent: Option<String>
}


fn main() {
    let args = ClapCli::parse();
    //let pkgs = [String::from("libsql-stateless"), String::from("libsql-stateless-easy")];

    //user agent
    let user_agent = match args.user_agent {
        Some(ua) => ua,
        None => String::from("npm@10.2.0/node@v21.1.0+arm64 (darwin)")
    };

    //pkgs
    let pkgs = match args.packages {
        Some(p) => p,
        None => [String::from("libsql-stateless"), String::from("libsql-stateless-easy")].to_vec()
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
    
    loop {
        // fetch_pkg(&npm_client, pkgs[0]);
        // fetch_pkg(&npm_client, pkgs[1]);
        pkgs.iter(|pkg| {fetch_pkg(&npm_client, pkg)});

        wait_rand_sec(3560);
    }
}
