use crate::contract::constants;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum Iface {
    ERC721 = 0,
    ERC1155 = 1,
    ERC721Enumerable = 2,
    ERC721Metadata = 3,
    ERC1155MetadataUri = 4,
}

impl Iface {
    pub fn id(&self) -> [u8; 4] {
        match self {
            Self::ERC721 => constants::ERC721_IFACE_ID,
            Self::ERC721Enumerable => constants::ERC721_ENUMERABLE_IFACE_ID,
            Self::ERC721Metadata => constants::ERC721_METADATA_IFACE_ID,
            Self::ERC1155 => constants::ERC1155_IFACE_ID,
            Self::ERC1155MetadataUri => constants::ERC1155_METADATA_URI_IFACE_ID,
        }
    }
    pub fn from_id(id: [u8; 4]) -> Self {
        match id {
            constants::ERC721_IFACE_ID => Self::ERC721,
            constants::ERC721_ENUMERABLE_IFACE_ID => Self::ERC721Enumerable,
            constants::ERC721_METADATA_IFACE_ID => Self::ERC721Metadata,
            constants::ERC1155_IFACE_ID => Self::ERC1155,
            constants::ERC1155_METADATA_URI_IFACE_ID => Self::ERC1155MetadataUri,
            _ => panic!("Bad id for Iface"),
        }
    }
    pub fn all_ids() -> Vec<[u8; 4]> {
        vec![
            constants::ERC721_IFACE_ID,
            constants::ERC721_ENUMERABLE_IFACE_ID,
            constants::ERC721_METADATA_IFACE_ID,
            constants::ERC1155_IFACE_ID,
            constants::ERC1155_METADATA_URI_IFACE_ID,
        ]
    }
}
impl fmt::Display for Iface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ERC721 => write!(f, "ERC721"),
            Self::ERC721Enumerable => write!(f, "ERC721Enumerable"),
            Self::ERC721Metadata => write!(f, "ERC721Metadata"),
            Self::ERC1155 => write!(f, "ERC1155"),
            Self::ERC1155MetadataUri => write!(f, "ERC1155MetadataURI"),
        }
    }
}
