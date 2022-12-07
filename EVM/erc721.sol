// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol";

contract TestERC721 is ERC721PresetMinterPauserAutoId
{
    constructor(
        string memory name,
        string memory symbol,
        string memory baseTokenURI) ERC721PresetMinterPauserAutoId(name,symbol,baseTokenURI){}

    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {
        string memory baseURI = _baseURI();
        return "ipfs://bafkreicol3okeflbtxhxdinwlx7uya7m345zghbzxjufh6g7oygb7lg6em";
        // return string(abi.encodePacked(baseURI, tokenId.toString(), ".json")
    }
}
