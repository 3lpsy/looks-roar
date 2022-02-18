use super::queries::IfacesQuery;
use crate::contract::types::Iface;
use crate::utils::AppError;
use ethers::core::types::Address;
use ethers::providers::Middleware;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub struct NFTAbi;

impl NFTAbi {
    pub async fn get_nft_ifaces_for_addresses<M: Middleware>(
        addresses: Vec<Address>,
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Iface>>, Box<dyn Error>> {
        let responses =
            IfacesQuery::get_ifaces_for_addresses(addresses, Iface::all_ids(), provider).await?;
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
}
