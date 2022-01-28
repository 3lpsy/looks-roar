use std::sync::Arc;

use ethers::contract::abigen;
use ethers::core::types::Address;
use ethers::providers::{Http, Middleware, Provider};

use super::{constants, types};
use std::io;

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
}

impl NFTAbi<Provider<Http>> {
    // just handle both cases
    pub async fn guess_type(
        address: Address,
        provider: Arc<Provider<Http>>,
    ) -> Option<types::NFTIface> {
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
                    Err(e) => None,
                },
            },
            Err(e) => None,
        }
    }

    pub async fn new(address: Address, provider: Arc<Provider<Http>>) -> Result<Self, io::Error> {
        let iface = match Self::guess_type(address, provider.clone()).await {
            Some(found) => found,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No iface found for address",
                ));
            }
        };

        match iface {
            types::NFTIface::ERC721 => Ok(Self {
                erc721: Some(ERC721::new(address, provider)),
                erc1155: None,
                iface,
            }),
            types::NFTIface::ERC1155 => Ok(Self {
                erc721: None,
                erc1155: Some(ERC1155::new(address, provider)),
                iface,
            }),
        }
    }
}
