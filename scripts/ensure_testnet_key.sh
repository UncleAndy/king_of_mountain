#!/usr/bin/env bash
set -euo pipefail

# Ensure a `deployer` identity exists and is funded on testnet.
# Safe to call before every testnet action — it auto-creates + funds
# if the key is missing. Does NOT touch mainnet (my-real-admin).
#
# Outputs ONLY the deployer public key to stdout (as the last/only line)
# so callers can capture it with:  ADDR=$(bash scripts/ensure_testnet_key.sh)
# All status messages go to stderr.
#
# NOTE (NixOS): there is no /usr/bin/env, so the shebang above will NOT
# work when the script is executed directly. Always invoke it as:
#     bash scripts/ensure_testnet_key.sh
# (the Makefile does this). The shebang is kept only for portability.

KEY_NAME="deployer"
NETWORK="testnet"

if stellar keys public-key "$KEY_NAME" >/dev/null 2>&1; then
    echo "✅ Identity '$KEY_NAME' already exists: $(stellar keys public-key "$KEY_NAME")" >&2
else
    echo "⚠️  Identity '$KEY_NAME' not found — generating + funding on $NETWORK..." >&2
    stellar keys generate "$KEY_NAME" --network "$NETWORK" --overwrite >/dev/null 2>&1
    echo "✅ Generated: $(stellar keys public-key "$KEY_NAME")" >&2
    stellar keys fund "$KEY_NAME" --network "$NETWORK" 2>&1 | sed 's/^/✅ /' >&2
fi

# Only the address goes to stdout — ready for capture by the caller
stellar keys public-key "$KEY_NAME"
