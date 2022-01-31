use crate::contract::{abi, types};
use ethers::core::types::Address;
use ethers::providers::{Middleware, Provider};
use std::io;
use std::sync::Arc;

// what is M

pub struct NFT<M> {
    imp: abi::NFTAbi<M>,
}

impl<M: Middleware> NFT<M> {
    pub async fn build(address: Address, provider: Arc<M>) -> Result<Self, io::Error> {
        match abi::NFTAbi::build(address, provider.clone()).await {
            Ok(imp) => Ok(Self { imp }),
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
