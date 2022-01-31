use crate::cache::types::AddressCache;
use crate::contract::{abi, types};
use bincode;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use sled::{self, IVec};
use std::io;
use std::sync::Arc;

// what is M
// TODO: wrap cache is db wrapper
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
                        // TODO: is deser deadly in rust?
                        println!("Building from cache: {:?}", address.clone());
                        Self::build_from_cache(address, provider.clone(), Some(c), data)
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

    fn build_from_cache(
        address: Address,
        provider: Arc<M>,
        cache: Option<sled::Db>,
        cbin: IVec,
    ) -> Result<Self, io::Error> {
        //TODO: handle bad data
        let cval: AddressCache = bincode::deserialize(&cbin).unwrap();
        let imp = abi::NFTAbi::new(address, provider, cval.iface, cval.opt_ifaces);
        Ok(Self { imp, cache })
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
                    let cval = AddressCache {
                        iface: imp.iface().clone(),
                        opt_ifaces: imp.opt_ifaces().clone(),
                    };
                    let cbin = bincode::serialize(&cval).unwrap();
                    imp.insert(address, cbin);
                    Ok(imp)
                } else {
                    Ok(imp)
                }
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot initialize provider from value: {:?}", e),
            )),
        }
    }
    pub fn insert<K, V>(&self, key: K, bin: V) -> Option<IVec>
    where
        K: AsRef<[u8]> + std::fmt::Debug + Copy,
        V: Into<IVec>,
    {
        let k = key.clone();
        match &self.cache {
            Some(c) => {
                match c.insert(key, bin) {
                    Ok(res) => {
                        //...
                        println!("Saved to cache: {:?}", k);
                        res
                    }
                    Err(e) => {
                        println!("Cache insert error: {:?}", e);
                        None
                    }
                }
            }
            None => {
                //....
                None
            }
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
