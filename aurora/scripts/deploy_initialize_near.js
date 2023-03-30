// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require("hardhat");

async function main() {

  const [signer] = await hre.ethers.getSigners();
  console.log("signer:", signer.address);

  const AuroraTest = await hre.ethers.getContractFactory("AuroraTest");
  const auroraTest = await AuroraTest.deploy();

  await auroraTest.deployed();

  console.log(
    `AuroraTest deployed to ${auroraTest.address}`
  );

  await auroraTest.initialize("t7t.testnet");
  const nearContractOwner = await auroraTest.TestOnlyNearOwner("t7t.testnet");
  console.log("Aurora<->Near owner:", nearContractOwner);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
