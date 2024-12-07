# Usar la imagen oficial de Rust
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

# Instalar Solana CLI
RUN curl -sSfL https://release.anza.xyz/v1.16.11/solana-install-init-x86_64-unknown-linux-gnu | sh
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Instalar Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Crear directorio de trabajo
WORKDIR /project

# Copiar todo el proyecto y el script dinámico
COPY . /project

# Configurar el cluster dinámicamente antes de compilar
RUN chmod +x set_cluster.sh
RUN ./set_cluster.sh

# Construir el proyecto
RUN anchor build

# Establecer el entrypoint por defecto
ENTRYPOINT [ "/bin/bash" ]
