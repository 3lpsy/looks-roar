use core::fmt;
use std::sync::Arc;

use ethers::contract::abigen;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};

use super::types;

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
    iface: types::NFTIface,
}

impl<M> NFTAbi<M> {
    pub fn new(address: Address, provider: Arc<Provider<Http>>, iface: types::NFTIface) -> Self {}
}
