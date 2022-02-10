use crate::contract::{abi, types};
use crate::db::{Db, NftEntry};
use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::prelude::U256;
use ethers::providers::Middleware;
use std::error::Error;
use std::sync::Arc;

// what is M
// TODO: wrap cache is db wrapper
pub struct NFT<M> {
    imp: abi::NFTAbi<M>,
    db: Option<Db>,
    nfts: Vec<NftEntry>,
}

impl<M: Middleware> NFT<M> {
    pub async fn build(
        addresses: Vec<Address>,
        provider: Arc<M>,
        db: Option<Db>,
        fresh: bool,
    ) -> Result<Self, Box<dyn Error>> {
        match (db, fresh) {
            // db is in use and we are not refreshing
            (Some(dbi), false) => {
                let (mut entries, mut absent) = dbi.get_addresses_or_absent(&addresses);
                for entry in Self::build_entries(absent, provider).await? {
                    dbi.save_address(entry.0, entry.1);
                }
                unimplemented!()
            }
            (Some(dbi), true) => {
                for entry in Self::build_entries(addresses, provider).await? {
                    dbi.save_address(entry.0, entry.1);
                }
                unimplemented!()
            }
            (None, _) => {
                let entries = Self::build_entries(addresses, provider).await?;
                unimplemented!();
            }
        }
    }
    async fn build_entries(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<(Address, Vec<NftEntry>), Box<dyn Error>> {
        let ifaces = abi::NFTAbi::guess_nft_ifaces(addresses, provider).await?;
        unimplemented!();
    }

    pub async fn enumerate(&self) -> Result<Vec<U256>, Box<dyn Error>> {
        // TODO: check cache
        match self.imp.fetch_tokens().await {
            Ok(tokens) => {
                //..
                Ok(tokens)
            }
            Err(e) => Err(e),
        }
    }
}
