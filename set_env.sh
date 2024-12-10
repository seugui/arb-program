#!/bin/bash

if [[ "$1" == "localnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "http://127.0.0.1:8899"|' Anchor.toml
  solana config set --url http://127.0.0.1:8899
  echo "Switched to Localnet"

elif [[ "$1" == "devnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "https://api.devnet.solana.com"|' Anchor.toml
  solana config set --url https://api.devnet.solana.com
  echo "Switched to Devnet"

elif [[ "$1" == "mainnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "https://api.mainnet-beta.solana.com"|' Anchor.toml
  solana config set --url https://api.mainnet-beta.solana.com
  echo "Switched to Mainnet"

else
  echo "Usage: source set_env.sh [localnet|devnet|mainnet]"
fi
