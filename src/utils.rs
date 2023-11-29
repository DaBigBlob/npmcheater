use reqwest::{self, blocking::Client, StatusCode};
use serde_json::Value;

pub fn get_tarball_url(client: &Client , pkg_name: &str) -> Result<String, String> {
    let reg = match client
    .get("https://registry.npmjs.org/".to_owned()+pkg_name)
    .send() {
        Ok(res) => match res.json::<Value>() {
            Ok(json) => json,
            Err(_) => return Err(String::from("JSON conversion failed"))
        },
        Err(_) => return Err(String::from("could not fetch tarball url"))
    };

    match reg.get("versions") {
        Some(v) => match v.as_object() {
            Some(obj) => match obj.values().last() {
                Some(lst) => match lst.get("dist") {
                    Some(dist) => match dist.get("tarball") {
                        Some(tar) => match tar.as_str() {
                            Some(str) => Ok(String::from(str)),
                            None => return Err(String::from("tarball url not string"))
                        },
                        None => return Err(String::from("no tarball in dist"))
                    },
                    None => return Err(String::from("no dist in latest pkg"))
                },
                None => return Err(String::from("could not get latest from packages"))
            },
            None => return Err(String::from("could not resolve JSON as object"))
        },
        None => return Err(String::from("no versions in JSON"))
    }
}

pub fn get_tarball(client: &Client , tarball_url: String) -> Result<(), String> {
    match client
    .get(tarball_url.as_str())
    .send() {
        Ok(res) => match res.status() {
            StatusCode::OK => return Ok(()),
            _ =>  return Err(res.status().to_string())
        },
        Err(e) => return Err(e.to_string())
    }
}