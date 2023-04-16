/*!
Contract bridge from Near to Aurora
*/
mod aurora;
mod utils;

#[macro_use]
extern crate lazy_static;
use crate::aurora::{CallArgs, FunctionCallArgsV2};
use aurora_engine_types::types::RawU256;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId, PanicOnDefault, Promise,
};

const AURORA_BRIDGE_ADDRESS: &str = "aurora";

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ContractBridge;

#[near_bindgen]
impl ContractBridge {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self
    }

    pub fn function_call(
        &mut self,
        aurora_address: String,
        function: String,
        parameters: Vec<String>,
    ) -> Promise {
        let input = utils::solidity_function(&function, &parameters);
        let aurora_contract = utils::from_string_to_address(&aurora_address);
        let aurora_address: AccountId = AURORA_BRIDGE_ADDRESS
            .parse()
            .expect("Internal error: Aurora address is not correct");

        aurora::ext_aurora::ext(aurora_address).call(CallArgs::V2(FunctionCallArgsV2 {
            contract: aurora_contract.0,
            value: RawU256::default(),
            input,
        }))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aurora::SubmitResult;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    // @TODO: Tests in progress

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
    }

    #[test]
    fn test_output() {
        let result = [
            0x07, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x60, 0x56, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, // 0x07, 0x01, 0x64, 0x00, 0x00, 0x00, 0x08, 0xc3, 0x79, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x45, 0x31, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x92, 0x00, 0x00, 0x00, 0x00,
                  //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        let result_decoded = SubmitResult::deserialize(&mut result.as_slice());
        println!("{:?}", result_decoded);
    }
}
