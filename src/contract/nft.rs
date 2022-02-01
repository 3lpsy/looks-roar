use crate::cache::types::AddressCache;
use crate::cache::Cache;
use crate::contract::{abi, types};
use ethers::core::types::Address;
use ethers::providers::Middleware;
use std::io;
use std::sync::Arc;

// what is M
// TODO: wrap cache is db wrapper
pub struct NFT<M> {
    imp: abi::NFTAbi<M>,
    db: Option<Cache>,
}

impl<M: Middleware> NFT<M> {
    pub async fn build(
        address: Address,
        provider: Arc<M>,
        db: Option<Cache>,
        fresh: bool,
    ) -> Result<Self, io::Error> {
        match (db, fresh) {
            (Some(dbi), false) => match dbi.get_address(&address) {
                Some(address_cache) => {
                    // TODO: is deser deadly in rust?
                    println!("Building from cache: {:?}", address.clone());
                    let imp = abi::NFTAbi::new(
                        address,
                        provider,
                        address_cache.iface,
                        address_cache.opt_ifaces,
                    );
                    Ok(Self { imp, db: Some(dbi) })
                }
                None => {
                    println!("Cache miss: {:?}", address.clone());
                    Self::build_from_scratch(address, provider, Some(dbi)).await
                }
            },
            (Some(dbi), true) => Self::build_from_scratch(address, provider, Some(dbi)).await,
            (None, _) => Self::build_from_scratch(address, provider, None).await,
        }
    }

    async fn build_from_scratch(
        address: Address,
        provider: Arc<M>,
        db: Option<Cache>,
    ) -> Result<Self, io::Error> {
        match abi::NFTAbi::build(address, provider.clone()).await {
            Ok(imp) => {
                // TODO: need to update cache if it exists
                match db {
                    Some(dbi) => {
                        let ace = AddressCache::new(imp.iface, imp.opt_ifaces.clone());
                        let _res = dbi.save_address(&address, &ace);
                        Ok(Self { imp, db: Some(dbi) })
                    }
                    None => Ok(Self { imp, db: None }),
                }
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot initialize provider from value: {:?}", e),
            )),
        }
    }
    pub fn iface(&self) -> &types::NFTIface {
        &self.imp.iface
    }
    pub fn opt_ifaces(&self) -> &Vec<types::NFTOptIface> {
        &self.imp.opt_ifaces
    }
}
