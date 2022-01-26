use ethers::contract::abigen;

pub trait NFTContract<M> {}

abigen!(
    ERC721,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

impl NFTContract<M> for ERC721<M> {}

abigen!(
    ERC1155,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

impl NFTContract<M> for ERC1155<M> {}
