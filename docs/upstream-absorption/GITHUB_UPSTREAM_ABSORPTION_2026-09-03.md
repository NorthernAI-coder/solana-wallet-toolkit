# Upstream Absorption — Solana Wallet Toolkit — 2026-09-03

## Immediate modernization note
The README currently describes `solana-labs/solana` as the official Rust SDK source. That upstream is archived; active Solana validator/client development has moved to **anza-xyz/agave**. Do not blindly replace cryptographic primitives, but update dependency provenance/security assumptions to current maintained packages.

## anza-xyz/agave — Apache-2.0
Use as the primary maintained upstream reference for current Solana runtime/client behavior.

### Actions
- Audit current Rust Solana crate versions and replace archived-source assumptions with maintained Anza/Solana package provenance where appropriate.
- Pin exact versions/lockfiles for security-sensitive builds.
- Add compatibility tests against current CLI/keypair format and transaction signing.
- Preserve the toolkit's offline/air-gapped key-generation path; network-facing additions must be separate modules.

## Jupiter Swap API V2 — documentation reference
Use current Jupiter developer docs for swap/execution integration, but the `jup-ag/docs` repo itself has no declared OSS license in repository metadata, so treat it as documentation/reference rather than copyable source.

### If transaction/swap functionality is added
- Isolate quote/order/build/execute behind a `SwapProvider` interface.
- Never expose secret keys to remote APIs.
- Validate transaction message, writable accounts, program IDs, fees, slippage and destination before signing.
- Simulate where available before broadcast.
- Add hard slippage/fee/value limits and a kill switch.
- Treat network/API data as untrusted input.

## rpcpool/yellowstone-grpc
Whole repository is AGPL-3.0; its explicitly Apache-2.0 client/proto/example portions may be evaluated for a proprietary client. **Do not copy AGPL server/plugin implementation into a closed-source wallet without an explicit licensing decision.**

### Streaming use
For future live wallet/trading telemetry, keep Yellowstone as a read-only data source separated from signing/execution. Reconcile critical observations against ordinary RPC before value-moving actions.

## Security gates
- key creation/signing remains offline-capable
- zero logging of private key material
- strict transaction decoding/allowlisting before signing
- no AI/agent may bypass deterministic risk controls
- fuzz tests for transaction parsing and hostile RPC/API responses
- current maintained Solana/Agave compatibility documented in CI

## Free/commercial rule
Agave and the Apache-only Yellowstone client/proto portions are commercially friendly. Jupiter docs are reference-only unless a license grants code/text reuse. No mandatory paid RPC/service may be required for the basic wallet.