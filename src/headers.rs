use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

// [COMMON]
static H_accept_encoding: &str = "gzip";
static H_connection: &str = "Keep-Alive";
static H_host: &str = "registry.npmjs.org";
static H_npm_auth_type: &str = "web";
static H_npm_command: &str = "install";
static H_pacote_version: &str = "15.1.3";
static H_user_agent: &str = "npm/9.6.7 node/v20.3.1 linux x64 workspaces/false";



pub fn tar_url_headers(pkg_name: &str) -> HeaderMap{
    let mut headers = HeaderMap::new();

    headers.insert(
        HeaderName::from_static("accept"),
        HeaderValue::from_static("application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*")
    );
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*"));
    headers.insert(HeaderName::from_static("accept-encoding"), HeaderValue::from_static("gzip"));
    headers.insert(HeaderName::from_static("connection"), HeaderValue::from_static("Keep-Alive"));
    headers.insert(HeaderName::from_static("host"), HeaderValue::from_static("registry.npmjs.org"));
    headers.insert(HeaderName::from_static("npm-auth-type"), HeaderValue::from_static("web"));
    headers.insert(HeaderName::from_static("npm-command"), HeaderValue::from_static("install"));
    headers.insert(HeaderName::from_static("pacote-req-type"), HeaderValue::from_static("packument"));
    headers.insert(HeaderName::from_static("pacote-version"), HeaderValue::from_static("15.1.3"));
    headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("npm/9.6.7 node/v20.3.1 linux x64 workspaces/false"));

    
    return headers;
}