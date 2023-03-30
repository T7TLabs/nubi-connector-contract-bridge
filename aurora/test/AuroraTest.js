const { assert, expect } = require("chai");

describe("AuroraTest", function () {
    let auroraTest;
    let owner, otherAccount;
  
    before(async () => {
      [owner, otherAccount] = await ethers.getSigners();
      const AuroraTest = await ethers.getContractFactory("AuroraTest");
      auroraTest = await AuroraTest.deploy();
      await auroraTest.deployed();
      await auroraTest.initialize();
    });

    // it("TestOwner() public view method, should return correct address", async function () {
    //   expect(await auroraTest.TestOwner()).to.equal(owner.address);
    // });

    // it("TestOnlyOwner() onlyOwner view method called by owner, should return correct address", async function () {
    //     expect(await auroraTest.TestOnlyOwner()).to.equal(owner.address);
    // });

    // it("TestOnlyOwner() onlyOwner view method called by another address, should be reverted", async function () {
    //     await expect(auroraTest.connect(otherAccount).TestOnlyOwner()).to.be.reverted
    // });

    // it("TestInputPars11() public view method passing [boolean], should return true", async function () {
    //     //expect(await auroraTest.TestInputPars11(true)).to.be.a('boolean');
    //     expect(await auroraTest.TestInputPars11(true)).to.equal(true);
    // });

    // it("TestInputPars12() public view method passing [address], should return true", async function () {
    //     expect(await auroraTest.TestInputPars12("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")).to.equal(true);
    // });

    // it("TestInputPars13() public view method passing [uint256], should return true", async function () {
    //     expect(await auroraTest.TestInputPars13(1)).to.equal(true);
    // });

    // it("TestInputPars310() public view method passing [uint256[]], should return true", async function () {
    //     expect(await auroraTest.TestInputPars13([1,2,3,4,5])).to.equal(true);
    // });

    // it("TestInputPars313() public view method passing [uint256,address,bool,uint256[]], should return true", async function () {
    //     expect(await auroraTest.TestInputPars313(1,"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",true,[1,2,3,4,5])).to.equal(true);
    // });

});