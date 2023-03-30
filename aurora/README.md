Install packages
`npm install`
`npm install --save-dev package_name`

# AuroraTest smart contract for testing Rust Proxy contract
`AuroraTest.sol` in `contracts` folder is initializable smart contract with number of methods for testing Near's rust proxy contract methods.

Make sure you set required for hardhat AURORA_TESTNET_PRIVATE_KEY in .env file!!!
`AURORA_TESTNET_PRIVATE_KEY=YOUR_PRIVATE_KEY`

Unit Tests
`npx hardhat test`

Deploy and note address contract is deployed to (use this method when contract has to be initialized from Near's Proxy Contract)
`npx hardhat run scripts/deploy.js --network AURORA_TESTNET`

Deploy, Initialize and note address contract is deployed to (use this method when interacting with AuroraTest wihtin Aurora)
`npx hardhat run scripts/deploy_initialize.js --network AURORA_TESTNET`

Interact with the contract using noted address of the deployed contract (i.e., set AuroraTestContractAddr variable in interact.js)
`npx hardhat run scripts/interact.js --network AURORA_TESTNET`