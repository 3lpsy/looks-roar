use crate::contract::{abi, constants, types};

pub struct NFT2<M> {
    erc721: Option<abi::ERC721<M>>,
    erc1155: Option<abi::ERC1155<M>>,
    pub iface: types::NFTIface,
}

impl<M> NFT2<M> {
    pub fn new_erc721(contract: abi::ERC721<M>) -> Self {
        Self {
            erc721: Some(contract),
            erc1155: None,
            iface: types::NFTIface::ERC721,
        }
    }
    pub fn new_erc1155(contract: abi::ERC1155<M>) -> Self {
        Self {
            erc1155: Some(contract),
            erc721: None,
            iface: types::NFTIface::ERC1155,
        }
    }
}
