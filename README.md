# Commands

```sh
# Build
cargo near build --no-docker

# Create account
near create-account stratum-miner-v1.testnet --useFaucet

# Deploy
near deploy stratum-miner-v1.testnet ./target/near/str_test.wasm

# View
near view stratum-miner-v1.testnet get_counter

# Calculate hash using cargo test

# Call (mutate)
near call stratum-miner-v1.testnet submit_proof '{"proof": [1, 27, 77, 3, 221, 140, 1, 241, 4, 145, 67, 207, 156, 76, 129, 126, 75, 22, 127, 29, 27, 131, 229, 198, 240, 241, 13, 137, 186, 30, 123, 206]}'  --accountId stratum-miner-v1.testnet
```

# str-test

cargo-near-new-project-description

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
