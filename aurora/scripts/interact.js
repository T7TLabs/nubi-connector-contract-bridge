// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require("hardhat");
const AuroraTestContractAddr = "0x3CaC20EB39c01271FE8860ed9A179F9757F79aDE";

async function main() {
    const [signer] = await hre.ethers.getSigners();
    console.log("signer:", signer.address);

    const balance = await hre.ethers.provider.getBalance(signer.address);
    console.log("signer's balance:", balance);

    const auroraTest = await hre.ethers.getContractAt("AuroraTest", AuroraTestContractAddr);

    console.log("TestOwner, returns correct address:", await auroraTest.TestOwner());
    console.log("TestOnlyOwner, returns correct address:", await auroraTest.TestOnlyOwner());

    console.log("TestInputPars11, returns true:", await auroraTest.TestInputPars11(true));
    console.log("TestInputPars12, returns address:", await auroraTest.TestInputPars12("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"));
    console.log("TestInputPars13, returns uint256:", await auroraTest.TestInputPars13(1));
    console.log("TestInputPars221, returns address,bool,uint256:", await auroraTest.TestInputPars221("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",true,1));
    console.log("TestInputPars310, returns uint256[]:", await auroraTest.TestInputPars310([1,2,3,4,5]));
    console.log("TestInputPars313, returns uint256, address, bool, uint256[]:", await auroraTest.TestInputPars313(1,"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",true,[1,2,3,4,5]));

    console.log("TestInputParsStr1, returns string:", await auroraTest.TestInputParsStr1("Hello!"));
    console.log("TestInputParsStr2, returns uint256, string, uint256[]:", await auroraTest.TestInputParsStr2(1, "Hello!", [1,2]));

    console.log("TestInputParsBytes1, returns bytes:", await auroraTest.TestInputParsBytes1("0xabcdef"));
    console.log("TestInputParsBytes2, returns uint256,bytes,uint256[]:", await auroraTest.TestInputParsBytes2(1, "0xabcdef", [1,2,3]));

    console.log("TestInputParsFixedBytes1, returns bytes[2]:", await auroraTest.TestInputParsFixedBytes1(["0xabcd", "0xabef"]));
    console.log("TestInputParsFixedBytes2, returns uint256,bytes[2],uint256[]:", await auroraTest.TestInputParsFixedBytes2(1, ["0xabcd", "0xabef"], [1,2,3]));

    console.log("TestInputParsTuples1, returns tuple(uint256,bytes[2],uint256[2]):", await auroraTest.TestInputParsTuples1([1,"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"]));
    console.log("TestInputParsTuples2, returns uint256,tuple(uint256,bytes[2],uint256[2]),uint256[2]:", await auroraTest.TestInputParsTuples2(1, [1,"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"], [1,2,3]));

    console.log("TestInputParsTuples3, returns array of tuples tuple(uint256,bytes[2],uint256[2]):", await auroraTest.TestInputParsTuples3([[1,["0xabcd", "0xabef"],[1,2,3,4,5]], [1,["0xabcd", "0xabef"],[1,2,3,4,5]]]));
    console.log("TestInputParsTuples4, returns uint256, array of tuples tuple(uint256,bytes[2],uint256[2]), uint256[]:", await auroraTest.TestInputParsTuples4(1,[[1,["0xabcd", "0xabef"],[1,2,3,4,5]], [1,["0xabcd", "0xabef"],[1,2,3,4,5]]],[1,2,3]));

    console.log("TestInputParsArrays1, returns uint256[][]:", await auroraTest.TestInputParsArrays1([[1,2,3],[4,5,6]]));
    console.log("TestInputParsArrays2, returns uint256, uint256[][], uint256[]:", await auroraTest.TestInputParsArrays2(1,[[1,2,3],[4,5,6]],[1,2,3,4,5]));

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
