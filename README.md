# MoneyStream: An On-Chain money streaming gateway

## Summary
 Money stream program is a Proof of Concept (PoC) dApp that demonstrates an ability to stream continuously SPL tokens over time. Payment is done fully P2P without mediators. Program is built with Rust and [Anchor framework](https://github.com/project-serum/anchor)

## Solana Hackathon Submission

### Repos

The hackathon submission repository is:

- [ibrica/money-stream-program](https://github.com/ibrica/money-stream-program/): Sealevel smart contract for the SPL Token streaming between two parties.

There is also a demonstration of the web3 solution for connection to the dApp in repo:  
- [ibrica/money-stream](https://github.com/https://github.com/ibrica/money-stream-program/): Chrome extension and site plugin enabling streaming of SPL Tokens between two wallets (Still not finished, not submitted to hackathon)

### Demo

See [here](https://youtube/) for a demo and walkthrough.


## Motivation
Work is inspired by Andreas M. Antonopoulos [streaming money](https://www.youtube.com/watch?v=l235ydAx5oQ) idea, often mentioned in the bitcoin community. It should be perfectly possible to send money over time, continuously and without friction, until one of the parties stops the transfer. For example, services provided over video conferencing like mentoring could be charged over fractions of time, which would be very convenient. Today, thanks to the speed and low fees of chains like Solana, and stable coins which eliminate the volatility of cryptocurrencies it is not only possible but very probable that this kind of solutions will be immensely popular.

## How it works
Smart contract is open to external apps, the basic idea is that each app can get some fees for enabling the transfer.
The contract itself starts when the payer transmits SPL Tokens defining the amount limit of the whole payment. Tokens are saved in a vault which is controlled by a system program. Payment consists of fractions defined by a rate set between payer and receiver. In regular time fractions, payer ticks a contract and increment the number of fractions. The receiving party can regularly check the balance but the payments are performed when one of the parties cancels the transfer of money. At that time the charged amount is transferred to the receivers wallet and the rest is returned to the payer.


## Developer Guide

- [Getting started](#getting-started)
- [Testing](#testing)
  * [Run tests](#run-tests)


### Getting started

Clone the repo and install dependencies:

    git clone https://github.com/ibrica/money-stream-program.git
    yarn

You also need to install Rust,follow the instructions here https://rustup.rs/ and Solana Tools, installation instructions are here: https://docs.solana.com/cli/install-solana-cli-tools.

Default solana network is devnet, so connect Solana CLI to it and fund a wallet to be able to deploy the program.

    solana airdrop 5
    
## Testing

### Run tests

Build, deploy and run the unit tests all with one command (thanks to Anchor):

    anchor test
    

