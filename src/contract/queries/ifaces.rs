use crate::contract::abis::ERC165;
use crate::contract::types::Iface;
use crate::utils::AppError;
use ethers::abi::Tokenizable;
use ethers::core::types::Address;
use ethers::prelude::Multicall;
use ethers::providers::Middleware;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub struct IfacesQuery;

impl IfacesQuery {
    pub async fn get_ifaces_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        ifaces: Vec<Iface>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Iface>>, Box<dyn Error>> {
        println!("Making multicall");
        let (multi, mut queries) = Self::build_multi(addresses, ifaces, provider).await;

        // queries will hold index to zip against response

        let mut data: HashMap<Address, Vec<Iface>> = HashMap::new();

        // TODO: does it err if 165 is unsupported?
        match multi.call_raw().await {
            Ok(result) => result
                .iter()
                .zip(queries)
                .map(|(token, query)| {
                    // detokenize
                    let status = bool::from_token(token.to_owned()).unwrap();
                    // populate for address
                    data.entry(query.0).or_insert_with(Vec::new);
                    // append iface if status is true
                    if status {
                        let entry = data.get_mut(&query.0).unwrap();
                        entry.push(query.1)
                    }
                })
                .collect(),
            // TODO: find failure and retry?
            Err(_e) => return Err(AppError::boxed("Multicall failed".to_string(), 0)),
        };
        Ok(data)
    }
    pub async fn build_multi<M: Middleware>(
        addresses: Vec<Address>,
        ifaces: Vec<Iface>,
        provider: Arc<M>,
    ) -> (Multicall<M>, Vec<(Address, Iface)>) {
        let m = Multicall::new(provider.clone(), None).await.unwrap();
        let mut queries: Vec<(Address, Iface)> = vec![];
        let multi = addresses
            .iter()
            .map(|a| {
                //...
                ERC165::new(a.to_owned(), provider.clone())
            })
            .fold(m, |mut multi_carry, contract| {
                for iface in &ifaces[..] {
                    queries.push((contract.address().clone(), iface.clone()));
                    multi_carry.add_call(contract.supports_interface(iface.id()));
                }
                multi_carry
            });
        (multi, queries)
    }
}
