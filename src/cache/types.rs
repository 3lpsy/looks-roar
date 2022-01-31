use serde::{Deserialize, Serialize};

use crate::contract;

#[derive(Serialize, Deserialize)]
pub struct AddressCache {
    pub iface: contract::types::NFTIface,
    pub opt_ifaces: Vec<contract::types::NFTOptIface>,
}
