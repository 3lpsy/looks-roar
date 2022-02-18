use serde::{Deserialize, Serialize};

use crate::contract;
use crate::contract::types::Iface;

#[derive(Serialize, Deserialize)]
pub struct NftEntry {
    pub ifaces: Vec<contract::types::Iface>,
}

impl NftEntry {
    pub fn new(ifaces: Vec<Iface>) -> Self {
        Self { ifaces }
    }
}
