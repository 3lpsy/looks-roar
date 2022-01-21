use crate::market::constants;

#[derive(Clone, Debug)]
pub enum Network {
    Mainnet,
    Rinkeby,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => constants::API_BASE_MAINNET,
            Network::Rinkeby => constants::API_BASE_RINKEBY,
        }
    }
}
