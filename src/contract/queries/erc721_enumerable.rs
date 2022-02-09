use crate::contract::abis::ERC721Enumerable;
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
                let mut tokens: Vec<U256> = vec![];
                let total_supply_willing: u64 = total_supply.as_u64();
                let mut range = (0..total_supply_willing).peekable();
                while range.peek().is_some() {
                    let batch = range.by_ref().take(100);
                    let futs = batch.into_iter().map(|index| {
                        let imp = &imp;
                        let uindex = U256::from(index);
                        async move {
                            //...
                            let token = imp.token_by_index(uindex).call().await.unwrap();
                            println!("Token: {:?}", token.clone());
                            token
                        }
                    });
                    let mut found = join_all(futs).await;
                    tokens.append(&mut found);
                }
                println!("len: {:?}", tokens.len());
                Ok(tokens)
            }
            Err(e) => Err(AppError::boxed(
                format!("Total Supply call failed: {:?}", e),
                0,
            )),
        }
    }
}
