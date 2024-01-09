use log::debug;
use reqwest::{blocking::Client, StatusCode, header::HeaderMap};
use serde_json::Value;

pub fn client() -> reqwest::blocking::Client {
    let client = match reqwest::blocking::Client::builder()
    .deflate(true)
    .gzip(true)
    .brotli(true)
    .build() {
        Ok(c) => c,
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        }
    };
    client
}

pub struct TarDist {
    url: String,
    hash: String
}
impl TarDist {
    pub fn get(pkg: &str, client: &Client, headers: HeaderMap) -> Result<TarDist, String> {
        let reg = match client
        .get({
            let url = "https://registry.npmjs.org/".to_owned()+pkg;
            debug!("url is {}", url);
            url
        })
        .headers(headers)
        .send() {
            Ok(res) => match res.json::<Value>() {
                Ok(json) => json,
                Err(_) => return Err(String::from("JSON conversion failed"))
            },
            Err(_) => return Err(String::from("could not fetch tarball url"))
        };

        let dist = match reg.get("versions") {
            Some(v) => match v.as_object() {
                Some(obj) => match obj.values().last() {
                    Some(lst) => match lst.get("dist") {
                        Some(dist) => dist,
                        None => return Err(String::from("no dist in latest pkg"))
                    },
                    None => return Err(String::from("could not get latest from packages"))
                },
                None => return Err(String::from("could not resolve JSON as object"))
            },
            None => return Err(String::from("no versions in JSON"))
        };

        let url = match dist.get("tarball") {
            Some(tar) => match tar.as_str() {
                Some(str) => String::from(str),
                None => return Err(String::from("tarball url not string"))
            },
            None => return Err(String::from("no tarball in dist"))
        };

        let hash = match dist.get("integrity") {
            Some(tar) => match tar.as_str() {
                Some(str) => String::from(str),
                None => return Err(String::from("tarball url not string"))
            },
            None => return Err(String::from("no tarball in dist"))
        };


        Ok(TarDist {
            url,
            hash
        })
    }

    pub fn url(&self) -> &str{
        &self.url
    }

    pub fn hash(&self) -> &str{
        &self.hash
    }
}
pub fn try_till_get_tar_dist(pkg: &str, client: &Client) -> TarDist {
    match TarDist::get(pkg, client, crate::headers::tar_url(pkg)) {
        Ok(td) => {
            debug!("setting hash for {}: {}", pkg, td.hash());
            debug!("setting url for {}: {}", pkg, td.url());
            td
        },
        Err(_) => try_till_get_tar_dist(pkg, client)
    }
}

pub struct TarBall {}
impl TarBall {
    pub fn get(url: &str, client: &Client, headers: HeaderMap) -> Result<(), String>{
        match client
        .get(url)
        .headers(headers)
        .send() {
            Ok(res) => match res.status() {
                StatusCode::OK => return Ok(()),
                _ =>  return Err(res.status().to_string())
            },
            Err(e) => return Err(e.to_string())
        }
    }
}