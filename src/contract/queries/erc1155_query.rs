use crate::contract::abis::erc1155::TransferSingleFilter;
use crate::utils::AppError;
use ethers::abi::RawLog;
use ethers::abi::Uint;
use ethers::contract::EthEvent;
use ethers::core::types::Address;
use ethers::prelude::BlockNumber;
use ethers::prelude::Filter;
use ethers::prelude::LogMeta;
use ethers::prelude::Middleware;
use ethers::prelude::ValueOrArray;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::sync::Arc;

pub struct ERC1155Query;

impl ERC1155Query {
    pub async fn get_tokens_for_addresses<M: Middleware>(
        addresses: &[Address],
        provider: Arc<M>,
    ) -> Result<HashMap<Address, Vec<Uint>>, Box<dyn Error>> {
        let mut results: HashMap<Address, Vec<Uint>> = HashMap::new();
        for address in addresses {
            results.insert(*address, vec![]);
        }
        for (event, meta) in
            Self::get_unlimited_logs_for_event::<M, TransferSingleFilter>(addresses, provider)
                .await?
        {
            let address = meta.address;
            let token_id = event.id;
            let entry = results.get_mut(&address).unwrap();
            entry.push(token_id);
        }
        Ok(results)
    }
    pub async fn get_unlimited_logs_for_event<M: Middleware, D: EthEvent>(
        addresses: &[Address],
        provider: Arc<M>,
    ) -> Result<Vec<(D, LogMeta)>, Box<dyn Error>> {
        let filter = Filter::new()
            .event(&D::abi_signature())
            .address(ValueOrArray::Array(addresses.to_vec()))
            .from_block(0u64)
            .to_block(BlockNumber::Latest);
        let logs = match provider.get_logs(&filter).await {
            Ok(logs) => logs,
            Err(e) => return Err(AppError::boxed(format!("Failed to get logs: {:?}", e), 0)),
        };
        Ok(logs
            .iter()
            .map(|log| {
                dbg!(log.clone());
                let meta = LogMeta::from(log);
                let raw = RawLog {
                    topics: log.topics.clone(),
                    data: log.data.to_vec(),
                };
                let event = D::decode_log(&raw).unwrap();
                (event, meta)
            })
            .collect())
    }
}
