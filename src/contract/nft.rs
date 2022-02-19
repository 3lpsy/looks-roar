use crate::contract::abi;
use crate::contract::types::Iface;
use crate::db::{Db, NftEntry};
use ethers::abi::Uint;
use std::collections::HashMap;
// use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use std::error::Error;
use std::sync::Arc;

// what is M
// TODO: wrap cache is db wrapper
pub struct Nft<M> {
    db: Option<Db>,
    // TODO: make hashmap?
    pub nfts: Vec<(Address, NftEntry)>,
    pub provider: Arc<M>,
    pub fresh: bool,
}

impl<M: Middleware> Nft<M> {
    pub async fn build(
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
                    for (address, entry) in Self::build_entries(absent, provider.clone()).await? {
                        dbi.save_address(&address, &entry)?;
                        nfts.push((address, entry));
                    }
                }
                Ok(Self {
                    db: Some(dbi),
                    nfts,
                    provider,
                    fresh,
                })
            }
            (Some(dbi), true) => {
                // db exists and we are refreshing
                let mut nfts: Vec<(Address, NftEntry)> = vec![];
                for (address, entry) in Self::build_entries(addresses, provider.clone()).await? {
                    dbi.save_address(&address, &entry)?;
                    nfts.push((address, entry));
                }
                Ok(Self {
                    db: Some(dbi),
                    nfts,
                    provider,
                    fresh,
                })
            }
            (None, _) => {
                let mut nfts: Vec<(Address, NftEntry)> = vec![];
                for (address, entry) in Self::build_entries(addresses, provider.clone()).await? {
                    nfts.push((address, entry));
                }

                Ok(Self {
                    db: None,
                    nfts,
                    provider,
                    fresh,
                })
            }
        }
    }
    async fn build_entries(
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
    pub async fn enumerate(&mut self) -> Result<Vec<Uint>, Box<dyn Error>> {
        //TODO: why .as_ref()
        match (self.db.as_ref(), self.fresh) {
            (Some(dbi), false) => {
                //..
                // loop over each loaded nft
                // then query if no tokens exist
                let mut unloaded: HashMap<Address, Vec<Iface>> = HashMap::new();

                for (address, entry) in &self.nfts {
                    if entry.tokens.is_empty() {
                        unloaded.insert(address.clone(), entry.ifaces.clone());
                    }
                }
                // HashMap<Address, Vec<Uint>
                let data =
                    abi::NFTAbi::get_nft_tokens_for_addresses(unloaded, self.provider.clone())
                        .await?;

                unimplemented!();
            }
            (_, _) => unimplemented!(),
        }
    }
}
