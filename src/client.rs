use super::Mirror;
use reqwest::{Client, Proxy, Result};

pub fn new(proxy: &Proxy) -> Result<Client> {
    Client::builder()
        .proxy(proxy.clone())
        .timeout(std::time::Duration::from_secs(20))
        .build()
}
