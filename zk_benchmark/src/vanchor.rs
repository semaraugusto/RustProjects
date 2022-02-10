pub use vanchor_mod::*;
#[allow(clippy::too_many_arguments)]
pub mod vanchor_mod {
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
    #[doc = "VAnchor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VANCHOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"constructor\",\"inputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_levels\",\"type\":\"uint32\"},{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"uint8\",\"name\":\"_maxEdges\",\"type\":\"uint8\"}]},{\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"inputs\":[],\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"MAX_FEE\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"calculatePublicAmount\",\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_extAmount\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"commitments\",\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"configureLimits\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"currentRootIndex\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"edgeIndex\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"edgeList\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"filledSubtrees\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getChainId\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLastRoot\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"inputs\":[],\"outputs\":[{\"internalType\":\"struct LinkableTree.Edge[]\",\"name\":\"edges\",\"type\":\"tuple[]\",\"components\":[{\"type\":\"uint256\"},{\"type\":\"bytes32\"},{\"type\":\"uint256\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"roots\",\"type\":\"bytes32[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getProposalNonce\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"handler\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"hasEdge\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"hashLeftRight\",\"inputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"hasher\",\"inputs\":[],\"outputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"neighborChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"isKnownRoot\",\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"isSpent\",\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"isSpentArray\",\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"spent\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"isValidRoots\",\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"roots\",\"type\":\"bytes32[]\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastBalance\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"levels\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"maxEdges\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"neighborRoots\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextIndex\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nullifierHashes\",\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"register\",\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"bytes\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registerAndTransact\",\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"bytes\"}]},{\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_proofArgs\",\"type\":\"tuple\",\"components\":[{\"type\":\"bytes\"},{\"type\":\"bytes\"},{\"type\":\"bytes32[]\"},{\"type\":\"bytes32[2]\"},{\"type\":\"uint256\"},{\"type\":\"bytes32\"}]},{\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"int256\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"},{\"type\":\"bytes\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registerAndTransactWrap\",\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"bytes\"}]},{\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_proofArgs\",\"type\":\"tuple\",\"components\":[{\"type\":\"bytes\"},{\"type\":\"bytes\"},{\"type\":\"bytes32[]\"},{\"type\":\"bytes32[2]\"},{\"type\":\"uint256\"},{\"type\":\"bytes32\"}]},{\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"int256\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"},{\"type\":\"bytes\"}]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"roots\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setHandler\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"newHandler\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setVerifier\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"newVerifier\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"token\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transact\",\"inputs\":[{\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_args\",\"type\":\"tuple\",\"components\":[{\"type\":\"bytes\"},{\"type\":\"bytes\"},{\"type\":\"bytes32[]\"},{\"type\":\"bytes32[2]\"},{\"type\":\"uint256\"},{\"type\":\"bytes32\"}]},{\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"int256\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"},{\"type\":\"bytes\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transactWrap\",\"inputs\":[{\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_args\",\"type\":\"tuple\",\"components\":[{\"type\":\"bytes\"},{\"type\":\"bytes\"},{\"type\":\"bytes32[]\"},{\"type\":\"bytes32[2]\"},{\"type\":\"uint256\"},{\"type\":\"bytes32\"}]},{\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"int256\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"},{\"type\":\"bytes\"}]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"unpackProof\",\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\"}],\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"unwrapIntoNative\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unwrapIntoToken\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateEdge\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"verifier\",\"inputs\":[],\"outputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"withdrawAndUnwrap\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_minusExtAmount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"wrapAndDeposit\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_extAmount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"wrapNative\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"wrapToken\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"zeros\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\"},{\"type\":\"event\",\"name\":\"EdgeAddition\",\"inputs\":[{\"name\":\"chainID\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EdgeUpdate\",\"inputs\":[{\"name\":\"chainID\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"HashValue\",\"inputs\":[{\"name\":\"argsHash\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Insertion\",\"inputs\":[{\"name\":\"commitment\",\"type\":\"bytes32\",\"indexed\":true},{\"name\":\"leafIndex\",\"type\":\"uint32\",\"indexed\":false},{\"name\":\"timestamp\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewCommitment\",\"inputs\":[{\"name\":\"commitment\",\"type\":\"bytes32\",\"indexed\":false},{\"name\":\"index\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"encryptedOutput\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewNullifier\",\"inputs\":[{\"name\":\"nullifier\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PublicKey\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"indexed\":true},{\"name\":\"key\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct VAnchor<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for VAnchor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VAnchor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VAnchor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> VAnchor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), VANCHOR_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `FIELD_SIZE` (0x414a37ba) function"]
        pub fn field_size(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_EXT_AMOUNT` (0x7fe24ffe) function"]
        pub fn max_ext_amount(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([127, 226, 79, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FEE` (0xbc063e1a) function"]
        pub fn max_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function"]
        pub fn root_history_size(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_VALUE` (0xec732959) function"]
        pub fn zero_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculatePublicAmount` (0x2570b7b4) function"]
        pub fn calculate_public_amount(
            &self,
            ext_amount: I256,
            fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([37, 112, 183, 180], (ext_amount, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x839df945) function"]
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureLimits` (0x42d90711) function"]
        pub fn configure_limits(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            maximum_deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 217, 7, 17],
                    (minimal_withdrawal_amount, maximum_deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentNeighborRootIndex` (0x5d2d766c) function"]
        pub fn current_neighbor_root_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([93, 45, 118, 108], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentRootIndex` (0x90eeb02b) function"]
        pub fn current_root_index(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([144, 238, 176, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeExistsForChain` (0xfa731687) function"]
        pub fn edge_exists_for_chain(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 115, 22, 135], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeIndex` (0xe70ea87c) function"]
        pub fn edge_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([231, 14, 168, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeList` (0xdbc916b8) function"]
        pub fn edge_list(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                [u8; 32],
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([219, 201, 22, 184], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledSubtrees` (0xf178e47c) function"]
        pub fn filled_subtrees(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborEdges` (0x8c0d34d8) function"]
        pub fn get_latest_neighbor_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<(
                ethers::core::types::U256,
                [u8; 32],
                ethers::core::types::U256,
            )>,
        > {
            self.0
                .method_hash([140, 13, 52, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborRoots` (0x1e627617) function"]
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handler` (0xc80916d4) function"]
        pub fn handler(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasEdge` (0x92156311) function"]
        pub fn has_edge(
            &self,
            chain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 21, 99, 17], chain_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashLeftRight` (0x8ea3099e) function"]
        pub fn hash_left_right(
            &self,
            hasher: ethers::core::types::Address,
            left: [u8; 32],
            right: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 163, 9, 158], (hasher, left, right))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasher` (0xed33639f) function"]
        pub fn hasher(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([237, 51, 99, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xe4a30116) function"]
        pub fn initialize(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            maximum_deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [228, 163, 1, 22],
                    (minimal_withdrawal_amount, maximum_deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownNeighborRoot` (0x11e4dcb9) function"]
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ethers::core::types::U256,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([17, 228, 220, 185], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0x6d9833e3) function"]
        pub fn is_known_root(
            &self,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 152, 51, 227], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0xe5285dcc) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 40, 93, 204], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0x9fa12d0b) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([159, 161, 45, 11], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidRoots` (0x616e0957) function"]
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([97, 110, 9, 87], roots)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastBalance` (0x8f1c56bd) function"]
        pub fn last_balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([143, 28, 86, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `levels` (0x4ecf518b) function"]
        pub fn levels(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([78, 207, 81, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxEdges` (0x71523c32) function"]
        pub fn max_edges(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maximumDepositAmount` (0x78abb49b) function"]
        pub fn maximum_deposit_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 171, 180, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minimalWithdrawalAmount` (0x840b2791) function"]
        pub fn minimal_withdrawal_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([132, 11, 39, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `neighborRoots` (0x43e7119f) function"]
        pub fn neighbor_roots(
            &self,
            p0: ethers::core::types::U256,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextIndex` (0xfc7e9c6f) function"]
        pub fn next_index(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 126, 156, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nullifierHashes` (0x17cc915c) function"]
        pub fn nullifier_hashes(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 204, 145, 92], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `register` (0xb2bc6e0f) function"]
        pub fn register(
            &self,
            account: (ethers::core::types::Address, ethers::core::types::Bytes),
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAndTransact` (0xc9be7250) function"]
        pub fn register_and_transact(
            &self,
            account: (ethers::core::types::Address, ethers::core::types::Bytes),
            proof_args: (
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
                Vec<[u8; 32]>,
                [[u8; 32]; 2],
                ethers::core::types::U256,
                [u8; 32],
            ),
            ext_data: (
                ethers::core::types::Address,
                I256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
            ),
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 190, 114, 80], (account, proof_args, ext_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAndTransactWrap` (0xb7566a67) function"]
        pub fn register_and_transact_wrap(
            &self,
            account: (ethers::core::types::Address, ethers::core::types::Bytes),
            proof_args: (
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
                Vec<[u8; 32]>,
                [[u8; 32]; 2],
                ethers::core::types::U256,
                [u8; 32],
            ),
            ext_data: (
                ethers::core::types::Address,
                I256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
            ),
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 86, 106, 103],
                    (account, proof_args, ext_data, token_address),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0x72c1ad03) function"]
        pub fn set_handler(
            &self,
            new_handler: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (new_handler, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVerifier` (0xa0d192f5) function"]
        pub fn set_verifier(
            &self,
            new_verifier: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 209, 146, 245], (new_verifier, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transact` (0x9bbca3a9) function"]
        pub fn transact(
            &self,
            args: (
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
                Vec<[u8; 32]>,
                [[u8; 32]; 2],
                ethers::core::types::U256,
                [u8; 32],
            ),
            ext_data: (
                ethers::core::types::Address,
                I256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
            ),
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 188, 163, 169], (args, ext_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transactWrap` (0x4167bb1e) function"]
        pub fn transact_wrap(
            &self,
            args: (
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
                Vec<[u8; 32]>,
                [[u8; 32]; 2],
                ethers::core::types::U256,
                [u8; 32],
            ),
            ext_data: (
                ethers::core::types::Address,
                I256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
            ),
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 103, 187, 30], (args, ext_data, token_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpackProof` (0xf5ab0dd6) function"]
        pub fn unpack_proof(
            &self,
            proof: [ethers::core::types::U256; 8usize],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [ethers::core::types::U256; 2usize],
                [[ethers::core::types::U256; 2usize]; 2usize],
                [ethers::core::types::U256; 2usize],
            ),
        > {
            self.0
                .method_hash([245, 171, 13, 214], proof)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapIntoNative` (0x9ff80063) function"]
        pub fn unwrap_into_native(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 248, 0, 99], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapIntoToken` (0x4f401241) function"]
        pub fn unwrap_into_token(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 64, 18, 65], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateEdge` (0x44347ba9) function"]
        pub fn update_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            leaf_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 52, 123, 169], (source_chain_id, root, leaf_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifier` (0x2b7ac3f3) function"]
        pub fn verifier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0x95c87d1a) function"]
        pub fn withdraw_and_unwrap(
            &self,
            token_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            minus_ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [149, 200, 125, 26],
                    (token_address, recipient, minus_ext_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapAndDeposit` (0x2063e3d3) function"]
        pub fn wrap_and_deposit(
            &self,
            token_address: ethers::core::types::Address,
            ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 99, 227, 211], (token_address, ext_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapNative` (0x6ad481f3) function"]
        pub fn wrap_native(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 212, 129, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapToken` (0x460b53e3) function"]
        pub fn wrap_token(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 11, 83, 227], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `zeros` (0xe8295588) function"]
        pub fn zeros(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 41, 85, 136], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `EdgeAddition` event"]
        pub fn edge_addition_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EdgeAdditionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EdgeUpdate` event"]
        pub fn edge_update_filter(&self) -> ethers::contract::builders::Event<M, EdgeUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HashValue` event"]
        pub fn hash_value_filter(&self) -> ethers::contract::builders::Event<M, HashValueFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Insertion` event"]
        pub fn insertion_filter(&self) -> ethers::contract::builders::Event<M, InsertionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCommitment` event"]
        pub fn new_commitment_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCommitmentFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewNullifier` event"]
        pub fn new_nullifier_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewNullifierFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PublicKey` event"]
        pub fn public_key_filter(&self) -> ethers::contract::builders::Event<M, PublicKeyFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, VAnchorEvents> {
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
    #[ethevent(name = "EdgeAddition", abi = "EdgeAddition(uint256,uint256,bytes32)")]
    pub struct EdgeAdditionFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
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
    #[ethevent(name = "EdgeUpdate", abi = "EdgeUpdate(uint256,uint256,bytes32)")]
    pub struct EdgeUpdateFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
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
    #[ethevent(name = "HashValue", abi = "HashValue(bytes32)")]
    pub struct HashValueFilter {
        pub args_hash: [u8; 32],
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
    #[ethevent(name = "Insertion", abi = "Insertion(bytes32,uint32,uint256)")]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub leaf_index: u32,
        pub timestamp: ethers::core::types::U256,
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
    #[ethevent(name = "NewCommitment", abi = "NewCommitment(bytes32,uint256,bytes)")]
    pub struct NewCommitmentFilter {
        pub commitment: [u8; 32],
        pub index: ethers::core::types::U256,
        pub encrypted_output: ethers::core::types::Bytes,
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
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(bytes32)")]
    pub struct NewNullifierFilter {
        pub nullifier: [u8; 32],
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
    #[ethevent(name = "PublicKey", abi = "PublicKey(address,bytes)")]
    pub struct PublicKeyFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub key: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VAnchorEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        HashValueFilter(HashValueFilter),
        InsertionFilter(InsertionFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
    }
    impl ethers::contract::EthLogDecode for VAnchorEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(VAnchorEvents::EdgeAdditionFilter(decoded));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(VAnchorEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) = HashValueFilter::decode_log(log) {
                return Ok(VAnchorEvents::HashValueFilter(decoded));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(VAnchorEvents::InsertionFilter(decoded));
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(VAnchorEvents::NewCommitmentFilter(decoded));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(VAnchorEvents::NewNullifierFilter(decoded));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(VAnchorEvents::PublicKeyFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VAnchorEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAnchorEvents::EdgeAdditionFilter(element) => element.fmt(f),
                VAnchorEvents::EdgeUpdateFilter(element) => element.fmt(f),
                VAnchorEvents::HashValueFilter(element) => element.fmt(f),
                VAnchorEvents::InsertionFilter(element) => element.fmt(f),
                VAnchorEvents::NewCommitmentFilter(element) => element.fmt(f),
                VAnchorEvents::NewNullifierFilter(element) => element.fmt(f),
                VAnchorEvents::PublicKeyFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FIELD_SIZE`function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "FIELD_SIZE", abi = "FIELD_SIZE()")]
    pub struct FieldSizeCall;
    #[doc = "Container type for all input parameters for the `MAX_EXT_AMOUNT`function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_EXT_AMOUNT", abi = "MAX_EXT_AMOUNT()")]
    pub struct MaxExtAmountCall;
    #[doc = "Container type for all input parameters for the `MAX_FEE`function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    #[doc = "Container type for all input parameters for the `ROOT_HISTORY_SIZE`function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ROOT_HISTORY_SIZE", abi = "ROOT_HISTORY_SIZE()")]
    pub struct RootHistorySizeCall;
    #[doc = "Container type for all input parameters for the `ZERO_VALUE`function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    #[doc = "Container type for all input parameters for the `calculatePublicAmount`function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
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
        name = "calculatePublicAmount",
        abi = "calculatePublicAmount(int256,uint256)"
    )]
    pub struct CalculatePublicAmountCall {
        pub ext_amount: I256,
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `commitments`function with signature `commitments(bytes32)` and selector `[131, 157, 249, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "commitments", abi = "commitments(bytes32)")]
    pub struct CommitmentsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `configureLimits`function with signature `configureLimits(uint256,uint256)` and selector `[66, 217, 7, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "configureLimits", abi = "configureLimits(uint256,uint256)")]
    pub struct ConfigureLimitsCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub maximum_deposit_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `currentNeighborRootIndex`function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
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
        name = "currentNeighborRootIndex",
        abi = "currentNeighborRootIndex(uint256)"
    )]
    pub struct CurrentNeighborRootIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `currentRootIndex`function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "currentRootIndex", abi = "currentRootIndex()")]
    pub struct CurrentRootIndexCall;
    #[doc = "Container type for all input parameters for the `edgeExistsForChain`function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "edgeExistsForChain", abi = "edgeExistsForChain(uint256)")]
    pub struct EdgeExistsForChainCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeIndex`function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "edgeIndex", abi = "edgeIndex(uint256)")]
    pub struct EdgeIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeList`function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "edgeList", abi = "edgeList(uint256)")]
    pub struct EdgeListCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `filledSubtrees`function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getChainId`function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getLastRoot`function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborEdges`function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLatestNeighborEdges", abi = "getLatestNeighborEdges()")]
    pub struct GetLatestNeighborEdgesCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborRoots`function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLatestNeighborRoots", abi = "getLatestNeighborRoots()")]
    pub struct GetLatestNeighborRootsCall;
    #[doc = "Container type for all input parameters for the `getProposalNonce`function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    #[doc = "Container type for all input parameters for the `handler`function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    #[doc = "Container type for all input parameters for the `hasEdge`function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hasEdge", abi = "hasEdge(uint256)")]
    pub struct HasEdgeCall {
        pub chain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hashLeftRight`function with signature `hashLeftRight(address,bytes32,bytes32)` and selector `[142, 163, 9, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(address,bytes32,bytes32)")]
    pub struct HashLeftRightCall {
        pub hasher: ethers::core::types::Address,
        pub left: [u8; 32],
        pub right: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `hasher`function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hasher", abi = "hasher()")]
    pub struct HasherCall;
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(uint256,uint256)` and selector `[228, 163, 1, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256)")]
    pub struct InitializeCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub maximum_deposit_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isKnownNeighborRoot`function with signature `isKnownNeighborRoot(uint256,bytes32)` and selector `[17, 228, 220, 185]`"]
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
        name = "isKnownNeighborRoot",
        abi = "isKnownNeighborRoot(uint256,bytes32)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isKnownRoot`function with signature `isKnownRoot(bytes32)` and selector `[109, 152, 51, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(bytes32)")]
    pub struct IsKnownRootCall {
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isSpent`function with signature `isSpent(bytes32)` and selector `[229, 40, 93, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isSpent", abi = "isSpent(bytes32)")]
    pub struct IsSpentCall {
        pub nullifier_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isSpentArray`function with signature `isSpentArray(bytes32[])` and selector `[159, 161, 45, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(bytes32[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `isValidRoots`function with signature `isValidRoots(bytes32[])` and selector `[97, 110, 9, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(bytes32[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `lastBalance`function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lastBalance", abi = "lastBalance()")]
    pub struct LastBalanceCall;
    #[doc = "Container type for all input parameters for the `levels`function with signature `levels()` and selector `[78, 207, 81, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "levels", abi = "levels()")]
    pub struct LevelsCall;
    #[doc = "Container type for all input parameters for the `maxEdges`function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxEdges", abi = "maxEdges()")]
    pub struct MaxEdgesCall;
    #[doc = "Container type for all input parameters for the `maximumDepositAmount`function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maximumDepositAmount", abi = "maximumDepositAmount()")]
    pub struct MaximumDepositAmountCall;
    #[doc = "Container type for all input parameters for the `minimalWithdrawalAmount`function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "minimalWithdrawalAmount", abi = "minimalWithdrawalAmount()")]
    pub struct MinimalWithdrawalAmountCall;
    #[doc = "Container type for all input parameters for the `neighborRoots`function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "neighborRoots", abi = "neighborRoots(uint256,uint32)")]
    pub struct NeighborRootsCall(pub ethers::core::types::U256, pub u32);
    #[doc = "Container type for all input parameters for the `nextIndex`function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nextIndex", abi = "nextIndex()")]
    pub struct NextIndexCall;
    #[doc = "Container type for all input parameters for the `nullifierHashes`function with signature `nullifierHashes(bytes32)` and selector `[23, 204, 145, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(bytes32)")]
    pub struct NullifierHashesCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `register`function with signature `register((address,bytes))` and selector `[178, 188, 110, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "register", abi = "register((address,bytes))")]
    pub struct RegisterCall {
        pub account: (ethers::core::types::Address, ethers::core::types::Bytes),
    }
    #[doc = "Container type for all input parameters for the `registerAndTransact`function with signature `registerAndTransact((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))` and selector `[201, 190, 114, 80]`"]
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
        name = "registerAndTransact",
        abi = "registerAndTransact((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))"
    )]
    pub struct RegisterAndTransactCall {
        pub account: (ethers::core::types::Address, ethers::core::types::Bytes),
        pub proof_args: (
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
            Vec<[u8; 32]>,
            [[u8; 32]; 2],
            ethers::core::types::U256,
            [u8; 32],
        ),
        pub ext_data: (
            ethers::core::types::Address,
            I256,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
        ),
    }
    #[doc = "Container type for all input parameters for the `registerAndTransactWrap`function with signature `registerAndTransactWrap((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)` and selector `[183, 86, 106, 103]`"]
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
        name = "registerAndTransactWrap",
        abi = "registerAndTransactWrap((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)"
    )]
    pub struct RegisterAndTransactWrapCall {
        pub account: (ethers::core::types::Address, ethers::core::types::Bytes),
        pub proof_args: (
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
            Vec<[u8; 32]>,
            [[u8; 32]; 2],
            ethers::core::types::U256,
            [u8; 32],
        ),
        pub ext_data: (
            ethers::core::types::Address,
            I256,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
        ),
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `roots`function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setHandler`function with signature `setHandler(address,uint32)` and selector `[114, 193, 173, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub new_handler: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setVerifier`function with signature `setVerifier(address,uint32)` and selector `[160, 209, 146, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setVerifier", abi = "setVerifier(address,uint32)")]
    pub struct SetVerifierCall {
        pub new_verifier: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `transact`function with signature `transact((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))` and selector `[155, 188, 163, 169]`"]
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
        name = "transact",
        abi = "transact((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))"
    )]
    pub struct TransactCall {
        pub args: (
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
            Vec<[u8; 32]>,
            [[u8; 32]; 2],
            ethers::core::types::U256,
            [u8; 32],
        ),
        pub ext_data: (
            ethers::core::types::Address,
            I256,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
        ),
    }
    #[doc = "Container type for all input parameters for the `transactWrap`function with signature `transactWrap((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)` and selector `[65, 103, 187, 30]`"]
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
        name = "transactWrap",
        abi = "transactWrap((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)"
    )]
    pub struct TransactWrapCall {
        pub args: (
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
            Vec<[u8; 32]>,
            [[u8; 32]; 2],
            ethers::core::types::U256,
            [u8; 32],
        ),
        pub ext_data: (
            ethers::core::types::Address,
            I256,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::Bytes,
            ethers::core::types::Bytes,
        ),
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unpackProof`function with signature `unpackProof(uint256[8])` and selector `[245, 171, 13, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unpackProof", abi = "unpackProof(uint256[8])")]
    pub struct UnpackProofCall {
        pub proof: [ethers::core::types::U256; 8usize],
    }
    #[doc = "Container type for all input parameters for the `unwrapIntoNative`function with signature `unwrapIntoNative(address,uint256)` and selector `[159, 248, 0, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unwrapIntoNative", abi = "unwrapIntoNative(address,uint256)")]
    pub struct UnwrapIntoNativeCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapIntoToken`function with signature `unwrapIntoToken(address,uint256)` and selector `[79, 64, 18, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unwrapIntoToken", abi = "unwrapIntoToken(address,uint256)")]
    pub struct UnwrapIntoTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateEdge`function with signature `updateEdge(uint256,bytes32,uint256)` and selector `[68, 52, 123, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,bytes32,uint256)")]
    pub struct UpdateEdgeCall {
        pub source_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub leaf_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `verifier`function with signature `verifier()` and selector `[43, 122, 195, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap`function with signature `withdrawAndUnwrap(address,address,uint256)` and selector `[149, 200, 125, 26]`"]
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
        name = "withdrawAndUnwrap",
        abi = "withdrawAndUnwrap(address,address,uint256)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub token_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub minus_ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapAndDeposit`function with signature `wrapAndDeposit(address,uint256)` and selector `[32, 99, 227, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapAndDeposit", abi = "wrapAndDeposit(address,uint256)")]
    pub struct WrapAndDepositCall {
        pub token_address: ethers::core::types::Address,
        pub ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapNative`function with signature `wrapNative()` and selector `[106, 212, 129, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapNative", abi = "wrapNative()")]
    pub struct WrapNativeCall;
    #[doc = "Container type for all input parameters for the `wrapToken`function with signature `wrapToken(address,uint256)` and selector `[70, 11, 83, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapToken", abi = "wrapToken(address,uint256)")]
    pub struct WrapTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `zeros`function with signature `zeros(uint256)` and selector `[232, 41, 85, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "zeros", abi = "zeros(uint256)")]
    pub struct ZerosCall {
        pub i: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VAnchorCalls {
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        CalculatePublicAmount(CalculatePublicAmountCall),
        Commitments(CommitmentsCall),
        ConfigureLimits(ConfigureLimitsCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        CurrentRootIndex(CurrentRootIndexCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetChainId(GetChainIdCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborEdges(GetLatestNeighborEdgesCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetProposalNonce(GetProposalNonceCall),
        Handler(HandlerCall),
        HasEdge(HasEdgeCall),
        HashLeftRight(HashLeftRightCall),
        Hasher(HasherCall),
        Initialize(InitializeCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        LastBalance(LastBalanceCall),
        Levels(LevelsCall),
        MaxEdges(MaxEdgesCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimalWithdrawalAmount(MinimalWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NextIndex(NextIndexCall),
        NullifierHashes(NullifierHashesCall),
        Register(RegisterCall),
        RegisterAndTransact(RegisterAndTransactCall),
        RegisterAndTransactWrap(RegisterAndTransactWrapCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        SetVerifier(SetVerifierCall),
        Token(TokenCall),
        Transact(TransactCall),
        TransactWrap(TransactWrapCall),
        UnpackProof(UnpackProofCall),
        UnwrapIntoNative(UnwrapIntoNativeCall),
        UnwrapIntoToken(UnwrapIntoTokenCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        WrapAndDeposit(WrapAndDepositCall),
        WrapNative(WrapNativeCall),
        WrapToken(WrapTokenCall),
        Zeros(ZerosCall),
    }
    impl ethers::core::abi::AbiDecode for VAnchorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::MaxExtAmount(decoded));
            }
            if let Ok(decoded) = <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <CalculatePublicAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::CalculatePublicAmount(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Commitments(decoded));
            }
            if let Ok(decoded) =
                <ConfigureLimitsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::ConfigureLimits(decoded));
            }
            if let Ok(decoded) =
                <CurrentNeighborRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorCalls::CurrentNeighborRootIndex(decoded));
            }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::CurrentRootIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::EdgeExistsForChain(decoded));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::GetLastRoot(decoded));
            }
            if let Ok(decoded) =
                <GetLatestNeighborEdgesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::GetLatestNeighborEdges(decoded));
            }
            if let Ok(decoded) =
                <GetLatestNeighborRootsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::GetLatestNeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) = <HasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Hasher(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsKnownNeighborRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::IsKnownNeighborRoot(decoded));
            }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::LastBalance(decoded));
            }
            if let Ok(decoded) = <LevelsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Levels(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::MaxEdges(decoded));
            }
            if let Ok(decoded) =
                <MaximumDepositAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::MaximumDepositAmount(decoded));
            }
            if let Ok(decoded) =
                <MinimalWithdrawalAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::MinimalWithdrawalAmount(decoded));
            }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Register(decoded));
            }
            if let Ok(decoded) =
                <RegisterAndTransactCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::RegisterAndTransact(decoded));
            }
            if let Ok(decoded) =
                <RegisterAndTransactWrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::RegisterAndTransactWrap(decoded));
            }
            if let Ok(decoded) = <RootsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::SetVerifier(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TransactCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Transact(decoded));
            }
            if let Ok(decoded) =
                <TransactWrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::TransactWrap(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoNativeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::UnwrapIntoNative(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::UnwrapIntoToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <VerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Verifier(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::WithdrawAndUnwrap(decoded));
            }
            if let Ok(decoded) =
                <WrapAndDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::WrapAndDeposit(decoded));
            }
            if let Ok(decoded) =
                <WrapNativeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::WrapNative(decoded));
            }
            if let Ok(decoded) =
                <WrapTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::WrapToken(decoded));
            }
            if let Ok(decoded) = <ZerosCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAnchorCalls::Zeros(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VAnchorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VAnchorCalls::FieldSize(element) => element.encode(),
                VAnchorCalls::MaxExtAmount(element) => element.encode(),
                VAnchorCalls::MaxFee(element) => element.encode(),
                VAnchorCalls::RootHistorySize(element) => element.encode(),
                VAnchorCalls::ZeroValue(element) => element.encode(),
                VAnchorCalls::CalculatePublicAmount(element) => element.encode(),
                VAnchorCalls::Commitments(element) => element.encode(),
                VAnchorCalls::ConfigureLimits(element) => element.encode(),
                VAnchorCalls::CurrentNeighborRootIndex(element) => element.encode(),
                VAnchorCalls::CurrentRootIndex(element) => element.encode(),
                VAnchorCalls::EdgeExistsForChain(element) => element.encode(),
                VAnchorCalls::EdgeIndex(element) => element.encode(),
                VAnchorCalls::EdgeList(element) => element.encode(),
                VAnchorCalls::FilledSubtrees(element) => element.encode(),
                VAnchorCalls::GetChainId(element) => element.encode(),
                VAnchorCalls::GetLastRoot(element) => element.encode(),
                VAnchorCalls::GetLatestNeighborEdges(element) => element.encode(),
                VAnchorCalls::GetLatestNeighborRoots(element) => element.encode(),
                VAnchorCalls::GetProposalNonce(element) => element.encode(),
                VAnchorCalls::Handler(element) => element.encode(),
                VAnchorCalls::HasEdge(element) => element.encode(),
                VAnchorCalls::HashLeftRight(element) => element.encode(),
                VAnchorCalls::Hasher(element) => element.encode(),
                VAnchorCalls::Initialize(element) => element.encode(),
                VAnchorCalls::IsKnownNeighborRoot(element) => element.encode(),
                VAnchorCalls::IsKnownRoot(element) => element.encode(),
                VAnchorCalls::IsSpent(element) => element.encode(),
                VAnchorCalls::IsSpentArray(element) => element.encode(),
                VAnchorCalls::IsValidRoots(element) => element.encode(),
                VAnchorCalls::LastBalance(element) => element.encode(),
                VAnchorCalls::Levels(element) => element.encode(),
                VAnchorCalls::MaxEdges(element) => element.encode(),
                VAnchorCalls::MaximumDepositAmount(element) => element.encode(),
                VAnchorCalls::MinimalWithdrawalAmount(element) => element.encode(),
                VAnchorCalls::NeighborRoots(element) => element.encode(),
                VAnchorCalls::NextIndex(element) => element.encode(),
                VAnchorCalls::NullifierHashes(element) => element.encode(),
                VAnchorCalls::Register(element) => element.encode(),
                VAnchorCalls::RegisterAndTransact(element) => element.encode(),
                VAnchorCalls::RegisterAndTransactWrap(element) => element.encode(),
                VAnchorCalls::Roots(element) => element.encode(),
                VAnchorCalls::SetHandler(element) => element.encode(),
                VAnchorCalls::SetVerifier(element) => element.encode(),
                VAnchorCalls::Token(element) => element.encode(),
                VAnchorCalls::Transact(element) => element.encode(),
                VAnchorCalls::TransactWrap(element) => element.encode(),
                VAnchorCalls::UnpackProof(element) => element.encode(),
                VAnchorCalls::UnwrapIntoNative(element) => element.encode(),
                VAnchorCalls::UnwrapIntoToken(element) => element.encode(),
                VAnchorCalls::UpdateEdge(element) => element.encode(),
                VAnchorCalls::Verifier(element) => element.encode(),
                VAnchorCalls::WithdrawAndUnwrap(element) => element.encode(),
                VAnchorCalls::WrapAndDeposit(element) => element.encode(),
                VAnchorCalls::WrapNative(element) => element.encode(),
                VAnchorCalls::WrapToken(element) => element.encode(),
                VAnchorCalls::Zeros(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VAnchorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAnchorCalls::FieldSize(element) => element.fmt(f),
                VAnchorCalls::MaxExtAmount(element) => element.fmt(f),
                VAnchorCalls::MaxFee(element) => element.fmt(f),
                VAnchorCalls::RootHistorySize(element) => element.fmt(f),
                VAnchorCalls::ZeroValue(element) => element.fmt(f),
                VAnchorCalls::CalculatePublicAmount(element) => element.fmt(f),
                VAnchorCalls::Commitments(element) => element.fmt(f),
                VAnchorCalls::ConfigureLimits(element) => element.fmt(f),
                VAnchorCalls::CurrentNeighborRootIndex(element) => element.fmt(f),
                VAnchorCalls::CurrentRootIndex(element) => element.fmt(f),
                VAnchorCalls::EdgeExistsForChain(element) => element.fmt(f),
                VAnchorCalls::EdgeIndex(element) => element.fmt(f),
                VAnchorCalls::EdgeList(element) => element.fmt(f),
                VAnchorCalls::FilledSubtrees(element) => element.fmt(f),
                VAnchorCalls::GetChainId(element) => element.fmt(f),
                VAnchorCalls::GetLastRoot(element) => element.fmt(f),
                VAnchorCalls::GetLatestNeighborEdges(element) => element.fmt(f),
                VAnchorCalls::GetLatestNeighborRoots(element) => element.fmt(f),
                VAnchorCalls::GetProposalNonce(element) => element.fmt(f),
                VAnchorCalls::Handler(element) => element.fmt(f),
                VAnchorCalls::HasEdge(element) => element.fmt(f),
                VAnchorCalls::HashLeftRight(element) => element.fmt(f),
                VAnchorCalls::Hasher(element) => element.fmt(f),
                VAnchorCalls::Initialize(element) => element.fmt(f),
                VAnchorCalls::IsKnownNeighborRoot(element) => element.fmt(f),
                VAnchorCalls::IsKnownRoot(element) => element.fmt(f),
                VAnchorCalls::IsSpent(element) => element.fmt(f),
                VAnchorCalls::IsSpentArray(element) => element.fmt(f),
                VAnchorCalls::IsValidRoots(element) => element.fmt(f),
                VAnchorCalls::LastBalance(element) => element.fmt(f),
                VAnchorCalls::Levels(element) => element.fmt(f),
                VAnchorCalls::MaxEdges(element) => element.fmt(f),
                VAnchorCalls::MaximumDepositAmount(element) => element.fmt(f),
                VAnchorCalls::MinimalWithdrawalAmount(element) => element.fmt(f),
                VAnchorCalls::NeighborRoots(element) => element.fmt(f),
                VAnchorCalls::NextIndex(element) => element.fmt(f),
                VAnchorCalls::NullifierHashes(element) => element.fmt(f),
                VAnchorCalls::Register(element) => element.fmt(f),
                VAnchorCalls::RegisterAndTransact(element) => element.fmt(f),
                VAnchorCalls::RegisterAndTransactWrap(element) => element.fmt(f),
                VAnchorCalls::Roots(element) => element.fmt(f),
                VAnchorCalls::SetHandler(element) => element.fmt(f),
                VAnchorCalls::SetVerifier(element) => element.fmt(f),
                VAnchorCalls::Token(element) => element.fmt(f),
                VAnchorCalls::Transact(element) => element.fmt(f),
                VAnchorCalls::TransactWrap(element) => element.fmt(f),
                VAnchorCalls::UnpackProof(element) => element.fmt(f),
                VAnchorCalls::UnwrapIntoNative(element) => element.fmt(f),
                VAnchorCalls::UnwrapIntoToken(element) => element.fmt(f),
                VAnchorCalls::UpdateEdge(element) => element.fmt(f),
                VAnchorCalls::Verifier(element) => element.fmt(f),
                VAnchorCalls::WithdrawAndUnwrap(element) => element.fmt(f),
                VAnchorCalls::WrapAndDeposit(element) => element.fmt(f),
                VAnchorCalls::WrapNative(element) => element.fmt(f),
                VAnchorCalls::WrapToken(element) => element.fmt(f),
                VAnchorCalls::Zeros(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FieldSizeCall> for VAnchorCalls {
        fn from(var: FieldSizeCall) -> Self {
            VAnchorCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<MaxExtAmountCall> for VAnchorCalls {
        fn from(var: MaxExtAmountCall) -> Self {
            VAnchorCalls::MaxExtAmount(var)
        }
    }
    impl ::std::convert::From<MaxFeeCall> for VAnchorCalls {
        fn from(var: MaxFeeCall) -> Self {
            VAnchorCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for VAnchorCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            VAnchorCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for VAnchorCalls {
        fn from(var: ZeroValueCall) -> Self {
            VAnchorCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<CalculatePublicAmountCall> for VAnchorCalls {
        fn from(var: CalculatePublicAmountCall) -> Self {
            VAnchorCalls::CalculatePublicAmount(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for VAnchorCalls {
        fn from(var: CommitmentsCall) -> Self {
            VAnchorCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ConfigureLimitsCall> for VAnchorCalls {
        fn from(var: ConfigureLimitsCall) -> Self {
            VAnchorCalls::ConfigureLimits(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall> for VAnchorCalls {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            VAnchorCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall> for VAnchorCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            VAnchorCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for VAnchorCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            VAnchorCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for VAnchorCalls {
        fn from(var: EdgeIndexCall) -> Self {
            VAnchorCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for VAnchorCalls {
        fn from(var: EdgeListCall) -> Self {
            VAnchorCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for VAnchorCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            VAnchorCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for VAnchorCalls {
        fn from(var: GetChainIdCall) -> Self {
            VAnchorCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for VAnchorCalls {
        fn from(var: GetLastRootCall) -> Self {
            VAnchorCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall> for VAnchorCalls {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            VAnchorCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall> for VAnchorCalls {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            VAnchorCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall> for VAnchorCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            VAnchorCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for VAnchorCalls {
        fn from(var: HandlerCall) -> Self {
            VAnchorCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for VAnchorCalls {
        fn from(var: HasEdgeCall) -> Self {
            VAnchorCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for VAnchorCalls {
        fn from(var: HashLeftRightCall) -> Self {
            VAnchorCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for VAnchorCalls {
        fn from(var: HasherCall) -> Self {
            VAnchorCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VAnchorCalls {
        fn from(var: InitializeCall) -> Self {
            VAnchorCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall> for VAnchorCalls {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            VAnchorCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for VAnchorCalls {
        fn from(var: IsKnownRootCall) -> Self {
            VAnchorCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for VAnchorCalls {
        fn from(var: IsSpentCall) -> Self {
            VAnchorCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for VAnchorCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            VAnchorCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for VAnchorCalls {
        fn from(var: IsValidRootsCall) -> Self {
            VAnchorCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LastBalanceCall> for VAnchorCalls {
        fn from(var: LastBalanceCall) -> Self {
            VAnchorCalls::LastBalance(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for VAnchorCalls {
        fn from(var: LevelsCall) -> Self {
            VAnchorCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaxEdgesCall> for VAnchorCalls {
        fn from(var: MaxEdgesCall) -> Self {
            VAnchorCalls::MaxEdges(var)
        }
    }
    impl ::std::convert::From<MaximumDepositAmountCall> for VAnchorCalls {
        fn from(var: MaximumDepositAmountCall) -> Self {
            VAnchorCalls::MaximumDepositAmount(var)
        }
    }
    impl ::std::convert::From<MinimalWithdrawalAmountCall> for VAnchorCalls {
        fn from(var: MinimalWithdrawalAmountCall) -> Self {
            VAnchorCalls::MinimalWithdrawalAmount(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for VAnchorCalls {
        fn from(var: NeighborRootsCall) -> Self {
            VAnchorCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for VAnchorCalls {
        fn from(var: NextIndexCall) -> Self {
            VAnchorCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for VAnchorCalls {
        fn from(var: NullifierHashesCall) -> Self {
            VAnchorCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<RegisterCall> for VAnchorCalls {
        fn from(var: RegisterCall) -> Self {
            VAnchorCalls::Register(var)
        }
    }
    impl ::std::convert::From<RegisterAndTransactCall> for VAnchorCalls {
        fn from(var: RegisterAndTransactCall) -> Self {
            VAnchorCalls::RegisterAndTransact(var)
        }
    }
    impl ::std::convert::From<RegisterAndTransactWrapCall> for VAnchorCalls {
        fn from(var: RegisterAndTransactWrapCall) -> Self {
            VAnchorCalls::RegisterAndTransactWrap(var)
        }
    }
    impl ::std::convert::From<RootsCall> for VAnchorCalls {
        fn from(var: RootsCall) -> Self {
            VAnchorCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for VAnchorCalls {
        fn from(var: SetHandlerCall) -> Self {
            VAnchorCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<SetVerifierCall> for VAnchorCalls {
        fn from(var: SetVerifierCall) -> Self {
            VAnchorCalls::SetVerifier(var)
        }
    }
    impl ::std::convert::From<TokenCall> for VAnchorCalls {
        fn from(var: TokenCall) -> Self {
            VAnchorCalls::Token(var)
        }
    }
    impl ::std::convert::From<TransactCall> for VAnchorCalls {
        fn from(var: TransactCall) -> Self {
            VAnchorCalls::Transact(var)
        }
    }
    impl ::std::convert::From<TransactWrapCall> for VAnchorCalls {
        fn from(var: TransactWrapCall) -> Self {
            VAnchorCalls::TransactWrap(var)
        }
    }
    impl ::std::convert::From<UnpackProofCall> for VAnchorCalls {
        fn from(var: UnpackProofCall) -> Self {
            VAnchorCalls::UnpackProof(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoNativeCall> for VAnchorCalls {
        fn from(var: UnwrapIntoNativeCall) -> Self {
            VAnchorCalls::UnwrapIntoNative(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoTokenCall> for VAnchorCalls {
        fn from(var: UnwrapIntoTokenCall) -> Self {
            VAnchorCalls::UnwrapIntoToken(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for VAnchorCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            VAnchorCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<VerifierCall> for VAnchorCalls {
        fn from(var: VerifierCall) -> Self {
            VAnchorCalls::Verifier(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for VAnchorCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            VAnchorCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WrapAndDepositCall> for VAnchorCalls {
        fn from(var: WrapAndDepositCall) -> Self {
            VAnchorCalls::WrapAndDeposit(var)
        }
    }
    impl ::std::convert::From<WrapNativeCall> for VAnchorCalls {
        fn from(var: WrapNativeCall) -> Self {
            VAnchorCalls::WrapNative(var)
        }
    }
    impl ::std::convert::From<WrapTokenCall> for VAnchorCalls {
        fn from(var: WrapTokenCall) -> Self {
            VAnchorCalls::WrapToken(var)
        }
    }
    impl ::std::convert::From<ZerosCall> for VAnchorCalls {
        fn from(var: ZerosCall) -> Self {
            VAnchorCalls::Zeros(var)
        }
    }
}
