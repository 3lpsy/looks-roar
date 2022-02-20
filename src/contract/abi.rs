use super::queries::{ERC1155Query, ERC165Query, ERC721EnumerableQuery};
use crate::contract::types::Iface;
use crate::utils::AppError;
use ethers::core::types::Address;
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
    ) -> Result<HashMap<Address, Vec<Iface>>, Box<dyn Error>> {
        let mut erc721enums: Vec<Address> = vec![];
        let mut erc1155s: Vec<Address> = vec![];
        for (address, ifaces) in addfaces.iter() {
            if ifaces.contains(&Iface::ERC721Enumerable) {
                erc721enums.push(*address);
            } else if ifaces.contains(&Iface::ERC1155) {
                erc1155s.push(*address);
            }
            //other things
        }
        let erc721_tokens =
            ERC721EnumerableQuery::get_tokens_for_addresses(&erc721enums, provider.clone()).await?;
        let erc1155_tokens =
            ERC1155Query::get_tokens_for_addresses(&erc1155s, provider.clone()).await?;

        unimplemented!()
    }
}
