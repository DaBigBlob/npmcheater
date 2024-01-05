
mod cli;
mod queue;
mod fetcher;
mod headers;
mod logger;
mod sleep;

fn main() {
    let args = cli::Args::parse();
    let client = &fetcher::client();
    let queue = queue::init(args.packages(), client);

    loop {
        queue.iter().for_each(|elm| {
            match fetcher::TarDist::get(&elm.pkg(), &client, elm.tar_url()) {
                Ok(d) => {
                    log::info!("url fetched for {}", elm.pkg());
                    match fetcher::TarBall::get(d.url(), &client, elm.tar_ball()) {
                        Ok(_) => log::info!("tarball fetched for {}", elm.pkg()),
                        Err(_) => log::warn!("tarball for {} could not be fetched", elm.pkg())
                    };
                },
                Err(_) => log::warn!("url for {} could not be fetched", elm.pkg())
            };

        });

        sleep::wait_rand_sec(args.delay());
    }
}
