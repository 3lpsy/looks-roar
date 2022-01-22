use crate::market::constants;
use crate::market::types::Network;
use ethers::abi::Address;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error;

pub struct OpenSeaApi {
    http: Client,
    network: Network,
}

impl OpenSeaApi {
    pub fn new(network: Network) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Origin", HeaderValue::from_static("https://looksrare.org"));
        headers.insert("Referer", HeaderValue::from_static("https://looksrare.org"));
        headers.insert(
            "Cache-Control",
            HeaderValue::from_static("no-cache, must-revalidate"),
        );
        headers.insert("Pragma", HeaderValue::from_static("no-cache"));
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("en-US,en;q=0.5"),
        );

        let builder = Client::builder()
            .gzip(true)
            .user_agent(constants::USER_AGENT_EDGE94)
            .default_headers(headers)
            .http2_prior_knowledge();

        // api key

        let client = builder.build().unwrap();
        Self {
            http: client,
            network,
        }
    }
    pub fn get_floor(&self, contract: Address, limit: u8) -> Result<(), Box<dyn error::Error>> {
        // let request = OrderRequest::default();
        // let map = request.to_query();
        Ok(())
    }
}

pub struct OrderRequest {
    asset_contract_address: Address,
    side: u8,
}
