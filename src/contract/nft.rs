use crate::contract::abi;
use crate::db::{Db, NftEntry};
// use crate::utils::AppError;
use ethers::core::types::Address;
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
                let (entries, absent) = dbi.get_addresses_or_absent(&addresses);

                for (address, entry) in Self::build_entries(absent, provider).await? {
                    dbi.save_address(&address, &entry);
                }
                unimplemented!()
            }
            (Some(dbi), true) => {
                for entry in Self::build_entries(addresses, provider).await? {
                    dbi.save_address(&entry.0, &entry.1);
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
    ) -> Result<Vec<(Address, NftEntry)>, Box<dyn Error>> {
        let ifaces = abi::NFTAbi::guess_nft_ifaces(addresses, provider).await?;
        unimplemented!();
    }
}
