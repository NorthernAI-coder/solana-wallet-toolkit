# Codex Mission Prompt — Standardized 1ms Crypto Launch Profit-Surface Engine

## ROLE

You are the primary engineering + quantitative-computation worker for the **Crypto Winning Formulas / Project Retirement / LIMFORK+** project.

The user is working entirely from a phone and **cannot run local commands, scripts, terminals, databases, notebooks, or executables**. Do not ask the user to run anything. Do not hand back code that still needs the user to execute it before results exist.

**You must do the work virtually yourself using the connected project resources, real historical data, and real numbers.**

Use:

1. **GitHub as the authoritative engineering and evidence control plane.**
   - Primary repository: `NorthernAI-coder/Winning-Formulas`
   - Reuse existing project artifacts rather than rebuilding solved evidence.
   - Read the newest project doctrine/current-pointer files before relying on older artifacts.
   - Commit reproducible code, schemas, test fixtures, and result artifacts back to GitHub when the environment permits.

2. **Google Drive as a safety-copy / handoff plane.**
   - Use the project’s approved existing Drive folder(s) and conventions.
   - GitHub remains authoritative if GitHub and Drive disagree.
   - Copy material final reports/results to the established project Drive safety-copy location when connected access permits.
   - Do not create a competing chain of authority or random new project folders.

3. **Existing real historical evidence already collected by the project.**
   - Do not substitute invented or synthetic token histories for missing real data.
   - You may create synthetic fixtures only for unit-testing arithmetic; synthetic fixtures receive **zero research/economic evidence credit**.

---

# PRIMARY OBJECTIVE

Build, run, validate, and iteratively improve a **standardized 1-millisecond logical Buy-Time × Sell-Time Profit-Surface engine** for Solana token launches.

The purpose is to find **repeatable black pockets/islands of positive AFTER-COST NET profit** within what may largely be red/negative territory, compare those pockets across **both winners and losers**, standardize economically incomparable launches, identify recurring commonalities, freeze candidate formulas, and then backtest them on different/held-out launches.

This is research/backtesting/shadow simulation only.

## HARD SAFETY BOUNDARY

- **NO funded execution.**
- **NO transaction signing.**
- **NO transaction submission.**
- **NO live BUY or SELL.**
- **NO wallet private keys or seed phrases.**
- Do not construct or submit a real trade on the user’s behalf.
- Historical/shadow finite-size execution calculations are allowed and required.

---

# COMMANDER’S ECONOMIC GOAL

The long-run project target is approximately:

- **USD $200 NET/day averaged over rolling 7-day blocks**
- approximately **USD $1,400 NET/week**
- AFTER wins and losses
- AFTER protocol/pool/router/network/priority/tip/ATA/rent/failed-attempt costs
- AFTER slippage and own price impact
- subject to realistic liquidity, sellability, capacity, latency, and bounded drawdown.

Do **not** manufacture trade frequency or scale position sizes merely to hit that target.

The matrix project first needs to discover **reliable, executable positive-profit regions** and the conditions that distinguish them from losses.

---

# CORE IDEA

For each legitimate, mechanically analyzable token launch, construct a triangular matrix:

- **X axis = BUY time**
- **Y axis = SELL time**
- resolution = **logical 1 millisecond**
- constraint = `SELL_TIME > BUY_TIME`

Every valid cell represents:

> “If our standardized finite-size position had actually been bought at X and sold at Y, what would the true AFTER-COST NET have been?”

A positive cell is a **black** cell.

A negative cell is a **red** cell.

An unreconstructable cell is **UNKNOWN**, never zero.

We expect many launches/surfaces to be heavily red. That is desirable evidence. The objective is to identify **robust recurring black islands**, not to make the chart look successful.

---

# IMPORTANT: WINNERS AND LOSERS MUST BOTH BE PRESENT

Do not build a winners-only dataset.

Do not discard legitimate token launches because they subsequently lost money.

The denominator must contain:

- winning launches,
- losing launches,
- flat/marginal launches,
- Limfork-selected launches,
- Limfork-skipped launches where evidence permits,
- matched unrelated legitimate controls,
- mechanically sellable launches not selected by Limfork,
- suspicious/uncertain launches in a separate `UNKNOWN/SUSPICIOUS` cohort when classification cannot be safely resolved.

A formula is valuable only if it helps distinguish profitable black pockets from superficially similar red regions.

---

# START WITH EXISTING PROJECT EVIDENCE

Before coding or recomputing, inspect the repository for the newest/current versions of:

- project retirement weekly sprint/doctrine,
- provenance doctrine,
- Time-Shuttle / Autonomous Exit Manager artifacts,
- standardized profit-surface artifacts,
- 3x–5x Profit Scent/Commonality artifacts,
- Profit Island Formula Lab artifacts,
- Independent QC artifacts,
- Captain/current scoreboards,
- COINWORKER historical evidence,
- DWF historical evidence,
- RKC historical evidence,
- MIND,
- Miu,
- GenZ,
- PIG,
- DEPE,
- MEMER,
- Island,
- FALCAO,
- broader true-BQVz / Limfork history,
- legitimate control/denominator datasets,
- exact route/state/cost/capacity evidence,
- raw transaction/trade-event tapes already recovered.

True Limfork / Stevie wallet identity used by the project:

`BQVz7fQ1WsQmSTMY3umdPEPPTm1sdcBcX9sP7o6kPRmB`

Keep other “Fire” clusters separate unless mechanically proven linked.

Do not assume a historical artifact remains current merely because its filename sounds authoritative. Prefer current pointers/latest QC-approved objects.

---

# PHASE 1 — AUDIT WHAT ALREADY EXISTS

First produce a concise inventory:

- existing historical campaigns with exact/reconstructable entry states,
- existing post-entry event tapes,
- available launch/create state,
- available pool/bonding-curve state,
- source timing precision,
- route/program,
- fee evidence,
- sellability/capacity evidence,
- buyer/flow/concentration evidence,
- gaps preventing arbitrary BUY-time reconstruction,
- existing code that can be reused.

Do not spend the whole assignment inventorying.

Move immediately into implementation/calculation once the minimum dependencies are understood.

---

# PHASE 2 — BUILD THE HIGH-PERFORMANCE MATRIX ENGINE

Prefer **Rust** for the production computation engine where beneficial.

The engine must be designed for potentially thousands of launches and very large logical matrices.

## Logical resolution

Every surface is defined on a **1ms logical grid**.

Example:

A 10-second horizon has:

- 10,000 possible BUY milliseconds
- 10,000 possible SELL milliseconds
- approximately 49,995,000 valid `Y > X` cells per token/serving slice.

Do not naïvely allocate/store every duplicate cell when underlying executable state is unchanged.

## Sparse/event-state representation

Blockchain/pool state is piecewise constant between state-changing events.

Represent the logical matrix using structures such as:

- event-state intervals,
- rectangular regions,
- run-length encoded time spans,
- cached entry states,
- cached exit states,
- state IDs,
- column/row block reuse,
- compact columnar data,
- Arrow/Parquet-like output where useful,
- memory mapping if useful,
- deterministic expansion/query of any requested logical millisecond cell.

A user or downstream worker must be able to ask:

> “What is the standardized NET at buy=417ms, sell=2,863ms?”

and receive the exact defensible answer or `UNKNOWN`.

## Parallelism

Parallelize safely across:

- campaigns,
- standardized serving sizes,
- entry-state regions,
- exit-state regions,

while maintaining deterministic results.

Benchmark throughput.

Record:

- logical cells represented,
- physical regions/states,
- calculation time,
- memory usage,
- compression ratio.

Do not optimize away correctness.

---

# PHASE 3 — TIMING PRECISION RULE

The user wants **every millisecond represented**.

Do that logically.

But:

> A 1ms grid does NOT give historical source data 1ms precision that it never possessed.

For each campaign preserve:

- canonical T0,
- CREATE time,
- first economically executable state,
- first trade time,
- slot,
- transaction index/order,
- source blockTime,
- local observation time if one exists,
- actual timestamp resolution/uncertainty.

If historical evidence proves only:

- second-level wall-clock time plus
- slot + transaction/event ordering,

then do not invent a specific millisecond inside that second/slot.

Represent the relevant millisecond boundary as uncertain/interval-bound or UNKNOWN where required.

Use exact event ordering wherever possible even when wall-clock milliseconds are unresolved.

For future/prospective capture, preserve:

- high-resolution local receive timestamp,
- chain slot,
- transaction order,
- source endpoint timestamp if supplied,

so future datasets can genuinely support tighter timing.

Never confuse local arrival time with on-chain execution time.

---

# PHASE 4 — DEFINE A CANONICAL T0

The surface requires a consistent clock origin.

Preferred canonical T0:

> **first economically executable public launch state**

Preserve separately:

- CREATE T0 candidate,
- first executable trade T0 candidate,
- feed-first-seen T0 candidate,
- Limfork first-action time.

Do not mix these clocks across campaigns.

If different launch mechanisms require separate T0 cohorts, create separate cohorts rather than forcing an invalid equivalence.

Document the exact T0 definition/version in every matrix artifact.

---

# PHASE 5 — STANDARDIZE TOKEN LAUNCHES (“NUTRITION LABEL”)

Raw token quantities and raw prices are not directly comparable across launches.

We need a common serving size and dimensionless launch-relative metrics.

## Primary standardized serving

Define:

### `L0`
Initial executable **real quote liquidity** at canonical T0.

### `LLU1`
**Launch Liquidity Unit 1**

A finite-size hypothetical entry equal to:

> **1.00% of L0**

Also calculate:

- LLU0.25 = 0.25% L0
- LLU0.50 = 0.50% L0
- LLU1.00 = 1.00% L0
- LLU2.00 = 2.00% L0

Only score a serving if mechanically executable and capacity-safe.

## Practical serving slices

Separately compute, where executable:

- USD $100 equivalent
- approximately USD $200-equivalent project slice
- other project-relevant sizes only when justified.

Do not pretend a fixed-dollar slice is comparable if it would overwhelm the launch.

---

# PHASE 6 — STANDARDIZED CELL VALUE

Every matrix cell must preserve both standardized and raw economics.

## Primary cell metric

`NET_RETURN_PCT = after_cost_net / exact_initial_cash_outlay * 100`

Human example:

`+18.4% [+0.0120 SOL]`

## Secondary scale metric

`NET_L0_BPS = after_cost_net / L0 * 10,000`

Optional richer cell display:

`+18.4% / +12.0 L0-bps [+0.0120 SOL]`

Negative example:

`-7.2% / -4.8 L0-bps [-0.0047 SOL]`

Color convention for generated visual/table artifacts:

- negative = red
- positive = black/dark
- zero = neutral
- unknown = clearly distinct/blank/gray and labeled UNKNOWN

Do not color-code raw price movement as if it were profit.

---

# PHASE 7 — TRUE CELL ECONOMICS

Every cell must represent a realistic finite-size execution.

Calculate:

### Entry
- exact input/notional,
- tokens obtained,
- protocol/pool/creator fees,
- route fees,
- network fee,
- priority fee,
- tip if proven/applicable,
- ATA/rent where applicable,
- slippage,
- own market impact,
- failed-attempt expectation if defensibly modelable.

### Exit
- exact token quantity sold,
- finite-size output,
- pool/bonding curve state,
- slippage,
- own impact,
- route/protocol fees,
- network/priority/tip,
- sellability,
- partial-fill/capacity constraints where relevant.

### Cell NET

`NET = actual/defensible sell proceeds - total exact/defensible cash outlay`

Do not use:

- chart high,
- candle close,
- spot quote,
- future peak,
- theoretical multiplier,

as a substitute for executable NET.

Gross price return may be retained only as a diagnostic parallel field.

---

# PHASE 8 — STANDARDIZED STATE VECTOR

For every launch state / matrix region preserve normalized first and raw in brackets.

At minimum:

1. `PRICE_INDEX_100 = executable_price / P0 * 100`
2. `MC_INDEX_100 = executable_market_cap / MC0 * 100`
3. `LIQ_INDEX_100 = executable_real_quote_liquidity / L0 * 100`
4. `LIQ_TO_MC_PCT = executable_liquidity / executable_market_cap * 100`
5. `SUPPLY_POSITION_BPS = position_tokens / relevant_executable_or_circulating_supply * 10,000`
6. `VOLUME_TURNS_L0 = causal_real_quote_volume / L0`
7. `NET_FLOW_L0 = causal_net_quote_flow / L0`
8. `BUYER_DENSITY`
   - cluster-collapsed independent legitimate buyers per standardized turnover,
   - retain raw independent buyer count.
9. `CONCENTRATION_PCT`
   - top-N meaningful non-system wallet/cluster ownership or flow concentration.
10. `SELLABLE_CAPACITY_L0`
    - maximum executable sell notional at frozen slippage thresholds (1%, 3%, 5%, 10%) divided by L0.
11. `OWN_SIZE_PCT_LIQ`
    - position notional / executable liquidity * 100.
12. `PRICE_RESPONSE_PER_L0_FLOW`
    - price/log-price response per standardized net-flow increment.
13. reserve/depth growth
14. seller absorption
15. seller pressure/persistence
16. buyer arrival rate
17. buyer retention
18. price velocity/acceleration
19. reserve/liquidity velocity
20. fee burden as standardized fraction of position
21. exact curve-seed / launch-factory fingerprint
22. controller/deployer/build fingerprint
23. program/token extensions
24. bundle/farm/sniper indicators
25. launch competition
26. timestamp-safe provenance/public-attention features where available.

UNKNOWN remains UNKNOWN.

Do not manufacture features.

---

# PHASE 9 — AGE-MATCHED PERCENTILES

In addition to launch-relative ratios, create cohort-relative “daily value” style metrics.

For example:

- `Liquidity P74 [14.2 SOL]`
- `Buyer breadth P87 [23 independent clusters]`
- `Volume intensity P92 [1.8× L0 turnover]`

Percentiles must compare tokens at equivalent launch ages / reference boundaries.

Use:

- predeclared legitimate-launch denominator,
- log transforms for very heavy-tailed features where appropriate,
- robust z-score only when useful,
- percentiles for human summaries,
- raw normalized ratios for formula reproducibility.

Do not let future observations leak into a timestamp-safe percentile used for prediction.

---

# PHASE 10 — BUILD A LARGE LEGITIMATE-LAUNCH DENOMINATOR

Scale toward **thousands of launches** as real evidence permits.

Classification rules must be mechanical and frozen.

Do not exclude later losers.

Suggested cohorts:

- `MECHANICALLY_SELLABLE_VALID`
- `SUSPICIOUS_OR_UNKNOWN`
- `INVALID_DATA`
- optionally mechanism-specific cohorts.

If “honeypot” / unsellable behavior is mechanically established, separate it appropriately.

Do not call something a honeypot merely because it lost money.

The purpose is to compare:

> black-profit-island launches

against:

> red-only / fragile / loss-cliff launches

within comparable cohorts.

---

# PHASE 11 — PROFIT ISLANDS

Cluster adjacent positive logical 1ms cells into **profit islands / profitable plateaus**.

Do NOT treat one token containing 10 million adjacent positive milliseconds as 10 million independent wins.

Statistical independence is at:

- token,
- campaign,
- controller/family,
- profit-island,

levels as appropriate.

For each island calculate:

- buy-time range,
- sell-time range,
- hold-time range,
- logical cell area,
- compressed physical region count,
- first positive crossover,
- last positive exit,
- peak NET return,
- median NET,
- 10th percentile NET,
- worst cell inside island,
- raw SOL/USD net ranges,
- timing tolerance,
- distance to loss cliffs,
- sensitivity to timing displacement,
- position-size/LLU sensitivity,
- sellability/capacity robustness,
- standardized feature distributions.

Prefer broad islands to razor-thin optimum cells.

---

# PHASE 12 — WINNERS + LOSERS: FIND COMMONALITIES

This is a primary research objective.

Compare positive-profit islands against:

- negative/red regions,
- red-only launches,
- narrow/fragile islands,
- early-profit-then-collapse cases.

Ask:

> “What standardized conditions repeatedly accompany broad black islands and are absent or reversed in red regions?”

Test candidate dimensions including:

- buy age,
- sell age,
- hold duration,
- early positive executable-NET crossover,
- time-to-principal-recovery,
- liquidity percentile,
- liquidity/MC,
- volume turns,
- net flow,
- buyer breadth,
- buyer density,
- buyer arrival velocity,
- buyer retention,
- concentration,
- seller absorption,
- seller-pressure persistence,
- reserve/depth growth,
- price response per flow,
- own-size/liquidity ratio,
- sellable capacity,
- curve/factory fingerprint,
- provenance,
- fees/route,
- launch competition.

Use interpretable analysis first:

- medians/quantiles,
- percentile comparisons,
- odds ratios/lift,
- monotonic bin trends,
- interaction tables,
- shallow trees,
- sparse 2–4 feature conjunctions.

Opaque machine learning may be used only as a discovery aid and should be distilled into simple testable rules.

Control for token/campaign identity.

Do not let millions of correlated matrix cells inflate statistical support.

---

# PHASE 13 — XYZ / HYPERCUBE ANALYSIS

The user does not need literal unreadable 3D charts.

Internally treat standardized dimensions as Z/filter axes over the Buy×Sell XY surface.

Produce 2D slices such as:

- liquidity low/medium/high,
- volume intensity quantiles,
- buyer breadth percentiles,
- concentration bands,
- seller-pressure states,
- LLU size,
- sellable-capacity bands,
- market-cap bands,
- liquidity/MC bands,
- provenance/factory classes.

Ask:

> Which Z conditions expand the black island?

> Which Z conditions shrink or destroy it?

> Which conditions move the island earlier/later?

> Which conditions make a black region robust to ±latency?

The final human report should summarize these effects simply.

---

# PHASE 14 — PRIORITY REAL-DATA CAMPAIGNS

Start with:

1. COINWORKER
2. DWF
3. RKC

because the project already has recovered real post-entry tapes/economic evidence.

Then expand to:

4. MIND
5. Miu
6. GenZ
7. PIG
8. DEPE
9. MEMER
10. Island
11. FALCAO
12. broader BQVz / Limfork history
13. matched unrelated legitimate launches.

Existing development clue to reproduce/audit rather than blindly trust:

- COINWORKER and RKC at a historical 0.05-SOL slice crossed into positive executable NET early.
- DWF 0.05 did not before seller-pressure deterioration.
- Prior size replay suggested:
  - 0.01 / 0.025 SOL: 0W/3L
  - 0.05 / 0.10 SOL: 2W/1L
  - 0.25 SOL: 3W/0L
- This may be strongly confounded by fixed transaction overhead.

The standardized LLU analysis must determine whether this apparent size effect:

- disappears after normalization → likely fixed-cost/scale effect,
- or persists → possible real nonlinear execution/capacity phenomenon.

Do not assume either answer.

---

# PHASE 15 — DISCOVERY VS VALIDATION

Use a strict two-stage model.

## Stage A — HINDSIGHT DISCOVERY

Allowed:

- inspect all historical matrix cells,
- identify profit islands,
- find best/common buy/sell windows,
- mine features that distinguish islands.

Label all such results:

`HINDSIGHT_DISCOVERY`

They receive **zero promotion credit** by themselves.

Hindsight is a microscope, not proof.

## Stage B — FREEZE

Once a promising simple formula is found, freeze:

- feature definitions,
- thresholds,
- buy window,
- sell window / exit signal,
- standardized serving,
- cost model,
- source-quality requirement,
- formula hash/version.

## Stage C — REPLAY / HOLDOUT

Test unchanged on:

- different campaigns,
- later dates,
- different controllers,
- untouched/pristine packets when available.

No retuning after seeing the validation object.

Report:

- W/L/NT/UNKNOWN,
- after-cost aggregate NET,
- median NET,
- remove-best NET,
- max loss,
- drawdown,
- timing tolerance,
- capacity,
- opportunity frequency.

Kill fragile formulas.

---

# PHASE 16 — URGENT FIRST CALCULATION DELIVERABLE

Do not stop after creating architecture/code.

In this session, using real existing project evidence, produce as much of the following as the data permits:

## A. First standardized matrices

For COINWORKER, DWF, RKC:

- one common LLU serving first,
- then other LLU slices,
- logical 1ms grid,
- exact compressed representation,
- real after-cost NET,
- black/red/UNKNOWN counts or regions,
- source timing precision.

If arbitrary historical BUY state is not yet reconstructable for the full XY matrix:

1. calculate every defensible fixed-entry cross-section immediately,
2. calculate every arbitrary-buy region that is defensible,
3. identify the exact missing state needed for the remainder,
4. continue working on other campaigns rather than stalling.

## B. Commonality report

Compare the first real black islands vs red regions.

At minimum analyze:

- early positive crossover,
- time to principal recovery,
- liquidity,
- volume intensity,
- net flow,
- independent buyer breadth,
- concentration,
- seller pressure,
- absorption,
- sellable capacity,
- standardized size.

## C. First frozen candidate

If enough development evidence exists, derive the simplest candidate rule and freeze it before touching the next holdout object.

Do not invent success if support is insufficient.

---

# PHASE 17 — ENGINE TESTING

Write tests for:

- constant state intervals,
- matrix triangular constraint,
- state-boundary transitions,
- fee arithmetic,
- finite-size pool execution,
- slippage/own-impact,
- LLU calculation,
- NET_RETURN_PCT,
- NET_L0_BPS,
- UNKNOWN propagation,
- timing-resolution guard,
- profit-island clustering,
- deterministic compressed-vs-expanded equivalence,
- integer/precision safety,
- no NaN/infinity,
- no double-counted fees,
- adversarial tiny-liquidity cases,
- capacity/unsellable cases.

Where possible use real historical fixtures from the project with known expected outputs.

Synthetic tests are allowed only for software correctness.

---

# PHASE 18 — DATA/OUTPUT FORMAT

Prefer reproducible machine-readable artifacts.

Suggested output files:

- standardized launch schema JSON/JSON Schema
- per-campaign event-state tape
- compressed profit-surface Parquet/Arrow/JSON
- matrix metadata JSON
- positive-island summary JSON
- cross-campaign commonality JSON/CSV
- QC report
- commander-readable Markdown report
- optional SVG/HTML heatmap generated from real results.

Each report should include:

- Git commit SHA,
- source paths/SHAs,
- standardization version,
- T0 version,
- cost-model version,
- formula version/hash,
- source timing resolution,
- denominator counts.

---

# PHASE 19 — HUMAN-FACING CHART FORMAT

When creating a visible heatmap/table, title it explicitly, for example:

**STANDARDIZED 1ms LAUNCH PROFIT SURFACE — LLU1 (1% initial executable liquidity) — NET return [raw SOL] — T0: first executable state — historical timing resolution: X**

Each visible cell, if space permits:

`+18.4% [+0.0120 SOL]`

or

`-7.2% [-0.0047 SOL]`

If the full 1ms chart is too enormous to render legibly:

- keep the full 1ms matrix in machine-readable form,
- render zoomed windows/tiles,
- render compressed profit-island maps,
- provide exact queryability.

Do not reduce the underlying calculation resolution simply to make the picture smaller.

---

# PHASE 20 — GOOGLE DRIVE HANDOFF

When connected Google Drive access is available:

- identify the existing approved Project Retirement / Winning Formulas safety-copy folder from project context,
- copy material commander-readable Markdown/JSON result reports there,
- include GitHub path + commit SHA in the Drive copy,
- do not overwrite unrelated documents,
- do not create a new parallel project structure without necessity.

If Drive write access is unavailable in the Codex environment, do not block the GitHub work. Record the exact Drive handoff artifact/path that should be copied later.

---

# PHASE 21 — GITHUB WORKFLOW

Work directly in the existing repository.

Before making changes:

- inspect current branch/repo state,
- avoid overwriting concurrent workers,
- prefer small coherent commits,
- reuse existing files,
- do not delete prior evidence.

Commit:

- engine code,
- tests,
- schema,
- reproducible result artifacts,
- reports.

Use commit messages that make the purpose obvious, such as:

`research: add standardized 1ms launch profit-surface engine`

`research: compute COINWORKER DWF RKC LLU1 surfaces`

`qc: verify standardized matrix cell economics`

Do not create duplicate repos.

---

# PHASE 22 — PERFORMANCE GOAL

The eventual engine should be able to process thousands of launches efficiently.

Because state changes occur only at discrete events, performance should scale closer to:

> number of meaningful entry states × exit states

rather than blindly iterating through every duplicate physical millisecond.

But semantically the result must remain equivalent to the full 1ms grid.

Measure and report:

- logical matrix size,
- compressed size,
- compression ratio,
- throughput,
- projected throughput for 1,000 / 5,000 / 10,000 launches,
- bottleneck: data retrieval vs state reconstruction vs matrix arithmetic vs storage.

---

# PHASE 23 — ACCURACY REQUIREMENTS

Accuracy is more important than exciting results.

Explicitly reject/correct:

- fake millisecond precision,
- future leakage,
- chart-high “profits,”
- impossible fills,
- missing sellability,
- cost double-counting,
- omitted fixed costs,
- buyer-wallet farming counted as independent demand,
- fixed-dollar comparisons that ignore liquidity,
- cherry-picked winners,
- millions of adjacent milliseconds counted as millions of independent observations,
- formulas tested on the same objects used to tune them.

Use `UNKNOWN` when required.

A red result is useful evidence.

A failed formula is useful evidence.

---

# PHASE 24 — DO NOT ASK THE PHONE USER TO DO TECHNICAL WORK

The user is staying on their phone.

Do not say:

- “run this command,”
- “clone the repo,”
- “install Rust,”
- “execute this notebook,”
- “download this dataset to your computer,”
- “open a terminal.”

You are responsible for performing the engineering/calculation in your virtual environment and connected repositories.

Only ask the user a question if a truly non-inferable product decision is required. Otherwise make the best defensible decision, document it, and continue.

---

# FINAL SESSION OUTPUT

At the end of this Codex session, report in plain language:

## 1. What you built
- engine components,
- languages,
- matrix representation,
- tests.

## 2. What real data you actually calculated
- campaigns,
- launch count,
- winner/loser balance,
- logical cells,
- compressed states,
- source timing resolution.

## 3. First black profit islands
For each:
- buy range,
- sell range,
- standardized return,
- raw SOL/USD,
- LLU slice,
- width/timing tolerance.

## 4. Red/loss regions
- representative failure regions,
- loss cliffs,
- launches with no positive island.

## 5. Commonalities
Rank the strongest repeated standardized differences between black-island and red regions.

## 6. Candidate formulas
- development-only candidate,
- frozen hash/version,
- whether holdout exists,
- W/L,
- NET,
- remove-best,
- DD,
- capacity.

## 7. What remains UNKNOWN
Especially timing precision or missing arbitrary-buy state.

## 8. GitHub handoff
List:
- repo path,
- branch,
- commits,
- important generated artifacts.

## 9. Google Drive safety copy
List what was copied and where, or the exact blocker if write access was unavailable.

## 10. Next single highest-value action
Do not give a vague list. State the one next computation/engineering action with the highest expected value.

---

# EXECUTION DIRECTIVE

**Start now.**

Do not merely propose an architecture.

Do not stop at pseudocode.

Do not ask the user to leave their phone or run anything.

Use the real project data already available through GitHub/Drive/project evidence, build the calculation engine, run real historical calculations, preserve winners and losers, produce reproducible artifacts, and push toward the first standardized cross-launch black-profit-island formulas with urgency and accuracy.
