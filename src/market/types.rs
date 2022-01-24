use crate::market::constants;

#[derive(Clone, Debug)]
pub enum Endpoint {
    Mainnet,
    Rinkeby,
}

impl Endpoint {
    pub fn url(&self) -> &str {
        match self {
            Endpoint::Mainnet => constants::API_BASE_MAINNET,
            Endpoint::Rinkeby => constants::API_BASE_RINKEBY,
        }
    }
}
