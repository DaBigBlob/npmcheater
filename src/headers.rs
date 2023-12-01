use reqwest::header::HeaderMap;

// "accept": "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
// "accept-encoding": "gzip",
// "connection": "Keep-Alive",
// "host": "registry.npmjs.org",
// "npm-in-ci": "false",
// "npm-scope": "",
// "npm-session": "911b7880c91cb9f1",
// "pacote-pkg-id": "registry:libsql-stateless-easy",
// "pacote-req-type": "packument",
// "referer": "install libsql-stateless-easy",
// "user-agent": "npm/6.14.15 node/v12.22.9 linux x64",
// "x-forwarded-proto": "https",
// "x-real-ip": "34.83.150.28"

pub fn create_npm_headers() -> HeaderMap{
    let mut headers = HeaderMap::new();

    headers.insert("accept", "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*".parse().unwrap());
    headers.insert("accept-encoding", "gzip".parse().unwrap());
    headers.insert("connection", "Keep-Alive".parse().unwrap());
    headers.insert("host", "registry.npmjs.org".parse().unwrap());
    headers.insert("npm-in-ci", "false".parse().unwrap());
    headers.insert("npm-scope", "".parse().unwrap());
    headers.insert("npm-session", "911b7880c91cb9f1".parse().unwrap());
    headers.insert("pacote-pkg-id", "registry:libsql-stateless-easy".parse().unwrap());
    headers.insert("pacote-req-type", "packument".parse().unwrap());
    headers.insert("referer", "install libsql-stateless-easy".parse().unwrap());
    headers.insert("user-agent", "npm/6.14.15 node/v12.22.9 linux x64".parse().unwrap());
    headers.insert("x-forwarded-proto", "https".parse().unwrap());
    headers.insert("x-real-ip", "34.83.150.28".parse().unwrap());
    
    return headers;
}