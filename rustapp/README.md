NEAR FT Kollage
==================

The kolla is a dapp working under near smart contracts.
For 0.5 NEAR you can upload an NFT image stored in nft.storage to The kolla.
For the moment, the NEARS are transfered to our testnet account. However we are looking forward to write a contract which guarantees a charity end for the assets. 


Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Install dependencies: `yarn install`
4. Run the local development server: `yarn dev` 

It is optional to install the NEAR CLI because the contract is already connected to the frontend. However, in case you want to install it:
  `yarn install --global near-cli`

Understanding the files
===========
There are two principal directories: 
1. **"contract"**: It is where the contract (aka the backend) code is located.
2. **"src"**: It is where the frontend code (html, css, js) is located.

This project was initialized in the beggining with create-near-app.
