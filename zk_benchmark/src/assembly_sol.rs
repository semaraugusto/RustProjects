pub use assembly_sol::*;
#[allow(clippy::too_many_arguments)]
mod assembly_sol {
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
    #[doc = "AssemblySol was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ASSEMBLYSOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"constructor\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier_addr\",\"type\":\"address\"}]},{\"type\":\"function\",\"name\":\"BYTE_SIZE_MAX_EDGE_1\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"SNARK_FIELD\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getData\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_extDataHash\",\"type\":\"uint256\"},{\"internalType\":\"uint256[]\",\"name\":\"_inputNullifiers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_outputCommitments\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"_chainId\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\"}],\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getHash\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_extDataHash\",\"type\":\"uint256\"},{\"internalType\":\"uint256[]\",\"name\":\"_inputNullifiers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_outputCommitments\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"_chainId\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"helloWorld\",\"inputs\":[],\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"verifyInputs\",\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"_a\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"_b\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"_c\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[1]\",\"name\":\"_input\",\"type\":\"uint256[1]\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"ChainId\",\"inputs\":[{\"name\":\"chainId\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"DataValue\",\"inputs\":[{\"name\":\"data\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PartialSuccess\",\"inputs\":[{\"name\":\"info\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AssemblySol<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AssemblySol<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AssemblySol<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AssemblySol))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AssemblySol<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), ASSEMBLYSOL_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `BYTE_SIZE_MAX_EDGE_1` (0x8b502295) function"]
        pub fn byte_size_max_edge_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 80, 34, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SNARK_FIELD` (0x218df2e3) function"]
        pub fn snark_field(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([33, 141, 242, 227], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getData` (0x96e4746d) function"]
        pub fn get_data(
            &self,
            public_amount: ethers::core::types::U256,
            ext_data_hash: ethers::core::types::U256,
            input_nullifiers: ::std::vec::Vec<ethers::core::types::U256>,
            output_commitments: ::std::vec::Vec<ethers::core::types::U256>,
            chain_id: ethers::core::types::U256,
            roots: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [150, 228, 116, 109],
                    (
                        public_amount,
                        ext_data_hash,
                        input_nullifiers,
                        output_commitments,
                        chain_id,
                        roots,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHash` (0x8d6f96ca) function"]
        pub fn get_hash(
            &self,
            public_amount: ethers::core::types::U256,
            ext_data_hash: ethers::core::types::U256,
            input_nullifiers: ::std::vec::Vec<ethers::core::types::U256>,
            output_commitments: ::std::vec::Vec<ethers::core::types::U256>,
            chain_id: ethers::core::types::U256,
            roots: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [141, 111, 150, 202],
                    (
                        public_amount,
                        ext_data_hash,
                        input_nullifiers,
                        output_commitments,
                        chain_id,
                        roots,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `helloWorld` (0xc605f76c) function"]
        pub fn hello_world(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([198, 5, 247, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyInputs` (0xdc7a2fa5) function"]
        pub fn verify_inputs(
            &self,
            a: [ethers::core::types::U256; 2usize],
            b: [[ethers::core::types::U256; 2usize]; 2usize],
            c: [ethers::core::types::U256; 2usize],
            input: [ethers::core::types::U256; 1usize],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([220, 122, 47, 165], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ChainId` event"]
        pub fn chain_id_filter(&self) -> ethers::contract::builders::Event<M, ChainIdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DataValue` event"]
        pub fn data_value_filter(&self) -> ethers::contract::builders::Event<M, DataValueFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PartialSuccess` event"]
        pub fn partial_success_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PartialSuccessFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AssemblySolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "ChainId", abi = "ChainId(uint256)")]
    pub struct ChainIdFilter {
        pub chain_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "DataValue", abi = "DataValue(bytes)")]
    pub struct DataValueFilter {
        pub data: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "PartialSuccess", abi = "PartialSuccess(string)")]
    pub struct PartialSuccessFilter {
        pub info: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AssemblySolEvents {
        ChainIdFilter(ChainIdFilter),
        DataValueFilter(DataValueFilter),
        PartialSuccessFilter(PartialSuccessFilter),
    }
    impl ethers::contract::EthLogDecode for AssemblySolEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ChainIdFilter::decode_log(log) {
                return Ok(AssemblySolEvents::ChainIdFilter(decoded));
            }
            if let Ok(decoded) = DataValueFilter::decode_log(log) {
                return Ok(AssemblySolEvents::DataValueFilter(decoded));
            }
            if let Ok(decoded) = PartialSuccessFilter::decode_log(log) {
                return Ok(AssemblySolEvents::PartialSuccessFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AssemblySolEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AssemblySolEvents::ChainIdFilter(element) => element.fmt(f),
                AssemblySolEvents::DataValueFilter(element) => element.fmt(f),
                AssemblySolEvents::PartialSuccessFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `BYTE_SIZE_MAX_EDGE_1`function with signature `BYTE_SIZE_MAX_EDGE_1()` and selector `[139, 80, 34, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BYTE_SIZE_MAX_EDGE_1", abi = "BYTE_SIZE_MAX_EDGE_1()")]
    pub struct ByteSizeMaxEdge1Call;
    #[doc = "Container type for all input parameters for the `SNARK_FIELD`function with signature `SNARK_FIELD()` and selector `[33, 141, 242, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "SNARK_FIELD", abi = "SNARK_FIELD()")]
    pub struct SnarkFieldCall;
    #[doc = "Container type for all input parameters for the `getData`function with signature `getData(uint256,uint256,uint256[],uint256[],uint256,bytes32[])` and selector `[150, 228, 116, 109]`"]
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
        name = "getData",
        abi = "getData(uint256,uint256,uint256[],uint256[],uint256,bytes32[])"
    )]
    pub struct GetDataCall {
        pub public_amount: ethers::core::types::U256,
        pub ext_data_hash: ethers::core::types::U256,
        pub input_nullifiers: ::std::vec::Vec<ethers::core::types::U256>,
        pub output_commitments: ::std::vec::Vec<ethers::core::types::U256>,
        pub chain_id: ethers::core::types::U256,
        pub roots: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `getHash`function with signature `getHash(uint256,uint256,uint256[],uint256[],uint256,bytes32[])` and selector `[141, 111, 150, 202]`"]
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
        name = "getHash",
        abi = "getHash(uint256,uint256,uint256[],uint256[],uint256,bytes32[])"
    )]
    pub struct GetHashCall {
        pub public_amount: ethers::core::types::U256,
        pub ext_data_hash: ethers::core::types::U256,
        pub input_nullifiers: ::std::vec::Vec<ethers::core::types::U256>,
        pub output_commitments: ::std::vec::Vec<ethers::core::types::U256>,
        pub chain_id: ethers::core::types::U256,
        pub roots: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `helloWorld`function with signature `helloWorld()` and selector `[198, 5, 247, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "helloWorld", abi = "helloWorld()")]
    pub struct HelloWorldCall;
    #[doc = "Container type for all input parameters for the `verifyInputs`function with signature `verifyInputs(uint256[2],uint256[2][2],uint256[2],uint256[1])` and selector `[220, 122, 47, 165]`"]
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
        name = "verifyInputs",
        abi = "verifyInputs(uint256[2],uint256[2][2],uint256[2],uint256[1])"
    )]
    pub struct VerifyInputsCall {
        pub a: [ethers::core::types::U256; 2usize],
        pub b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub c: [ethers::core::types::U256; 2usize],
        pub input: [ethers::core::types::U256; 1usize],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AssemblySolCalls {
        ByteSizeMaxEdge1(ByteSizeMaxEdge1Call),
        SnarkField(SnarkFieldCall),
        GetData(GetDataCall),
        GetHash(GetHashCall),
        HelloWorld(HelloWorldCall),
        VerifyInputs(VerifyInputsCall),
    }
    impl ethers::core::abi::AbiDecode for AssemblySolCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ByteSizeMaxEdge1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::ByteSizeMaxEdge1(decoded));
            }
            if let Ok(decoded) =
                <SnarkFieldCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::SnarkField(decoded));
            }
            if let Ok(decoded) =
                <GetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::GetData(decoded));
            }
            if let Ok(decoded) =
                <GetHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::GetHash(decoded));
            }
            if let Ok(decoded) =
                <HelloWorldCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::HelloWorld(decoded));
            }
            if let Ok(decoded) =
                <VerifyInputsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssemblySolCalls::VerifyInputs(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AssemblySolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AssemblySolCalls::ByteSizeMaxEdge1(element) => element.encode(),
                AssemblySolCalls::SnarkField(element) => element.encode(),
                AssemblySolCalls::GetData(element) => element.encode(),
                AssemblySolCalls::GetHash(element) => element.encode(),
                AssemblySolCalls::HelloWorld(element) => element.encode(),
                AssemblySolCalls::VerifyInputs(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AssemblySolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AssemblySolCalls::ByteSizeMaxEdge1(element) => element.fmt(f),
                AssemblySolCalls::SnarkField(element) => element.fmt(f),
                AssemblySolCalls::GetData(element) => element.fmt(f),
                AssemblySolCalls::GetHash(element) => element.fmt(f),
                AssemblySolCalls::HelloWorld(element) => element.fmt(f),
                AssemblySolCalls::VerifyInputs(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ByteSizeMaxEdge1Call> for AssemblySolCalls {
        fn from(var: ByteSizeMaxEdge1Call) -> Self {
            AssemblySolCalls::ByteSizeMaxEdge1(var)
        }
    }
    impl ::std::convert::From<SnarkFieldCall> for AssemblySolCalls {
        fn from(var: SnarkFieldCall) -> Self {
            AssemblySolCalls::SnarkField(var)
        }
    }
    impl ::std::convert::From<GetDataCall> for AssemblySolCalls {
        fn from(var: GetDataCall) -> Self {
            AssemblySolCalls::GetData(var)
        }
    }
    impl ::std::convert::From<GetHashCall> for AssemblySolCalls {
        fn from(var: GetHashCall) -> Self {
            AssemblySolCalls::GetHash(var)
        }
    }
    impl ::std::convert::From<HelloWorldCall> for AssemblySolCalls {
        fn from(var: HelloWorldCall) -> Self {
            AssemblySolCalls::HelloWorld(var)
        }
    }
    impl ::std::convert::From<VerifyInputsCall> for AssemblySolCalls {
        fn from(var: VerifyInputsCall) -> Self {
            AssemblySolCalls::VerifyInputs(var)
        }
    }
}
