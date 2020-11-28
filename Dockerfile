FROM rust:1.48-buster AS base

ARG ENVIRONMENT=prod

RUN useradd -m red

WORKDIR /home/red/pokespeare

# Setup of dummy packages `src`s to cache deps
RUN mkdir -p src && echo "fn main() {}" > ./src/main.rs

# Copy root stuff + download and build deps
COPY Cargo.lock Cargo.toml ./
RUN mkdir .cargo && cargo vendor > .cargo/config
RUN cargo build --release

# Cleanup dummy packages `src`s used to cache deps
RUN rm -rf src
# Proceed with "real" builds: test build in case of NOT "prod" release and release build otherwise
COPY . .
RUN if [ ! "${ENVIRONMENT}" = "prod" ]; then cargo test --no-run; else cargo build --release; fi
RUN ls -la src/


FROM debian:buster-slim

RUN useradd -m red

WORKDIR /home/red/bin

RUN apt-get -y update && apt-get -y --no-install-recommends install ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

COPY --from=base /home/red/pokespeare/target/release/pokespeare .

ENTRYPOINT ["./pokespeare"]
