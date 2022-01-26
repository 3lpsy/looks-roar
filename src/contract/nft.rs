use crate::contract::{abi, constants, types, wrapper};
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
use std::sync::Arc;

// what is M

pub async fn build(address: Address, provider: Provider<Http>) -> Result<(), io::Error> {
    Ok(())
}

pub struct NFT<M> {
    imp: Box<dyn abi::NFTContract<M>>,
}

impl<M> NFT<M> {
    pub fn guess_type(x: &abi::ERC721<Provider<Http>>) -> Result<types::NFTIface, io::Error> {
        Ok(types::NFTIface::ERC721)
    }

    pub fn build(address: Address, provider: Provider<Http>) -> Result<Self, io::Error> {
        let prov = Arc::new(provider);
        let iface = abi::ERC721::new(address, prov.clone());
        match Self::guess_type(&iface) {
            Ok(answer) => match answer {
                types::NFTIface::ERC721 => Ok(Self {
                    imp: Box::new(iface),
                }),
                types::NFTIface::ERC1155 => Ok(Self {
                    imp: Box::new(abi::ERC1155::new(address, prov.clone())),
                }),
            },
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot initialize provider from value: {:?}", e),
            )),
        }
    }
}
