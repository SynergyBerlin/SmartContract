# Intermediary Escrow Contract

This is an ink! 6.0 smart contract for use as an intermediary escrow on EVM-compatible Substrate chains.

## Features

- Stores an admin (the deployer)
- Accepts payments via a payable message (`receive_payment`)
- Exposes a getter for the admin address

## Build

```sh
cargo contract build
```

Artifacts will be in `target/ink/`.

## Deploy (Upload)

```sh
cargo contract upload --suri //Alice -x
```

## Instantiate

```sh
cargo contract instantiate \
  --suri //Alice \
  --constructor new \
  target/ink/intermediary_escrow.contract -x
```

## Usage

- Call `receive_payment` (selector `0x636F696E`) to send funds to the contract.
- Call `get_admin` to retrieve the admin address.

## Notes

- This contract is designed for EVM chains (H160 addresses, U256 balances).
- Make sure your node supports ink! 6.0 contracts.

---

**Example integration:**

- Use this contract as the payment receiver in a multi-contract flow (e.g., called by a card contract).

---

For more info, see [ink! documentation](https://use.ink/).
