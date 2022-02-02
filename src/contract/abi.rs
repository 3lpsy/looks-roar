use std::sync::Arc;

use super::constants;
use crate::contract::types::{NFTIface, NFTOptIface};
use ethers::contract::abigen;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use std::error::Error;
use std::io;

abigen!(
    ERC165,
    r#"[
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);
// TODO: make these implement the same trait
abigen!(
    ERC721,
    r#"[
        function balanceOf(address _owner) external view returns (uint256)
        function ownerOf(uint256 _tokenId) external view returns (address)
        function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable
        function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable;
        function transferFrom(address _from, address _to, uint256 _tokenId) external payable
        function approve(address _approved, uint256 _tokenId) external payable
        function setApprovalForAll(address _operator, bool _approved) external
        function getApproved(uint256 _tokenId) external view returns (address)
        function isApprovedForAll(address _owner, address _operator) external view returns (bool)
        function supportsInterface(bytes4 interfaceId) external view returns (bool)
    ]"#
);

abigen!(
    ERC721Metadata,
    r#"[
        function name() external view returns (string _name)
        function symbol() external view returns (string _symbol)
        function tokenURI(uint256 _tokenId) external view returns (string)
    ]"#
);

abigen!(
    ERC721Enumerable,
    r#"[
        function totalSupply() external view returns (uint256)
        function tokenByIndex(uint256 _index) external view returns (uint256)
        function tokenOfOwnerByIndex(address _owner, uint256 _index) external view returns (uint256)
    ]"#
);

abigen!(
    ERC1155,
    r#"[
        function safeTransferFrom(address _from, address _to, uint256 _id, uint256 _value, bytes calldata _data) external
        function safeBatchTransferFrom(address _from, address _to, uint256[] calldata _ids, uint256[] calldata _values, bytes calldata _data) external
        function balanceOf(address _owner) external view returns (uint256)
        function balanceOfBatch(address[] calldata _owners, uint256[] calldata _ids) external view returns (uint256[] memory)
        function setApprovalForAll(address _operator, bool _approved) external
        function isApprovedForAll(address _owner, address _operator) external view returns (bool)
    ]"#
);

abigen!(
    ERC1155MetadataUri,
    r#"[
        function uri(uint256 _id) external view returns (string memory)
    ]"#
);

pub struct NFTAbi<M> {
    erc721: Option<ERC721<M>>,
    erc1155: Option<ERC1155<M>>,
    pub iface: NFTIface,
    pub opt_ifaces: Vec<NFTOptIface>,
    pub provider: Arc<M>,
}

impl<M: Middleware> NFTAbi<M> {
    // just handle both cases
    pub async fn guess_type(address: Address, provider: Arc<M>) -> Option<NFTIface> {
        let imp = ERC165::new(address, provider);
        match imp
            .supports_interface(constants::ERC1155_IFACE_ID)
            .call()
            .await
        {
            Ok(res) => match res {
                true => Some(NFTIface::ERC1155),
                false => match imp
                    .supports_interface(constants::ERC721_IFACE_ID)
                    .call()
                    .await
                {
                    Ok(res) => match res {
                        true => Some(NFTIface::ERC721),
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
        iface: NFTIface,
        opt_ifaces: Vec<NFTOptIface>,
    ) -> Self {
        match iface {
            NFTIface::ERC721 => Self {
                erc721: Some(ERC721::new(address, provider.clone())),
                erc1155: None,
                iface,
                opt_ifaces,
                provider,
            },
            NFTIface::ERC1155 => Self {
                erc721: None,
                erc1155: Some(ERC1155::new(address, provider.clone())),
                iface,
                opt_ifaces,
                provider,
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
        imp.load_opt_interfaces().await;
        Ok(imp)
    }
    // TODO: handle both normal iface / opt iface
    async fn load_opt_interfaces(&mut self) {
        let candidates = vec![
            NFTOptIface::ERC721Enumerable,
            NFTOptIface::ERC721Metadata,
            NFTOptIface::ERC1155MetadataUri,
        ];
        for candidate in candidates {
            if self.is_interface_supported(candidate).await {
                // TODO: can i avoid this clone?
                self.opt_ifaces.push(candidate);
            }
        }
        // can ERC1155's support optional ERC721 interfaces?
    }

    async fn is_interface_supported(&self, iface: NFTOptIface) -> bool {
        match self.to_erc165().supports_interface(iface.id()).call().await {
            Ok(answer) => answer,
            Err(_e) => false,
        }
    }

    fn address(&self) -> Address {
        match self.iface {
            NFTIface::ERC721 => match &self.erc721 {
                Some(imp) => imp.address(),
                None => panic!("Contructed without contract"),
            },
            NFTIface::ERC1155 => match &self.erc1155 {
                Some(imp) => imp.address(),
                None => panic!("Contructed with wrong contract/iface"),
            },
        }
    }

    fn to_erc165(&self) -> ERC165<M> {
        ERC165::new(self.address(), self.provider.clone())
    }
}
