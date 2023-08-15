// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity 0.8.20;

contract B {
    uint b;

    function getB() external view returns (uint) {
        return b;
    }
}
