# Usar una imagen base oficial de Rust
FROM rust:latest

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    libudev-dev \
    pkg-config \
    build-essential \
    curl \
    git \
    nodejs \
    npm && \
    apt-get clean

# Instalar la última versión estable de Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Instalar Solana CLI (especificando la versión más reciente estable)
RUN curl -sSfL https://release.anza.xyz/v1.16.11/solana-install-init-x86_64-unknown-linux-gnu | sh
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Instalar Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Crear directorio de trabajo
WORKDIR /project

# Copiar el contenido del proyecto al contenedor
COPY . /project

# Compilar el proyecto
RUN anchor build

# Establecer el entrypoint para el contenedor
ENTRYPOINT [ "/bin/bash" ]
