use crate::contract::{abi, constants, types, wrapper};
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
use std::sync::Arc;

// what is M

pub async fn build(address: Address, provider: Provider<Http>) -> Result<(), io::Error> {
    Ok(())
}

pub struct NFT {
    imp: Box<dyn abi::NFTContract>,
}

impl NFT {
    pub fn supports_interface(&self, identifier: [u8; 4]) -> bool {
        // not defined
        // but how to define on trait when its already defined it its struct
        match self.imp.supports_interface(identifier) {
            Ok(answer) => answer,
            Err(e) => false,
        }
    }

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
