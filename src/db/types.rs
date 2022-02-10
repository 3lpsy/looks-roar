use serde::{Deserialize, Serialize};

use crate::contract;
use crate::contract::types::NFTIface;

#[derive(Serialize, Deserialize)]
pub struct NftEntry {
    pub ifaces: Vec<contract::types::NFTIface>,
}

impl NftEntry {
    pub fn new(ifaces: Vec<NFTIface>) -> Self {
        Self { ifaces }
    }
}
