use super::queries::{ERC1155Query, ERC165Query, ERC721EnumerableQuery};
use crate::contract::types::Iface;
use crate::utils::AppError;
use ethers::abi::Uint;
use ethers::core::types::Address;
use ethers::prelude::H160;
use ethers::providers::Middleware;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub struct NFTAbi;

impl NFTAbi {
    pub async fn get_nft_ifaces_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Iface>>, Box<dyn Error>> {
        let responses =
            ERC165Query::get_ifaces_for_addresses(addresses, Iface::all(), provider).await?;
        let mut missing: Vec<Address> = vec![];

        // loop over map
        for (addr, ifaces) in &responses {
            if ifaces.is_empty() {
                missing.push(*addr)
            }
        }
        if !missing.is_empty() {
            return Err(AppError::boxed(
                format!("Found unsupported addresses for ifaces: {:?}", missing),
                0,
            ));
        }
        Ok(responses)
    }
    pub async fn get_nft_tokens_for_addresses<M: Middleware>(
        addfaces: HashMap<Address, Vec<Iface>>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Uint>>, Box<dyn Error>> {
        // prepare hashmap to contain enumerated results
        let mut itokens: HashMap<[u8; 4], Vec<Address>> = HashMap::new();
        for iface in Iface::all() {
            itokens.insert(iface.id(), vec![]);
        }
        // enumerate results
        for (address, ifaces) in addfaces.iter() {
            if ifaces.contains(&Iface::ERC721Enumerable) {
                let entry = itokens.get_mut(&Iface::ERC721Enumerable.id()).unwrap();
                entry.push(*address);
            } else if ifaces.contains(&Iface::ERC1155) {
                let entry = itokens.get_mut(&Iface::ERC1155.id()).unwrap();
                entry.push(*address);
            }
            //other things
        }
        let mut results: HashMap<H160, Vec<Uint>> = HashMap::new();

        for (k, v) in ERC721EnumerableQuery::get_tokens_for_addresses(
            itokens.get(&Iface::ERC721Enumerable.id()).unwrap(),
            provider.clone(),
        )
        .await?
        {
            results.insert(k, v);
        }

        for (k, v) in ERC1155Query::get_tokens_for_addresses(
            itokens.get(&Iface::ERC1155.id()).unwrap(),
            provider.clone(),
        )
        .await?
        {
            results.insert(k, v);
        }
        dbg!(results);
        unimplemented!()
    }
}
