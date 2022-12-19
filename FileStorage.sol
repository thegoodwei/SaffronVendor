pragma solidity ^0.6.0;

import "https://github.com/Arachnid/solidity-ipfs/blob/master/contracts/Ipfs.sol";

// contract to store and retrieve a file from IPFS
contract FileStorage {
    // address of the Ipfs contract
    Ipfs public ipfs;

    // mapping to store the IPFS hash of the file
    mapping (bytes32 => bytes32) public hashes;

    // constructor to set the address of the Ipfs contract
    constructor(address _ipfsAddress) public {
        ipfs = Ipfs(_ipfsAddress);
    }

    // function to store a file on IPFS and store the hash on the blockchain
    function storeFile(bytes32 _fileHash, bytes _file) public {
        // store the file on IPFS
        ipfs.add(_file);
        // store the IPFS hash on the blockchain
        hashes[_fileHash] = ipfs.getLastHash();
    }

    // function to retrieve a file from IPFS using the hash stored on the blockchain
    function retrieveFile(bytes32 _fileHash) public view returns (bytes) {
        // get the IPFS hash from the mapping
        bytes32 ipfsHash = hashes[_fileHash];
        // retrieve the file from IPFS using the IPFS hash
        return ipfs.get(ipfsHash);
    }
}
