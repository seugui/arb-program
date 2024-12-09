#!/bin/bash

# Verificar si se especificó el cluster
if [ -z "$ANCHOR_PROVIDER_CLUSTER" ]; then
  echo "Error: ANCHOR_PROVIDER_CLUSTER no está definido."
  exit 1
fi

# Cambiar el cluster en Anchor.toml
sed -i "s/^cluster = .*/cluster = \"$ANCHOR_PROVIDER_CLUSTER\"/" Anchor.toml

echo "Cluster configurado como: $ANCHOR_PROVIDER_CLUSTER"
