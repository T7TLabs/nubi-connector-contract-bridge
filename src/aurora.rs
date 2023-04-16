use aurora_engine_types::types::RawU256;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract,
};

// Define types
pub type RawAddress = [u8; 20];

#[ext_contract(ext_aurora)]
pub trait AuroraContract {
    #[result_serializer(borsh)]
    fn call(&self, #[serializer(borsh)] call_args: CallArgs);
}

// Aurora result
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransactionStatus {
    Succeed(Vec<u8>),
    Revert(Vec<u8>),
    OutOfGas,
    OutOfFund,
    OutOfOffset,
    CallTooDeep,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ResultLog {
    pub address: RawAddress,
    pub topics: Vec<RawU256>,
    pub data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SubmitResult {
    version: u8,
    pub status: TransactionStatus,
    pub gas_used: u64,
    pub logs: Vec<ResultLog>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct FunctionCallArgsV2 {
    pub contract: RawAddress,
    pub value: RawU256,
    pub input: Vec<u8>,
}

// Enum from Aurora
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum CallArgs {
    V2(FunctionCallArgsV2),
    Unused,
}
