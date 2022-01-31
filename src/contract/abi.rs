use std::sync::Arc;

use ethers::contract::abigen;
use ethers::core::types::Address;
use ethers::providers::{Middleware, Provider};

use super::{constants, types};
use std::io;

// TODO: make these implement the same trait
abigen!(
    ERC721,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

abigen!(
    ERC1155,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

pub struct NFTAbi<M> {
    erc721: Option<ERC721<M>>,
    erc1155: Option<ERC1155<M>>,
    pub iface: types::NFTIface,
    pub opt_ifaces: Vec<types::NFTOptIface>,
}

impl<M: Middleware> NFTAbi<M> {
    // just handle both cases
    pub async fn guess_type(address: Address, provider: Arc<M>) -> Option<types::NFTIface> {
        let imp = ERC721::new(address, provider);
        match imp
            .supports_interface(constants::ERC1155_IFACE_ID)
            .call()
            .await
        {
            Ok(res) => match res {
                true => Some(types::NFTIface::ERC1155),
                false => match imp
                    .supports_interface(constants::ERC721_IFACE_ID)
                    .call()
                    .await
                {
                    Ok(res) => match res {
                        true => Some(types::NFTIface::ERC721),
                        false => None,
                    },
                    Err(_e) => None,
                },
            },
            Err(_e) => None,
        }
    }

    pub fn new(
        address: Address,
        provider: Arc<M>,
        iface: types::NFTIface,
        opt_ifaces: Vec<types::NFTOptIface>,
    ) -> Self {
        match iface {
            types::NFTIface::ERC721 => Self {
                erc721: Some(ERC721::new(address, provider)),
                erc1155: None,
                iface,
                opt_ifaces,
            },
            types::NFTIface::ERC1155 => Self {
                erc721: None,
                erc1155: Some(ERC1155::new(address, provider)),
                iface,
                opt_ifaces,
            },
        }
    }

    pub async fn build(address: Address, provider: Arc<M>) -> Result<Self, io::Error> {
        let iface = match Self::guess_type(address, provider.clone()).await {
            Some(found) => found,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No iface found for address",
                ));
            }
        };
        let mut imp = Self::new(address, provider, iface, vec![]);
        imp.load_opt_interfaces();
        Ok(imp)
    }
    // TODO: handle both normal iface / opt iface
    async fn load_opt_interfaces(&mut self) {
        let candidates = vec![
            types::NFTOptIface::ERC721Enumerable,
            types::NFTOptIface::ERC721Metadata,
            types::NFTOptIface::ERC1155MetadataUri,
        ];
        for candidate in candidates {
            if self.is_interface_supported(candidate).await {
                // TODO: can i avoid this clone?
                self.opt_ifaces.push(candidate);
            }
        }
        // can ERC1155's support optional ERC721 interfaces?
    }

    async fn is_interface_supported(&self, iface: types::NFTOptIface) -> bool {
        match self.iface {
            types::NFTIface::ERC721 => {
                match self
                    .erc721
                    .as_ref()
                    .unwrap()
                    .supports_interface(iface.id())
                    .call()
                    .await
                {
                    Ok(answer) => answer,
                    Err(_e) => false,
                }
            }
            types::NFTIface::ERC1155 => {
                match self
                    .erc1155
                    .as_ref()
                    .unwrap()
                    .supports_interface(iface.id())
                    .call()
                    .await
                {
                    Ok(answer) => answer,
                    Err(_e) => false,
                }
            }
        }
    }
}
