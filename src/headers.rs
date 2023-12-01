use reqwest::header::{HeaderMap, HOST, USER_AGENT, ACCEPT, CONNECTION};

// user-agent: npm/10.2.3 node/v21.2.0 darwin arm64 workspaces/false\r\n
// pacote-version: 17.0.4\r\n
// pacote-req-type: packument\r\n
// pacote-pkg-id: registry:libsql-stateless\r\n
// accept: application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*\r\n
// npm-auth-type: web\r\n
// npm-command: install\r\n
// authorization: Bearer npm_tD8urGUsampleJmisamplepH6SgsamplevEa\r\n
// Accept-Encoding: gzip,deflate\r\n
// Host: registry.npmjs.org\r\n
// connection: keep-alive\r\n
// \r\n

pub fn create_npm_headers() -> HeaderMap{
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, "npm/10.2.3 node/v21.2.0 darwin arm64 workspaces/false".parse().unwrap());
    headers.insert("pacote-version", "17.0.4".parse().unwrap());
    headers.insert("pacote-req-type", "packument".parse().unwrap());
    headers.insert(ACCEPT, "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*".parse().unwrap());
    headers.insert("npm-auth-type", "web".parse().unwrap());
    headers.insert("npm-command", "install".parse().unwrap());
    headers.insert(HOST, "registry.npmjs.org".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());

    return headers;
}