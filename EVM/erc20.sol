// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";

contract TestErc20 is ERC20PresetMinterPauser{
    constructor(string memory name, string memory symbol) ERC20PresetMinterPauser(name, symbol){}
}