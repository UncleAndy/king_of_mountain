#!/usr/bin/env bash
set -euo pipefail

# Ensure a `deployer` identity exists and is funded on testnet.
# Safe to call before every testnet action — it auto-creates + funds
# if the key is missing. Does NOT touch mainnet (my-real-admin).
#
# NOTE (NixOS): there is no /usr/bin/env, so the shebang above will NOT
# work when the script is executed directly. Always invoke it as:
#     bash scripts/ensure_testnet_key.sh
# (the Makefile does this). The shebang is kept only for portability.

KEY_NAME="deployer"
NETWORK="testnet"

# Check if the identity already exists
if stellar keys public-key "$KEY_NAME" >/dev/null 2>&1; then
    echo "✅ Identity '$KEY_NAME' already exists: $(stellar keys public-key "$KEY_NAME")"
    exit 0
fi

echo "⚠️  Identity '$KEY_NAME' not found — generating + funding on $NETWORK..."
stellar keys generate "$KEY_NAME" --network "$NETWORK" --overwrite >/dev/null 2>&1
ADDRESS=$(stellar keys public-key "$KEY_NAME")
echo "✅ Generated: $ADDRESS"

stellar keys fund "$KEY_NAME" --network "$NETWORK" 2>&1
echo "✅ Funded: $ADDRESS ($NETWORK)"
