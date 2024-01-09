
use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClapCli {
    #[arg(short = 's', long = "silent")]
    silent: bool,

    #[arg(short = 'p', long)]
    packages: Option<Vec<String>>,

    #[arg(short = 'm', long = "max-sleep-mili")]
    delay: Option<u64>
}

pub struct Args {
    packages: Vec<String>,
    delay: u64
}

impl Args {
    pub fn parse() -> Args{
        let _args = ClapCli::parse();

        //logs
        let logging = !_args.silent;
        if logging {crate::logger::init(logging)};

        //pkgs
        let packages = match _args.packages {
            Some(p) => {
                log::info!("running for packages: {}", p.join(", "));
                p
            },
            None => {
                let pkgs = [String::from("libsql-stateless"), String::from("libsql-stateless-easy")].to_vec();
                log::warn!("--packages not set; using default packages: {}", pkgs.join(", "));
                pkgs
            }
        };

        //max deplay
        let delay = match _args.delay {
            Some(d) => match d {
                0 => {
                    log::warn!("0ms being rounded to 1ms for max delay");
                    1
                },
                dd => {
                    log::info!("max delay set to {}ms", dd);
                    dd
                }
            },
            None => {
                log::warn!("--max-sleep-mili not set; using default max sleep of 3560ms");
                6000
            }
        };

        Args {
            packages,
            delay
        }
    }

    pub fn packages(&self) -> &Vec<String>{
        &self.packages
    }
    pub fn delay(&self) -> u64{
        self.delay
    }
}