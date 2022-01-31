use crate::contract::constants;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum NFTIface {
    ERC721 = 0,
    ERC1155 = 1,
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum NFTOptIface {
    ERC721Enumerable = 0,
    ERC721Metadata = 1,
    ERC1155MetadataUri = 2,
}

impl NFTOptIface {
    pub fn id(&self) -> [u8; 4] {
        match self {
            Self::ERC721Enumerable => constants::ERC721_ENUMERABLE_IFACE_ID,
            Self::ERC721Metadata => constants::ERC721_METADATA_IFACE_ID,
            Self::ERC1155MetadataUri => constants::ERC1155_METADATA_URI_IFACE_ID,
        }
    }
    pub fn from_id(id: [u8; 4]) -> Self {
        match id {
            constants::ERC721_ENUMERABLE_IFACE_ID => Self::ERC721Enumerable,
            constants::ERC721_METADATA_IFACE_ID => Self::ERC721Metadata,
            constants::ERC1155_METADATA_URI_IFACE_ID => Self::ERC1155MetadataUri,
            _ => panic!("Bad id for NFTOptIface"),
        }
    }
}
impl fmt::Display for NFTOptIface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ERC721Enumerable => write!(f, "ERC721Enumerable"),
            Self::ERC721Metadata => write!(f, "ERC721Metadata"),
            Self::ERC1155MetadataUri => write!(f, "ERC1155MetadataURI"),
        }
    }
}
