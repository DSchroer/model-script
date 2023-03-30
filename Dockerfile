FROM debian:unstable-slim
RUN apt-get update; apt-get install -y \
    clang \
    cmake \
    libasound2-dev \
    libudev-dev \
    libatk1.0-dev \
    libgtk-3-dev \
    curl \
    gcc \
    g++ \
    mingw-w64 \
    zip \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    git \
    wget \
    libxml2-dev \
    libssl-dev \
    gnupg2 \
    apt-transport-https  \
    ca-certificates  \
    software-properties-common \
    xz-utils \
    emscripten \
    bash

# OSXCross for Mac Builds
RUN git clone https://github.com/tpoechtrager/osxcross
RUN cd osxcross && \
    wget -nc https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz && \
    mv MacOSX11.3.sdk.tar.xz tarballs/ && \
    UNATTENDED=yes ./build.sh
RUN ln -s /osxcross/target/bin/x86_64-apple-darwin20.4-ld /osxcross/target/bin/x86_64-apple-darwin-ld

# Rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Remove win32 dlls to force static linking
RUN rm /usr/lib/gcc/x86_64-w64-mingw32/12-win32/*.dll.a

# Toolchains
RUN rustup toolchain install nightly
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-apple-darwin
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-emscripten
RUN cargo install wasm-bindgen-cli

