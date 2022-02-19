use ethers::abi::Uint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::contract;
use crate::contract::types::Iface;
#[derive(Serialize, Deserialize)]
pub struct TokenEntry;

#[derive(Serialize, Deserialize)]
pub struct NftEntry {
    pub ifaces: Vec<contract::types::Iface>,
    pub tokens: HashMap<Uint, TokenEntry>,
}

impl NftEntry {
    pub fn new(ifaces: Vec<Iface>) -> Self {
        let tokens = HashMap::new();
        Self { ifaces, tokens }
    }
}
