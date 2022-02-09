pub use erc721enumerable_mod::*;
#[allow(clippy::too_many_arguments)]
mod erc721enumerable_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ERC721Enumerable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC721ENUMERABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: abi :: parse_abi_str ("[\n    function totalSupply() external view returns (uint256)\n    function tokenByIndex(uint256 _index) external view returns (uint256)\n    function tokenOfOwnerByIndex(address _owner, uint256 _index) external view returns (uint256)\n]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ERC721Enumerable<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ERC721Enumerable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC721Enumerable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC721Enumerable))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ERC721Enumerable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                ERC721ENUMERABLE_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `tokenByIndex` (0x4f6ccce7) function"]
        pub fn token_by_index(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function"]
        pub fn token_of_owner_by_index(
            &self,
            owner: ethers::core::types::Address,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `tokenByIndex`function with signature `tokenByIndex(uint256)` and selector `[79, 108, 204, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `tokenOfOwnerByIndex`function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `[47, 116, 92, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721EnumerableCalls {
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TotalSupply(TotalSupplyCall),
    }
    impl ethers::core::abi::AbiDecode for ERC721EnumerableCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <TokenByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721EnumerableCalls::TokenByIndex(decoded));
            }
            if let Ok(decoded) =
                <TokenOfOwnerByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721EnumerableCalls::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721EnumerableCalls::TotalSupply(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC721EnumerableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC721EnumerableCalls::TokenByIndex(element) => element.encode(),
                ERC721EnumerableCalls::TokenOfOwnerByIndex(element) => element.encode(),
                ERC721EnumerableCalls::TotalSupply(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC721EnumerableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721EnumerableCalls::TokenByIndex(element) => element.fmt(f),
                ERC721EnumerableCalls::TokenOfOwnerByIndex(element) => element.fmt(f),
                ERC721EnumerableCalls::TotalSupply(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<TokenByIndexCall> for ERC721EnumerableCalls {
        fn from(var: TokenByIndexCall) -> Self {
            ERC721EnumerableCalls::TokenByIndex(var)
        }
    }
    impl ::std::convert::From<TokenOfOwnerByIndexCall> for ERC721EnumerableCalls {
        fn from(var: TokenOfOwnerByIndexCall) -> Self {
            ERC721EnumerableCalls::TokenOfOwnerByIndex(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ERC721EnumerableCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ERC721EnumerableCalls::TotalSupply(var)
        }
    }
}
