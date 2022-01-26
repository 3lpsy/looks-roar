use core::fmt;

use ethers::contract::abigen;

pub trait NFTContract {}

abigen!(
    ERC721,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

impl<M> NFTContract for ERC721<M> {}

abigen!(
    ERC1155,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

impl<M> NFTContract for ERC1155<M> {}
