import time
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.contracts import ContractInstance

# --- CONFIGURATION ---
NODE_URL = "ws://127.0.0.1:9944"

CARD_CONTRACT_METADATA = './card_contract/target/ink/nfc_smart_contract.contract'
ESCROW_CONTRACT_METADATA = '../intermediary_escrow/target/ink/intermediary_escrow.contract'

CARD_CONTRACT_ADDRESS = "0x2c6fc00458f198f46ef072e1516b83cd56db7cf5"  # Replace with your deployed CardContract address
ESCROW_CONTRACT_ADDRESS = "0xb2ecdfb581808e799e5cf0c6cbd3bc5359f11de0"  # Replace with your deployed IntermediaryEscrow address

# The PoS terminal does NOT have the delegate key. It only receives a signature from the phone.
# For simulation, we'll use Bob's key to sign, but in a real system, the phone would sign and send the signature to the terminal.

DELEGATE_KEYPAIR = Keypair.create_from_uri('//Bob')

def main():
    print("🛒 PoS Terminal Simulator (waiting for NFC tap)")

    # Connect to node
    substrate = SubstrateInterface(
        url=NODE_URL,
    )
    print(f"✅ Connected to chain: {substrate.chain}")

    # Load contracts
    card_contract = ContractInstance.create_from_address(
        contract_address=CARD_CONTRACT_ADDRESS,
        metadata_file=CARD_CONTRACT_METADATA,
        substrate=substrate
    )
    print(f"✅ Loaded CardContract: {CARD_CONTRACT_ADDRESS}")

    while True:
        input("\n[PoS] Waiting for user tap... (press Enter to simulate)")
        amount = int(0.2 * 10**18)  # 0.2 DEV

        # In a real system, the PoS would send a payment request to the phone and receive a signature.
        # Here, we simulate the phone signing by using Bob's key directly.
        print("[PoS] Sending payment request to phone...")
        time.sleep(1)
        print("[Phone] Signing transaction with delegate key (Bob)...")
        time.sleep(1)

        # The phone would sign and send the transaction data/signature to the PoS.
        # The PoS submits the transaction to the chain.
        print("[PoS] Submitting signed transaction to CardContract...")

        receipt = card_contract.exec(
            keypair=DELEGATE_KEYPAIR,
            method='pay',
            args={
                'intermediary_contract': ESCROW_CONTRACT_ADDRESS,
                'amount': amount
            },
            value=0,
            gas_limit=200_000_000_000
        )
        print(f"✅ Transaction sent. Block: {receipt.block_hash}")
        if receipt.is_success:
            print("🎉 PAYMENT SUCCESSFUL!")
        else:
            print(f"❌ PAYMENT FAILED: {receipt.error_message}")

if __name__ == "__main__":
    main() 