use crate::contract::abi::{self, ERC721Enumerable};
use crate::utils::AppError;
use ethers::prelude::U256;
use ethers::providers::Middleware;
use std::error::Error;

pub struct ERC721EnumerableQuery {}

impl ERC721EnumerableQuery {
    pub async fn fetch_tokens<M: Middleware>(
        imp: ERC721Enumerable<M>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        match imp.total_supply().call().await {
            Ok(total_supply) => {
                let mut tokens: Vec<String> = vec![];
                for index in 0..total_supply {
                    let index_str = index.to_string();
                    match imp.token_by_index(index).call().await {
                        Ok(token) => tokens.push(token),
                        Err(e) => {
                            println!("TokenByIndex Error: {:?}", e);
                        }
                    }
                }
                unimplemented!();
            }
            Err(e) => Err(AppError::boxed(
                format!("Total Supply call failed: {:?}", e),
                0,
            )),
        }
    }
}
