extern crate lazy_static;

lazy_static! {
    // HTTP client to share
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}
