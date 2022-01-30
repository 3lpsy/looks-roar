use crate::contract::{abi, types};
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
use std::sync::Arc;

// what is M

pub async fn build(address: Address, provider: Provider<Http>) -> Result<(), io::Error> {
    Ok(())
}

pub struct NFT<M> {
    imp: abi::NFTAbi<M>,
}

impl NFT<Provider<Http>> {
    pub async fn build(address: Address, provider: Arc<Provider<Http>>) -> Result<Self, io::Error> {
        match abi::NFTAbi::new(address, provider.clone()).await {
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
    pub async fn load_metadata(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
