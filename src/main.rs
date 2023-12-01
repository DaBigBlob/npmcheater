mod utils;
mod sleep;
mod logs;
mod headers;

use logs::log_info;
use reqwest;
use clap::Parser;
use utils::fetch_pkg;
use sleep::wait_rand_sec;
use headers::create_npm_headers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClapCli {
    #[arg(short, long = "show-logs")]
    show_logs: Option<bool>,

    #[arg(short, long)]
    packages: Option<Vec<String>>,

    #[arg(short, long = "max-sleep-mili")]
    max_sleep_mili: Option<u64>
}

fn main() {
    let args = ClapCli::parse();
    //let pkgs = [String::from("libsql-stateless"), String::from("libsql-stateless-easy")];

    //logs
    let logging = match args.show_logs {
        Some(l) => l,
        None => true
    };
    log_info("Logging set to: ".to_owned()+&logging.to_string());

    //pkgs
    let pkgs = match args.packages {
        Some(p) => p,
        None => [String::from("libsql-stateless"), String::from("libsql-stateless-easy")].to_vec()
    };
    log_info("Running for packages: ".to_owned()+&pkgs.join(", "));

    //max deplay
    let max_delay = match args.max_sleep_mili {
        Some(d) => match d {
            0 => 1,
            dd => dd
        },
        None => 3560
    };
    log_info("Max delay set to: ".to_owned()+&max_delay.to_string());

    let npm_client = match reqwest::blocking::Client::builder()
    .default_headers(create_npm_headers())
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
        pkgs.iter().for_each(|pkg| {fetch_pkg(&npm_client, pkg, logging)});

        wait_rand_sec(max_delay, logging);
    }
}
