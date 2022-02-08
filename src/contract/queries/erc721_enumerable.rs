use crate::contract::abi::ERC721Enumerable;
use crate::utils::AppError;
use ethers::prelude::builders::ContractCall;
use ethers::prelude::U256;
use ethers::providers::Middleware;
use futures::future::join_all;
use futures::prelude::stream::FuturesUnordered;
use std::error::Error;

pub struct ERC721EnumerableQuery {}

impl ERC721EnumerableQuery {
    pub async fn fetch_tokens<M: Middleware>(
        imp: ERC721Enumerable<M>,
    ) -> Result<Vec<U256>, Box<dyn Error>> {
        match imp.total_supply().call().await {
            Ok(total_supply) => {
                let total_supply_willing: u64 = total_supply.as_u64();
                let range = 0..total_supply_willing;
                let tokens = join_all(range.into_iter().map(|index| {
                    let imp = &imp;
                    let uindex = U256::from(index);
                    async move {
                        //...
                        let token = imp.token_by_index(uindex).call().await.unwrap();
                        println!("Token: {:?}", token.clone());
                        token
                    }
                }))
                .await;
                Ok(tokens)
            }
            Err(e) => Err(AppError::boxed(
                format!("Total Supply call failed: {:?}", e),
                0,
            )),
        }
    }
}
