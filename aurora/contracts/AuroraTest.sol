//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract AuroraTest is Initializable {

    address private _owner;
    string private _nearOwner;

    struct tupleInput0 {
        uint256 tupleIn01;
        address tupleIn02;
    }

    struct tupleInput1 {
        uint256 tupleIn1;
        bytes[2] tupleIn2;
        uint256[] tupleIn3;
    }

    function initialize(string calldata nearOwner) public initializer {
        _owner = msg.sender;
        _nearOwner = nearOwner;
    }

    // onlyOwner modifier that validates only 
    // if caller of function is contract owner, 
    // otherwise not
    modifier onlyOwner() 
    {
        require(isOwner(), "Function accessible only by the owner !!");
        _;
    }

    // function for owner to verify their ownership.
    // Returns true for owners otherwise false
    function isOwner() public view returns(bool) {
        return msg.sender == _owner;
    }

    function isNearOwner(string memory caller) public view returns(bool) {
        return keccak256(abi.encodePacked(caller)) == keccak256(abi.encodePacked(_nearOwner));
    }

    function TestOnlyNearOwner(string memory caller) public view returns (string memory nearOwner) {
        require(isNearOwner(caller), "Function accessible only by the Near (Aurora<->Near) owner !!");

        return _nearOwner;
    }

    function TestOwner() public view returns (address) {
        return _owner;
    }

    function TestOnlyOwner() onlyOwner public view returns (address) {
        return _owner;
    }

    function TestInputPars11(bool par) public view returns(bool) {
        return par;
    }

    function TestInputPars12(address par) public view returns(address) {
        return par;
    }

    function TestInputPars13(uint256 par) public view returns(uint256) {
        return par;
    }

    // new to test padding to 32 bytes (2023.01.09)
    function TestInputPars14(uint160 par) public view returns(uint160) {
        return par;
    }

    // new to test sign handling (2023.01.09): left-padded with 0xff for negative and 0x00 for positive
    function TestInputPars15(int256 par) public view returns(int256) {
        return par;
    }

    function TestInputPars211(bool par1, address par2, uint256 par3) public view returns(bool, address, uint256) {
        return (par1, par2, par3);
    }

    function TestInputPars212(bool par1, uint256 par2, address par3) public view returns(bool, uint256, address) {
        return (par1, par2, par3);
    }

    function TestInputPars220(address par1, address par2, address par3) public view returns(address, address, address) {
        // (ret1, ret2, ret3) = (par1, par2, par3);
        return (par1, par2, par3);
    }

    function TestInputPars221(address par1, bool par2, uint256 par3) public view returns(address, bool, uint256) {
        return (par1, par2, par3);
    }

    function TestInputPars222(address par1, uint256 par2, bool par3) public view returns(address, uint256, bool) {
        return (par1, par2, par3);
    }

    function TestInputPars231(uint256 par1, bool par2, address par3) public view returns(uint256, bool, address) {
        return (par1, par2, par3);
    }

    function TestInputPars232(uint256 par1, address par2, bool par3) public view returns(uint256, address, bool) {
        return (par1, par2, par3);
    }

    function TestInputPars310(uint256[] memory par) public view returns(uint256[] memory) {
        return (par);
    }

    function TestInputPars311(uint256 par1, uint256[] calldata par2) public view returns(uint256, uint256[] calldata) {
        return (par1, par2);
    }

    function TestInputPars312(uint256 par1, address par2, uint256[] calldata par3) public view returns(uint256, address, uint256[] calldata) {
        return (par1, par2, par3);
    }

    function TestInputPars313(uint256 par1, address par2, bool par3, uint256[] calldata par4) public view returns(uint256, address, bool, uint256[] calldata) {
        return (par1, par2, par3, par4);
    }

    // new types (2023.01.09)
    function TestInputParsStr1(string memory par1) public view returns(string memory) {
        return par1;
    }

    function TestInputParsStr2(uint256 par1, string memory par2, uint256[] calldata par3) public view returns(uint256, string memory, uint256[] calldata) {
        return (par1, par2, par3);
    }

    function TestInputParsBytes1(bytes memory par1) public view returns(bytes memory) {
        return par1;
    }

    function TestInputParsBytes2(uint256 par1, bytes memory par2, uint256[] calldata par3) public view returns(uint256, bytes memory, uint256[] calldata) {
        return (par1, par2, par3);
    }

    function TestInputParsFixedBytes1(bytes[2] memory par1) public view returns(bytes[2] memory) {
        return par1;
    }

    function TestInputParsFixedBytes2(uint256 par1, bytes[2] memory par2, uint256[] calldata par3) public view returns(uint256, bytes[2] memory, uint256[] calldata) {
        return (par1, par2, par3);
    }

    function TestInputParsTuples1(tupleInput0 memory _data) public view returns(tupleInput0 memory) {
        return _data;
    }

    function TestInputParsTuples2(uint256 par1, tupleInput0 memory _data, uint256[] calldata par3) public view returns(uint256, tupleInput0 memory, uint256[] calldata) {
        return (par1, _data, par3);
    }

    function TestInputParsTuples3(tupleInput1[] memory _data) public view returns(tupleInput1[] memory) {
        return _data;
    }

    function TestInputParsTuples4(uint256 par1, tupleInput1[] memory _data, uint256[] calldata par3) public view returns(uint256, tupleInput1[] memory, uint256[] calldata) {
        return (par1, _data, par3);
    }

    function TestInputParsArrays1(uint256[][] memory par1) public view returns(uint256[][] memory) {
        return par1;
    }

    function TestInputParsArrays2(uint256 par1, uint256[][] memory par2, uint256[] calldata par3) public view returns(uint256, uint256[][] memory, uint256[] calldata) {
        return (par1, par2, par3);
    }

}