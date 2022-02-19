use crate::utils::AppError;
use ethers::abi::Tokenizable;
use ethers::abi::Uint;
use ethers::core::types::Address;
use ethers::prelude::{Middleware, Multicall};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::contract::abis::ERC721Enumerable;
pub struct ERC721EnumerableQuery;

impl ERC721EnumerableQuery {
    pub async fn get_tokens_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Uint>>, Box<dyn Error>> {
        let total_supplies =
            Self::get_total_supply_for_addresses(addresses, provider.clone()).await?;

        unimplemented!()
    }
    pub async fn build_multi_token_by_index_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        total_supplies: Vec<Uint>,
        provider: Arc<M>,
    ) -> Multicall<M> {
        let m = Multicall::new(provider.clone(), None).await.unwrap();
        addresses
            .iter()
            .map(|a| {
                //...
                ERC721Enumerable::new(a.to_owned(), provider.clone())
            })
            .zip(total_supplies.iter())
            .fold(m, |mut multi_carry, (contract, total_supply)| {
                for i in 0..=total_supply.as_u64() {
                    multi_carry.add_call(contract.token_by_index(Uint::from(i)));
                }
                multi_carry
            })
    }

    pub async fn get_total_supply_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<Vec<Uint>, Box<dyn Error>> {
        let multi = Self::build_multi_total_supply_for_addresses(addresses, provider).await;
        match multi.call_raw().await {
            Ok(results) => {
                // TODO: why iterator issues with Uint/can't use .map?
                let mut supplies: Vec<Uint> = vec![];
                for result in results {
                    let val = Uint::from_token(result.to_owned()).unwrap();
                    supplies.push(val)
                }
                Ok(supplies)
            }
            Err(_e) => return Err(AppError::boxed("Multicall failed".to_string(), 0)),
        }
    }

    pub async fn build_multi_total_supply_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Multicall<M> {
        let m = Multicall::new(provider.clone(), None).await.unwrap();
        addresses
            .iter()
            .map(|a| {
                //...
                ERC721Enumerable::new(a.to_owned(), provider.clone())
            })
            .fold(m, |mut multi_carry, contract| {
                multi_carry.add_call(contract.total_supply());
                multi_carry
            })
    }
}
