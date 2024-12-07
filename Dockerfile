# Usar la imagen oficial de Rust como base
FROM rust:latest

# Instalar dependencias necesarias para Solana, Anchor y pruebas
RUN apt-get update && apt-get install -y --no-install-recommends \
    libudev-dev \
    pkg-config \
    build-essential \
    curl \
    git \
    nodejs \
    npm && \
    curl -sSfL https://release.anza.xyz/v1.16.11/solana-install-init-x86_64-unknown-linux-gnu | sh && \
    cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Establecer el PATH para Solana CLI
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Crear directorio de trabajo
WORKDIR /project

# Copiar todo el proyecto y el script dinámico
COPY . /project

# Instalar dependencias para pruebas
RUN npm install --save-dev @project-serum/anchor

# Dar permisos al script dinámico
RUN chmod +x set_cluster.sh

# Establecer el entrypoint por defecto
ENTRYPOINT [ "/bin/bash" ]
