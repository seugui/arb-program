#!/bin/bash

if [[ "$1" == "localnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "localnet"|' Anchor.toml
  solana config set --url http://127.0.0.1:8898
  echo "Switched to Localnet"

elif [[ "$1" == "devnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "devnet"|' Anchor.toml
  solana config set --url https://api.devnet.solana.com
  echo "Switched to Devnet"

elif [[ "$1" == "mainnet" ]]; then
  sed -i '' 's|cluster = .*|cluster = "mainnet"|' Anchor.toml
  solana config set --url https://api.mainnet-beta.solana.com
  echo "Switched to Mainnet"

else
  echo "Usage: source set_env.sh [localnet|devnet|mainnet]"
fi
