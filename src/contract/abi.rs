use ethers::contract::abigen;

abigen!(
    GenericNFT,
    r#"[
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

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
