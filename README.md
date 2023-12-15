# BryteSchemes API

## Description

BryteSchemes API provides REST endpoints that provide utility to retrieve your account data as JSON with programs using the Bryte-Descriptor library (https://github.com/brytelands/bryte-descriptor).

Try it here: https://test.brytelands.io/

[![Watch the demo](https://img.youtube.com/vi/0jN9Wz95tGA/hqdefault.jpg)](https://www.youtube.com/embed/0jN9Wz95tGA)

## Build

```shell
cargo build
```
 
## Run

```shell
cargo run
```

or with docker:

```shell
docker-compose up
```

## Config

Override these values to set your own endpoints, for instance if you are using specific RPC endpoints. 

```shell
    BRYTESCHEMES_API_RPC_URL_DEV: https://api.devnet.solana.com
    BRYTESCHEMES_API_RPC_URL_TEST: https://api.testnet.solana.com
    BRYTESCHEMES_API_RPC_URL_MAIN: https://api.mainnet.solana.com
    BRYTESCHEMES_API_RPC_URL_LOCAL: http://localhost:8899
```

