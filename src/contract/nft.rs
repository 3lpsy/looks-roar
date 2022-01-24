use crate::contract::{abi, constants};
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum NFTType {
    ERC721,
    ERC1155,
}

// what is M
pub struct NFT<M> {
    erc721: Option<abi::ERC721<M>>,
    erc1155: Option<abi::ERC1155<M>>,
    pub iface: NFTType,
}

impl<M> NFT<M> {
    pub fn new_erc721(contract: abi::ERC721<M>) -> Self {
        Self {
            erc721: Some(contract),
            erc1155: None,
            iface: NFTType::ERC721,
        }
    }
    pub fn new_erc1155(contract: abi::ERC1155<M>) -> Self {
        Self {
            erc1155: Some(contract),
            erc721: None,
            iface: NFTType::ERC1155,
        }
    }
}

pub async fn build_any(
    address: Address,
    provider: Provider<Http>,
) -> Result<NFT<Provider<Http>>, io::Error> {
    let provarc = Arc::new(provider);
    let iface = abi::GenericNFT::new(address, provarc.clone());
    match iface
        .supports_interface(constants::ERC1155_IFACE_ID)
        .call()
        .await
    {
        Ok(answer) => match answer {
        true => {
                println!("Supports 1155");
                let contract = abi::ERC1155::new(address, provarc.clone());
                Ok(NFT::new_erc1155(contract))
            }
            false => {
                match iface
                    .supports_interface(constants::ERC721_IFACE_ID)
                    .call()
                    .await
                {
                    Ok(answer) => match answer {
                        true => {
                            println!("Supports 721");
                            let contract = abi::ERC721::new(address, provarc.clone());
                            Ok(NFT::new_erc721(contract))
                        }
                        false => {
                            panic!("Bad Supports Interface Answer");
                        }
                    },
                    Err(_) => panic!("Bad Supports Interface Answer"),
                }
            }
        },
        Err(e) => {
            // what to do
            panic!("Bad Supports Interface Answer");
        }
    }
}
