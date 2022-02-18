use crate::contract::abi;
use crate::db::{Db, NftEntry};
// use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use std::error::Error;
use std::sync::Arc;

// what is M
// TODO: wrap cache is db wrapper
pub struct NFT {
    db: Option<Db>,
    pub nfts: Vec<(Address, NftEntry)>,
}

impl NFT {
    pub async fn build<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
        db: Option<Db>,
        fresh: bool,
    ) -> Result<Self, Box<dyn Error>> {
        match (db, fresh) {
            // db is in use and we are not refreshing
            (Some(dbi), false) => {
                let mut nfts: Vec<(Address, NftEntry)> = vec![];
                let (present, absent) = dbi.get_addresses_or_absent(&addresses);
                for (address, entry) in present {
                    nfts.push((address, entry));
                }
                // only query and build absent
                if !absent.is_empty() {
                    for (address, entry) in Self::build_entries(absent, provider).await? {
                        dbi.save_address(&address, &entry)?;
                        nfts.push((address, entry));
                    }
                }
                Ok(Self {
                    db: Some(dbi),
                    nfts,
                })
            }
            (Some(dbi), true) => {
                // db exists and we are refreshing
                let mut nfts: Vec<(Address, NftEntry)> = vec![];
                for (address, entry) in Self::build_entries(addresses, provider).await? {
                    dbi.save_address(&address, &entry)?;
                    nfts.push((address, entry));
                }
                Ok(Self {
                    db: Some(dbi),
                    nfts,
                })
            }
            (None, _) => {
                let mut nfts: Vec<(Address, NftEntry)> = vec![];
                for (address, entry) in Self::build_entries(addresses, provider).await? {
                    nfts.push((address, entry));
                }

                Ok(Self { db: None, nfts })
            }
        }
    }
    async fn build_entries<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<Vec<(Address, NftEntry)>, Box<dyn Error>> {
        let data = abi::NFTAbi::get_nft_ifaces_for_addresses(addresses, provider).await?;
        let mut results: Vec<(Address, NftEntry)> = vec![];
        for (address, supported) in data {
            results.push((address, NftEntry::new(supported)));
        }
        Ok(results)
    }
}
