use serde::{Deserialize, Serialize};

use crate::contract;
use crate::contract::types::{NFTIface, NFTOptIface};
use ethers::core::types::Address;

#[derive(Serialize, Deserialize)]
pub struct AddressCache {
    pub iface: contract::types::NFTIface,
    pub opt_ifaces: Vec<contract::types::NFTOptIface>,
}

impl AddressCache {
    pub fn new(iface: NFTIface, opt_ifaces: Vec<NFTOptIface>) -> Self {
        Self { iface, opt_ifaces }
    }
}
