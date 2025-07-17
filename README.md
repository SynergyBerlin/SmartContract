NFC Tap-to-Pay Smart Contracts for Polkadot

This project is a submission for the Blockspace Synergy 2025 Hackathon. It provides the on-chain infrastructure for a seamless, NFC-based "tap-to-pay" system using Polkadot ecosystem tokens.

The core idea is to combine the user experience of traditional contactless payments with the security and decentralization of blockchain, using Account Abstraction to create secure, single-purpose payment "cards" on a user's phone.
Deployed Contracts (Local Dev Node)

These contracts were deployed on a local Moonbeam development node (./moonbeam-dev --dev).

    Intermediary Escrow Contract: [PASTE THE ESCROW CONTRACT ADDRESS HERE]

    Card Contract (Example):

        Address: 0x2c6fc00458f198f46ef072e1516b83cd56db7cf5

        Owner (Deployer): //Alice (0x9621dde636de098b43efb0fa9b61facfe328f99d)

        Delegate (Phone's Key): //Bob (0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac)

Project Architecture

The system consists of three main components:

    On-Chain Smart Contracts (This Repository): Two ink! smart contracts that handle the payment logic.

        CardContract: An account abstraction contract representing the user's payment card. It is owned by the user's main wallet but delegates payment authority to a low-security key on their phone.

        IntermediaryEscrow: A contract that receives funds from the CardContract. For Milestone 1, it simply receives payments. In the future, it will manage fraud-protection holding periods and integrate with DEXs.

    Merchant Terminal (Laptop): A Python application connected to a standard USB NFC reader. It generates payment requests and confirms transactions.

    User Client (Android App): A mobile application that uses Host Card Emulation (HCE) to act as an NFC card. It securely stores the delegate key and uses it to sign payment transactions when tapped on a merchant terminal.

User Flow

    Setup: The user installs the mobile app, which generates a new delegate key. The user sends funds to a CardContract and assigns the new key as the delegate.

    Tap: The merchant enters a price on their terminal. The user taps their phone on the NFC reader.

    Request: The terminal sends a payment request (amount, destination) to the phone via NFC.

    Sign & Pay: The mobile app receives the request, signs a pay transaction using the stored delegate key, and submits it to the blockchain.

    Confirm: The terminal sees the transaction succeed on-chain and displays a "Payment Successful" message.

How to Build and Deploy Locally

This project is built with ink! and designed for EVM-compatible networks that support Wasm contracts, like Moonbeam.
Prerequisites

    Rust & Cargo

    cargo-contract CLI (version 4.0.0+ recommended)

    A local Moonbeam development node (See instructions)

1. Run Local Node

Start the Moonbeam development node in a separate terminal.
Generated bash

./moonbeam-dev --dev

IGNORE_WHEN_COPYING_START
Use code with caution. Bash
IGNORE_WHEN_COPYING_END 2. Build the Contracts

Compile the Wasm and generate the contract metadata.
Generated bash

cargo contract build

IGNORE_WHEN_COPYING_START
Use code with caution. Bash
IGNORE_WHEN_COPYING_END 3. Deploy the Contracts

Open a new terminal in the project directory.

First, deploy the intermediary_escrow contract:
Generated bash

cargo contract instantiate \
 --contract intermediary_escrow \
 --suri //Alice \
 --execute

IGNORE_WHEN_COPYING_START
Use code with caution. Bash
IGNORE_WHEN_COPYING_END

Copy the resulting contract address.

Next, deploy the card_contract, providing a delegate address (e.g., Bob's EVM address from the dev node):
Generated bash

# Delegate address for //Bob on moonbeam-dev

DELEGATE_ADDR="0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"

cargo contract instantiate \
 --contract card_contract \
 --suri //Alice \
 --args $DELEGATE_ADDR \
 --execute

IGNORE_WHEN_COPYING_START
Use code with caution. Bash
IGNORE_WHEN_COPYING_END

You now have both contracts running on your local network.
Hackathon Bounties Targeted

This project is strategically designed to compete for multiple bounties:

    Polkadot Main Track: As a novel payment solution with a clear MVP path.

    ink! Bounty: For "Best use cases for Account Abstraction", which is the core of our CardContract. We also plan to integrate XCM in Milestone 2.

    UX Bounty: By aiming to completely hide the complexity of crypto payments behind a familiar tap-to-pay user journey.

    Hyperbridge Bounty (Future): Our Milestone 2 plan includes using Hyperbridge's cross-chain storage queries to implement a "reputation" system, allowing trusted users to bypass the payment escrow period.
