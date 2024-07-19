use std::future::Future;

use reqwest::{header::{HeaderMap, HeaderValue}, Client, ClientBuilder, Method, RequestBuilder};

/// For retrieving data from the Mojang API.
///
/// # Usage
///
/// Construct an instance using ['MojangApi::new(bearer_token)'](MojangApi::new).
/// The parameter should be a bearer token obtained from an endpoint. More information at
/// [unofficieal mojang docs](https://mojang-api-docs.gapple.pw/authentication/)
///
/// An instance provides access to calling methods
/// FIX: add example of endpoint here
pub struct MojangApi {
    /// Reqwest CLient
    client: Client
}

impl MojangApi {
    ///Constructs a new instance 
    pub fn new(bearer_token: impl AsRef<[u8]>) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "Authorization",
            HeaderValue::from_bytes(bearer_token.as_ref()).unwrap()
        );

        let client = ClientBuilder::new().default_headers(default_headers);

        Self {
            client: client.build().unwrap()
        }
    }

}
