use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

macro_rules! headermap {
    ($($key: expr => $val: expr ), *) => {{
        let mut headers = HeaderMap::new();
        $( headers.insert(HeaderName::from_static($key), HeaderValue::from_static($val)); )*
        headers
    }}
}
//not the best way; i couldnt be bothered
macro_rules! shn { //static header value
    ($val: expr) => {
        HeaderName::from_static($val)
    }
}
macro_rules! shv { //static header value
    ($val: expr) => {
        HeaderValue::from_static($val)
    }
}

fn default_headers() -> HeaderMap{
    headermap![
        "accept-encoding" => "gzip",
        "connection" => "Keep-Alive",
        "host" => "registry.npmjs.org",
        "npm-auth-type" => "web",
        "npm-command" => "install",
        "pacote-version" => "15.1.3",
        "user-agent" => "npm/9.6.7 node/v20.3.1 linux x64 workspaces/false"
    ]
}

pub fn tar_url(pkg: &str) -> HeaderMap{
    let mut hdr = default_headers();
    hdr.insert(shn!("accept"), shv!("application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*"));
    hdr.insert(
        shn!("pacote-pkg-id"),
        HeaderValue::from_str(("registry:".to_owned()+pkg).as_str())
        .unwrap_or(shv!("registry:libsql-stateless"))
    );
    hdr.insert(shn!("pacote-req-type"), shv!("packument"));
    hdr
}

pub fn tar_ball(url: &str, hash: &str) -> HeaderMap{
    let mut hdr = default_headers();
    hdr.insert(shn!("accept"), shv!("*/*"));
    hdr.insert(
        shn!("pacote-integrity"),
        HeaderValue::from_str(hash)
        .unwrap_or(shv!("sha512-yd0f+NptVDhwY8Yqu0rRc3embIHQQLbJdPIhAAEnGi6JKO6M9RO3dL6Ke/0pUgZujOndk6vrrlbIN8ErayrzlA=="))
    );
    hdr.insert(
        shn!("pacote-pkg-id"),
        HeaderValue::from_str(("remote:libsql-stateless@".to_owned()+url).as_str())
        .unwrap_or(shv!("remote:libsql-stateless@https://registry.npmjs.org/libsql-stateless/-/libsql-stateless-2.7.4.tgz"))
    );
    hdr.insert(shn!("pacote-req-type"), shv!("tarball"));
    hdr
}