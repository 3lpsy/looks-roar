use std::sync::Arc;

use super::constants;
use super::queries;
use crate::contract::abis::{ERC721Enumerable, ERC1155, ERC165, ERC721};
use crate::contract::types::{NFTIface, NFTOptIface};
use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::prelude::U256;
use ethers::providers::Middleware;
use std::error::Error;

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

    pub async fn build(address: Address, provider: Arc<M>) -> Result<Self, Box<dyn Error>> {
        let iface = match Self::guess_type(address, provider.clone()).await {
            Some(found) => found,
            None => {
                let m = format!("No iface found for address: {:?}", address);
                return Err(AppError::boxed(m, 0));
            }
        };
        let mut imp = Self::new(address, provider, iface, vec![]);
        imp.load_opt_interfaces().await;
        Ok(imp)
    }

    pub async fn fetch_tokens(&self) -> Result<Vec<U256>, Box<dyn Error>> {
        if self.has_opt_interface(NFTOptIface::ERC721Enumerable) {
            let tokens =
                queries::ERC721EnumerableQuery::fetch_tokens(self.to_erc721_enumerable()).await?;
            return Ok(tokens);
        }
        unimplemented!();
    }

    // TODO: handle both normal iface / opt iface
    async fn load_opt_interfaces(&mut self) {
        let candidates = vec![
            NFTOptIface::ERC721Enumerable,
            NFTOptIface::ERC721Metadata,
            NFTOptIface::ERC1155MetadataUri,
        ];
        for candidate in candidates {
            if self.query_interface_support(candidate).await {
                // TODO: can i avoid this clone?
                self.opt_ifaces.push(candidate);
            }
        }
        // can ERC1155's support optional ERC721 interfaces?
    }

    pub fn has_opt_interface(&self, iface: NFTOptIface) -> bool {
        self.opt_ifaces.contains(&iface)
    }

    async fn query_interface_support(&self, iface: NFTOptIface) -> bool {
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
    fn to_erc721_enumerable(&self) -> ERC721Enumerable<M> {
        ERC721Enumerable::new(self.address(), self.provider.clone())
    }
}
