use crate::utils::AppError;
use ethers::abi::Tokenizable;
use ethers::abi::Uint;
use ethers::core::types::Address;
use ethers::prelude::{Middleware, Multicall};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::contract::abis::ERC1155;
pub struct ERC1155Query;

impl ERC1155Query {
    pub async fn get_tokens_for_addresses<M: Middleware>(
        addresses: &Vec<Address>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Uint>>, Box<dyn Error>> {
        unimplemented!()
    }
}
