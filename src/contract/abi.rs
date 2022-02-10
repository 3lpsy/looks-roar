use std::sync::Arc;

use super::constants;
use super::queries;
use crate::contract::abis::{ERC721Enumerable, ERC1155, ERC165, ERC721};
use crate::contract::types::NFTIface;
use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::prelude::Multicall;
use ethers::prelude::Signer;
use ethers::prelude::U256;
use ethers::providers::Middleware;
use std::error::Error;

pub struct NftIfacesResponse {
    address: Address,
    ifaces: Vec<NFTIface>,
}
pub struct IfaceResponse {
    address: Address,
    ifaces: [u8; 4],
}

impl From<IfaceResponse> for NftIfacesResponse {
    fn from(response: IfaceResponse) -> Self {
        let ifaces = response
            .ifaces
            .into_iter()
            .map(|i| NFTIface::from_id(i))
            .collect();
        Self {
            address: response.address,
            ifaces,
        }
    }
}

pub struct NFTAbi;

impl NFTAbi {
    pub async fn guess_nft_ifaces<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<Vec<NftIfacesResponse>, Box<dyn Error>> {
        let responses =
            Self::guess_ifaces_or_unsupported(addresses, NFTIface::all_ids(), provider).await?;
        let mut missing: Vec<Address> = vec![];
        for response in responses {
            if response.ifaces > 0 {
                missing.push(response.address)
            }
        }
        if missing.len() > 0 {
            Err(AppError::new(
                format!("Found unsupported addresses for ifaces: {:?}", missing),
                0,
            ))
        }
        unimplemented!()
    }

    pub async fn guess_ifaces_or_unsupported<M: Middleware>(
        addresses: Vec<Address>,
        ifaces: Vec<[u8; 4]>,
        provider: Arc<M>,
    ) -> Vec<IfaceResponse> {
        for address in addresses {
            let contract = ERC165::new(address, provider.clone());
            let multi = Multicall::new(provider.clone(), address);
            // can i do a multi call to many addresses
            let calls = ifaces.into_iter(
        }
        // check for 1155s support first
        for iface in ifaces {
            let calls = addresses
                .into_iter()
                .map(|a| contract.supports_interface(iface))
                .collect();
        }

        unimplemented!();
    }

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
        opt_ifaces: Vec<NFTIface>,
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
