use reqwest::{header::HeaderMap, blocking::Client};
use crate::fetcher;
use crate::headers;

pub struct Elm {
    pkg: String,
    tar_url: HeaderMap,
    tar_ball: HeaderMap
}
impl Elm {
    pub fn pkg(&self) -> &String{
        &self.pkg
    }
    pub fn tar_url(&self) -> HeaderMap{
        self.tar_url.clone()
    }
    pub fn tar_ball(&self) -> HeaderMap{
        self.tar_ball.clone()
    }
}
pub fn init(pkgs: &Vec<String>, client: &Client) -> Vec<Elm>{
    pkgs.iter().map(|pkg| {
        let dist = fetcher::try_till_get_tar_dist(pkg, client);
        Elm {
            pkg: pkg.to_string(),
            tar_url: headers::tar_url(pkg),
            tar_ball: headers::tar_ball(dist.url(), dist.hash())
        }
    }).collect()
}