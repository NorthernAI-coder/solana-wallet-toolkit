# Autotrader Fortress

A deterministic Rust risk firewall and adversarial simulator for an AI-assisted Solana **paper, shadow, and devnet** trading system.

## Safety boundary

This crate intentionally contains:

- no private-key loading
- no seed phrase handling
- no transaction signing
- no mainnet send/broadcast function
- no API that lets an LLM modify risk limits
- no API that lets an LLM declare security/market evidence as true

The AI layer may propose a trade intent. Trusted adapters and deterministic Rust policy code decide whether that intent is admissible for simulation/shadow execution.

## Architecture

```text
Market / chain / project evidence
        |
        v
Trusted evidence adapters -----> TokenEvidence / MarketEvidence / ExecutionEvidence
                                      |
OpenAI model -> ModelProposal --------+----> PolicyEngine ----> paper/shadow/devnet action
                                             |
                                             +----> immutable audit record
```

The model proposal is deliberately narrow:

- token mint
- entry / exit / emergency-exit intent
- paper / shadow / devnet mode
- proposed notional
- phenotype
- explanation

It cannot self-certify the mint, security status, DEX-first status, liquidity, volume, market cap, price-source agreement, route quality, quote freshness, portfolio exposure, or risk state.

## Core invariants

1. **Signal cannot equal fill.** A quote must be strictly later than the signal that caused the action.
2. **Fresh execution evidence.** Quotes from the future or older than the configured maximum age are rejected.
3. **Entry rules never trap exits.** Liquidity, market-cap, DEX-first, GT-score and entry-trigger gates apply to entries, not risk-reducing exits.
4. **Two stop controls.** `entry_halt_active` stops new entries while preserving exits; `global_kill_switch_active` freezes every action.
5. **Emergency escape profile.** Emergency exits tolerate distressed slippage, price impact, fee and source divergence more than entries while still requiring a verified fresh route.
6. **Integer money math.** Dollar amounts are integer cents and percentages are basis points. Exposure calculations use checked arithmetic and `u128` intermediates.
7. **Hard portfolio limits.** Position size, total exposure, daily loss and open-position limits are deterministic policy gates.
8. **Exact identity and provenance.** Entry requires exact-mint verification, security approval, DEX-first verification and a confirmed entry trigger supplied by trusted evidence collectors.
9. **No score rescues a failed gate.** A high score or persuasive model explanation cannot override a hard rejection.

## Default entry policy

The current conservative defaults are deliberately explicit and testable:

| Gate | Default |
|---|---:|
| Maximum position | 10% of NAV |
| Maximum total exposure | 40% of NAV |
| Daily realized-loss halt | 5% of NAV |
| Maximum entry slippage | 3% |
| Maximum entry price impact | 2% |
| Minimum liquidity | $10,000 |
| Minimum 24h volume | $5,000 |
| Maximum market cap | $300,000 |
| Minimum volume / market-cap absorption | 20% |
| Minimum GeckoTerminal score | 56 |
| Minimum independent price sources | 2 |
| Maximum price-source divergence | 5% |
| Maximum quote age | 15 seconds |
| Maximum open positions | 5 |

These are policy defaults, not claims that the strategy is profitable. They should be changed only through deliberate versioned policy changes and then re-audited out-of-sample.

## Phenotype routing

`Phenotype` keeps multiple candidate types eligible without allowing one scoring formula to erase rare winners:

- Builder
- Provenance
- Controller behavior
- GitHub pre-token
- Phoenix / migration
- DEX-first launch
- Creator-approved meme
- Structural demand
- Residual value
- Other

Phenotype classification is descriptive context. Hard safety and execution gates remain common.

## Run tests

```bash
cargo test --manifest-path autotrader-rust/Cargo.toml
```

## Run adversarial simulation

Default: one million iterations.

```bash
cargo run --release \
  --manifest-path autotrader-rust/Cargo.toml \
  --bin fortress-sim -- 1000000
```

Each iteration evaluates:

- a randomized valid entry that should pass
- a randomized corrupted entry that must fail
- a normal risk-reducing exit that should pass even with entry-only gates deliberately broken
- an emergency exit under distressed conditions that should pass within the emergency envelope
- an additional stale/same-signal emergency-route attack that must fail

The simulation exits non-zero on any false accept or false reject.

## OpenAI integration contract

The model-facing tool should create only a `ModelProposal`. Trusted application code must populate evidence structures independently before calling `PolicyEngine::evaluate`.

Recommended model tool semantics:

```text
propose_trade(
  mint,
  purpose,
  mode,
  notional_cents,
  phenotype,
  reason
)
```

Do **not** expose setters such as `security_gate_passed=true`, `route_verified=true`, `liquidity=...`, or risk-limit modification to the model.

## Audit discipline

Every strategy version should retain:

- strategy/policy version hash
- observation timestamp
- ARMED timestamp when applicable
- signal timestamp
- next executable quote timestamp
- quote source and route identity
- simulated slippage, price impact and fees
- independent price-source observations
- model proposal
- deterministic accept/reject report
- eventual paper/shadow outcome

Historical tests must be time-causal: information discovered after a decision timestamp cannot be used to justify that earlier decision.
