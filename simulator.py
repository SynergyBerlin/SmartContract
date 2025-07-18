import time
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.contracts import ContractInstance

# --- 1. CONFIGURATION ---
# This section contains all the variables you need to set up.

# The WebSocket URL of your local Moonbeam dev node
NODE_URL = "ws://127.0.0.1:9944"

# The path to your compiled contract metadata file.
# This file is created when you run `cargo contract build`.
CARD_CONTRACT_METADATA = './target/ink/card_contract.contract'

# --- ACTION REQUIRED: Fill these in after you deploy your contracts ---
# The EVM-style (H160) address of your deployed CardContract.
CARD_CONTRACT_ADDRESS = "0x2c6fc00458f198f46ef072e1516b83cd56db7cf5" # Use your deployed address

# The EVM-style (H160) address of your deployed IntermediaryEscrow contract.
INTERMEDIARY_ESCROW_ADDRESS = "[PASTE YOUR DEPLOYED ESCROW ADDRESS HERE]" 

# The secret key for the DELEGATE account. For our simulation, this is //Bob.
# This is the key that is authorized to spend from the CardContract.
DELEGATE_KEYPAIR = Keypair.create_from_uri('//Bob')

# The secret key for the OWNER account. For our simulation, this is //Alice.
# We use this key to send funds TO the CardContract so it has a balance to spend.
OWNER_KEYPAIR = Keypair.create_from_uri('//Alice')

# --- 2. HELPER FUNCTIONS ---

def check_balance(substrate, contract_address):
    """Checks and returns the balance of a given contract address."""
    result = substrate.query("System", "Account", [contract_address])
    balance = result.value['data']['free']
    return balance

# --- 3. MAIN SCRIPT ---

def main():
    print("🚀 Initializing The Ultimate Milestone 1 Simulator...")
    
    # Connect to the local node
    try:
        substrate = SubstrateInterface(url=NODE_URL)
        print(f"✅ Connected to local node: {substrate.chain}")
    except ConnectionRefusedError:
        print("❌ CONNECTION FAILED: Is your local Moonbeam dev node running? (`./moonbeam-dev --dev`)")
        return

    # Load the contract instance from its metadata and address
    try:
        card_contract = ContractInstance.create_from_address(
            contract_address=CARD_CONTRACT_ADDRESS,
            metadata_file=CARD_CONTRACT_METADATA,
            substrate=substrate
        )
        print(f"✅ Loaded CardContract at: {CARD_CONTRACT_ADDRESS}")
    except FileNotFoundError:
        print(f"❌ METADATA NOT FOUND: Make sure '{CARD_CONTRACT_METADATA}' exists. Did you run `cargo contract build`?")
        return
        
    if not INTERMEDIARY_ESCROW_ADDRESS or "[PASTE" in INTERMEDIARY_ESCROW_ADDRESS:
        print("❌ CONFIGURATION ERROR: Please paste your Intermediary Escrow contract address into the script.")
        return

    # --- Pre-flight Check: Ensure the CardContract has funds ---
    card_balance = check_balance(substrate, CARD_CONTRACT_ADDRESS)
    print(f"💳 CardContract current balance: {card_balance / 10**18} DEV")

    if card_balance == 0:
        print("\n⚠️  CardContract has no funds! Let's send it some.")
        
        transfer_call = substrate.compose_call(
            call_module='Balances',
            call_function='transfer_keep_alive',
            call_params={
                'dest': CARD_CONTRACT_ADDRESS,
                'value': 5 * 10**18  # Send 5 DEV tokens
            }
        )
        extrinsic = substrate.create_signed_extrinsic(call=transfer_call, keypair=OWNER_KEYPAIR)
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"✅ Successfully funded CardContract in block {receipt.block_hash}")
        time.sleep(2) # Wait a moment for balance to update

    # --- Main Simulation Loop ---
    while True:
        print("\n-------------------------------------------")
        input("Press Enter to simulate a new PoS transaction...")
        
        # Define a mock amount for this transaction
        amount_to_pay_base_units = int(0.2 * (10**18)) # 0.2 DEV tokens

        print(f"✍️  Simulating phone signing a transaction to pay {amount_to_pay_base_units / 10**18} DEV...")
        print(f"   - Caller (Delegate): {DELEGATE_KEYPAIR.ss58_address}")
        print(f"   - Destination (Escrow): {INTERMEDIARY_ESCROW_ADDRESS}")
        
        try:
            # This is the core of the PoC: The delegate calls the 'pay' function
            call = card_contract.call(
                keypair=DELEGATE_KEYPAIR,
                method='pay',
                args={
                    'intermediary_contract': INTERMEDIARY_ESCROW_ADDRESS,
                    'amount': amount_to_pay_base_units
                },
                gas_limit=200000000000 # A safe gas limit
            )

            # Sign and submit the transaction to the blockchain
            extrinsic = substrate.create_signed_extrinsic(call=call, keypair=DELEGATE_KEYPAIR)
            receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            print(f"✅ Transaction included in block: {receipt.block_hash}")

            if receipt.is_success:
                print("🎉 PAYMENT SUCCESSFUL! 🎉")
            else:
                print(f"❌ PAYMENT FAILED! On-chain error: {receipt.error_message}")
        
        except Exception as e:
            print(f"An unexpected error occurred: {e}")
        
        print("-------------------------------------------\n")


if __name__ == "__main__":
    main()