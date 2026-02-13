# p-token (Pinocchio Token Program)

A high-performance, zero-allocation reimplementation of the SPL Token program built with [Pinocchio](https://github.com/anza-xyz/pinocchio) — a zero-dependency library for creating Solana programs.

## Addresses

| | Address |
|---|---|
| **Program** | [`JsNFnUs9e2vDk3mUR5hjVr69j6JpM4VtoDk7QiZUSPL`](https://solscan.io/account/JsNFnUs9e2vDk3mUR5hjVr69j6JpM4VtoDk7QiZUSPL) |
| **Token** | [`4xCUgkuWmpk1gmrcT9PmmCPVY8kFnHKCsMoLsRrNSPL`](https://solscan.io/token/4xCUgkuWmpk1gmrcT9PmmCPVY8kFnHKCsMoLsRrNSPL) |

## Overview

p-token is a drop-in replacement for the SPL Token program that achieves significantly lower compute unit (CU) consumption through:

- **`#![no_std]` + zero heap allocation** — the entire program runs without a global allocator
- **Custom entrypoint with transfer fast-path** — `transfer` and `transfer_checked` (the most common operations) bypass generic account deserialization entirely
- **Two-tier instruction dispatch** — common instructions are matched first to minimize branch comparisons on the hot path
- **Batch instruction support** — multiple token operations in a single instruction (discriminator 255)

## Verified Build

This program is deployed as a [verified build](https://solana.com/docs/programs/verified-builds) on Solana mainnet. Anyone can independently verify the on-chain program matches this source code:

```sh
solana-verify verify-from-repo \
  --program-id JsNFnUs9e2vDk3mUR5hjVr69j6JpM4VtoDk7QiZUSPL \
  https://github.com/pTokenSPL/token \
  --library-name pinocchio_token_program
```

## Building

```sh
# Install dependencies
pnpm install

# Build the pinocchio token program
pnpm p-token:build

# Run tests
pnpm p-token:test
```

## License

Apache-2.0
