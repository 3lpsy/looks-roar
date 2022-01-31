use crate::cache::types::AddressCache;
use crate::contract::{abi, types};
use bincode;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use sled;
use std::io;
use std::sync::Arc;

// what is M

pub struct NFT<M> {
    imp: abi::NFTAbi<M>,
    cache: Option<sled::Db>,
}

impl<M: Middleware> NFT<M> {
    pub async fn build(
        address: Address,
        provider: Arc<M>,
        cache: Option<sled::Db>,
    ) -> Result<Self, io::Error> {
        match cache {
            Some(c) => match c.get(address) {
                Ok(res) => match res {
                    Some(data) => {
                        unimplemented!();
                    }
                    None => {
                        // not in cache scenario
                        // why do I need to rewrap the cache to avoid partial borrow?
                        Self::build_from_scratch(address, provider.clone(), Some(c)).await
                    }
                },
                Err(e) => {
                    // TODO: better handling
                    println!("Failed to read from cache: {:?}", e);
                    unimplemented!();
                }
            },
            None => Self::build_from_scratch(address, provider.clone(), cache).await,
        }
    }
    async fn build_from_scratch(
        address: Address,
        provider: Arc<M>,
        cache: Option<sled::Db>,
    ) -> Result<Self, io::Error> {
        match abi::NFTAbi::build(address, provider.clone()).await {
            Ok(imp) => {
                // TODO: need to update cache if it exists
                let imp = Self { imp, cache };

                if imp.cache.is_some() {
                    let ckey = address;
                    let cval = AddressCache {
                        iface: imp.iface().clone(),
                        opt_ifaces: imp.opt_ifaces().clone(),
                    };
                    let cbin = bincode::serialize(&cval).unwrap();
                    imp.cache.unwrap().insert(ckey.to_string(), &cbin);
                }

                Ok(imp)
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot initialize provider from value"),
            )),
        }
    }
    pub fn iface(&self) -> &types::NFTIface {
        &self.imp.iface
    }
    pub fn opt_ifaces(&self) -> &Vec<types::NFTOptIface> {
        &self.imp.opt_ifaces
    }
    pub async fn load_metadata(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
