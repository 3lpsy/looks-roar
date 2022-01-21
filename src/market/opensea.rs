use crate::market::types::Network;
use reqwest::Client;

pub struct OpenSeaApi {
    client: Client,
    network: Network,
}
