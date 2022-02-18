use std::collections::HashMap;
use std::sync::Arc;

use super::constants;
// use super::queries;
use crate::contract::abis::ERC165;
use crate::contract::types::NFTIface;
use crate::utils::AppError;
use ethers::abi::Tokenizable;
use ethers::core::types::Address;
use ethers::prelude::Multicall;
// use ethers::prelude::Signer;
// use ethers::prelude::U256;
use ethers::providers::Middleware;
use std::error::Error;

pub struct NftIfacesResponse {
    address: Address,
    ifaces: Vec<NFTIface>,
}
pub struct IfaceResponse {
    address: Address,
    ifaces: Vec<[u8; 4]>,
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

pub struct NFTAbi<M> {
    x: M,
}

impl<M: Middleware> NFTAbi<M> {
    pub async fn guess_nft_ifaces(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<[u8; 4]>>, Box<dyn Error>> {
        let responses = Self::guess_ifaces(addresses, NFTIface::all_ids(), provider).await?;
        let mut missing: Vec<Address> = vec![];

        // loop over map
        for (addr, ifaces) in &responses {
            if ifaces.is_empty() {
                missing.push(*addr)
            }
        }
        if !missing.is_empty() {
            return Err(AppError::boxed(
                format!("Found unsupported addresses for ifaces: {:?}", missing),
                0,
            ));
        }
        Ok(responses)
    }

    // ifaces is probably going to be less than 1000 calls in a multicall
    pub async fn guess_ifaces(
        addresses: Vec<Address>,
        ifaces: Vec<[u8; 4]>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<[u8; 4]>>, Box<dyn Error>> {
        // M is included in potential error so can't use '?'
        // multicall is restricted to 16 at a time
        let m = Multicall::new(provider.clone(), None).await.unwrap();
        let mut queries: Vec<(Address, [u8; 4])> = vec![];
        let multi = addresses
            .iter()
            .map(|a| {
                //...
                ERC165::new(a.to_owned(), provider.clone())
            })
            .fold(m, |mut multi_carry, contract| {
                for iface in &ifaces[..] {
                    queries.push((contract.address().clone(), iface.clone()));
                    multi_carry.add_call(contract.supports_interface(*iface));
                }
                multi_carry
            });

        // TODO: does it err if 165 is unsupported?
        let responses: Vec<bool> = match multi.call_raw().await {
            Ok(result) => result
                .iter()
                .map(|t| bool::from_token(t.to_owned()).unwrap())
                .collect(),
            Err(_e) => return Err(AppError::boxed("Multicall failed".to_string(), 0)),
        };

        assert!(responses.len() == queries.len());

        let res = responses
            .iter()
            .zip(queries.iter())
            .map(|(status, query)| (query.0, query.1, status))
            .fold(HashMap::new(), |mut carry, (addr, iface, status)| {
                carry.entry(addr).or_insert_with(Vec::new);
                let entry = carry.get_mut(&addr).unwrap();
                if *status {
                    // only append true ifaces
                    entry.push(iface);
                }
                carry
            });

        Ok(res)
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
}
