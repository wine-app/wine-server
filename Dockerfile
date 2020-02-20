#-------------------------------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
#-------------------------------------------------------------------------------------------------------------

FROM rust

RUN rustup default nightly && \
    cargo install cargo-watch

WORKDIR /workspace
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build

EXPOSE 8080


# WORKDIR /usr/src/app
# COPY Cargo.toml .

# RUN rustup default nightly && \
#     mkdir -p src && \
#     echo "fn main() {}" > src/main.rs && \
#     cargo build -Z unstable-options --out-dir /output

# FROM rust as application

# COPY --from=dependencies /usr/src/app/Cargo.toml .
# COPY --from=dependencies /usr/local/cargo /usr/local/cargo

# COPY src/ src/
# COPY migrations/ migrations/

# VOLUME /output

