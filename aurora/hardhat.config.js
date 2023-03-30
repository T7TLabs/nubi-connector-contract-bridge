require("@nomicfoundation/hardhat-toolbox");
// require("@nomiclabs/hardhat-waffle");
require('dotenv').config();

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

const { AURORA_TESTNET_PRIVATE_KEY } = process.env;
// console.log("AURORA_TESTNET_PRIVATE_KEY", AURORA_TESTNET_PRIVATE_KEY)

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.17",
  networks: {
    AURORA_TESTNET: {
      url: 'https://testnet.aurora.dev',
      accounts: [AURORA_TESTNET_PRIVATE_KEY],
    },
  },
};
