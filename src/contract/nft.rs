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
    imp: abi::NFTAbi<M>,
    iface: types::NFTIface,
}

impl<M> NFT<M> {
    pub fn build(address: Address, provider: Provider<Http>) -> Result<Self, io::Error> {
        let prov = Arc::new(provider);
        let iface = abi::NFTAbi::new(address, prov.clone(), types::NFTIface::ERC721);
        match iface::guess_type(&iface) {
            Ok(answer) => match answer {
                types::NFTIface::ERC721 => Ok(Self {
                    imp: iface,
                    iface: types::NFTIface::ERC721,
                }),
                types::NFTIface::ERC1155 => {
                    let imp = abi::NFTAbi::new(
                        abi::ERC1155::new(address, prov.clone()),
                        types::NFTIface::ERC1155,
                    );
                    Ok(Self {
                        imp: imp,
                        iface: types::NFTIface::ERC1155,
                    })
                }
            },
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot initialize provider from value: {:?}", e),
            )),
        }
    }
}
