// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity 0.8.20;

contract A {
    uint a;

    function getA() external view returns (uint) {
        return a;
    }
}
