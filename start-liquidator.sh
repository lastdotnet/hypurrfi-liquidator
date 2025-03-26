#!/bin/bash

# Read .env file
if [ -f .env ]; then
    export $(cat .env | grep -v '#' | sed 's/\r$//' | xargs)
else
    echo ".env file not found"
    exit 1
fi

# Run liquidator with values from .env
./target/release/aave-v3-liquidator \
    --archive-rpc "$ARCHIVE_RPC" \
    --write-rpc "$WRITE_RPC" \
    --private-key "$PRIVATE_KEY" \
    --bid-percentage "$BID_PERCENTAGE" \
    --deployment "$DEPLOYMENT" \
    --liquidator-address "$LIQUIDATOR_ADDRESS" \
    --chain-id "$CHAIN_ID"