### Proxy Contract

A smart contract that facilitates cross-contract communication for Near Dapps/Users with Aurora smart contracts.

### Necessity

A classic example is managing Near User's assets (NEP141), bridged using either Rainbow bridge interface or programatically within a Near Dapp, on Aurora (ERC20).
Managing the assets requires Near to Aurora cross-contract calls that would trigger either Read-only or Write (state changing) methods of a corresponding ERC20 contract.

To deal with both type of methods, a proper construction of a cross-contract call combining address of a recipient contract and an input data properly hashing and concatenating called method's name and its input parameters is required.
The proposed smart contract eases such cross-contract communication for Near smart contract developers by taking care of properly contructing the calls.
Developers are required only to provide an address of an Aurora smart contract, method's name that needs to be called and specify its parameters, while verctorization of the data and serialization into Aurora structs are done by the proposed smart contract.

To deal with Write (state changing) methods, ownership requirements are a Must for a Near Dapp/User to be able to manage assets on Aurora.
Deployment of the proposed smart contract to Near address on Near, owning ERC20 assets on Aurora, would make sure that Near Dapp/User can manage the assets directly from Near smart contract.
In this way, developers can focus on Near smart contracts development and manage the assets on Aurora directly and securely, thus eliminating extra steps of using aurora-is-near javascript libraries thereby reducing room for potential errors.

Example of a Read-only method, checking allowance of `"A5C7FDbe2a8B814369e89EAed7EE52630fcb4C59"`(USN) ERC20 token by an owner `"6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474"` to spender `"ffffffffffffffffffffffffffffffffffffffff"`:
```console
near call deployedTo_NearAccountId function_call '{"aurora_contract_address": "A5C7FDbe2a8B814369e89EAed7EE52630fcb4C59", "function": "allowance(address,address)", "parameters": ["6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474","ffffffffffffffffffffffffffffffffffffffff"],"}' --accountId deployedTo_NearAccountId
```

Example of a Write (state changing method) method, approving `"A5C7FDbe2a8B814369e89EAed7EE52630fcb4C59"` ERC20 (USN) token owned by owner `"6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474"` to be spend by spender `"ffffffffffffffffffffffffffffffffffffffff"`:
```console
near call deployedTo_NearAccountId function_call '{"aurora_contract_address": "A5C7FDbe2a8B814369e89EAed7EE52630fcb4C59", "function": "approve(address,uint256)", "parameters": ["6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474","1"],"}' --accountId deployedTo_NearAccountId
```
Note: owner's address must be equal to the last 20-bytes of a `deployedTo_NearAccountId`'s keccak hash (e.g., for `deployedTo_NearAccountId=account.testnet` the last 20-bytes of keccak(`account.testnet`) is equal to `6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474`), otherwise the ownership would not hold true and the cross-contract call would fail.

Where:
-  `deployedTo_NearAccountId` is an account id to which the smart contract is deployed
-  `aurora_contract_address` is the Aurora contract's address to which a cross-contract call is made
-  `function` is a `aurora_contract_address`'s function name
-  `parameters` is a list of parameters passed to the `function`

### How to use our solution

First, one needs to deploy the contract to the Account Id owned:
```console
near deploy --wasmFile target/wasm32-unknown-unknown/release/ProxyContract.wasm --accountId deployedTo_NearAccountId
```

Initialize the contract:
```console
near call deployedTo_NearAccountId new '{}' --accountId deployedTo_NearAccountId
```

Make a desired cross-contract call:
```console
near call deployedTo_NearAccountId function_call '{"function": "function_name(type of parameter 1, type of parameter 2, ...)", "parameters": [parameter 1, parameter 2, ...]}' --accountId deployedTo_NearAccountId
```

The contract does a thorough check of the input parameters to verify they correctness. In case they are incorrect, the cross-contract call is reverted (avoiding extra gas fee costs), and a clear error message is shown.

### Implementation within a NEAR contract

The example before showed the benefits of using this dynamic cross-contract call bridge while using the CLI. But its real benefits come when using it inside a smart contract in NEAR.

Someone who needs to implement a cross-contract function call between NEAR and Aurora would need to pack, encode and parse all the data to binary correctly (proper `keccak`, get correctly the selector bytes, etc), and proceed with calling Aurora.

In contrast, by deploying the proposed contract to owned account id, the person just needs to make a call within the contract as follows:
<!-- ```rust
solidity_function("approve(address,uint256)", &["6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474","1"]);
``` -->
```rust
function_call("aurora_contract_address", "approve(address,uint256)", &["6eba6ef721bd532ca8e5d5ec8faf7f0fa0dac474","1"]);
```

Just by passing a vector of strings (`str` to be more precise), the user doesn't need to worry about packaging, formatting, converting, or encoding any data. All of these operations happen smoothly behing the scenes.