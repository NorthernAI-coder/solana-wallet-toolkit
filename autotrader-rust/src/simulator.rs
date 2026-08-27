use crate::policy::{
    EvidenceBinding, ExecutionEvidence, ExecutionMode, MarketEvidence, ModelProposal, Phenotype,
    PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig, TimelineEvidence, TokenEvidence,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationReport {
    pub iterations: u64,
    pub valid_entry_checks: u64,
    pub invalid_entry_checks: u64,
    pub valid_exit_checks: u64,
    pub emergency_exit_checks: u64,
    pub false_accepts: u64,
    pub false_rejects: u64,
    pub emergency_route_failures_caught: u64,
}

impl SimulationReport {
    pub fn passed(&self) -> bool {
        self.false_accepts == 0 && self.false_rejects == 0
    }

    pub fn total_checks(&self) -> u64 {
        self.valid_entry_checks
            + self.invalid_entry_checks
            + self.valid_exit_checks
            + self.emergency_exit_checks
    }
}

#[derive(Clone, Copy, Debug)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    fn range_u64(&mut self, low: u64, high_inclusive: u64) -> u64 {
        if low >= high_inclusive {
            return low;
        }
        low + self.next_u64() % (high_inclusive - low + 1)
    }

    fn range_u32(&mut self, low: u32, high_inclusive: u32) -> u32 {
        self.range_u64(low as u64, high_inclusive as u64) as u32
    }
}

type Fixture = (
    ModelProposal,
    EvidenceBinding,
    TokenEvidence,
    MarketEvidence,
    ExecutionEvidence,
    PortfolioState,
);

pub fn run_adversarial_simulation(iterations: u64) -> SimulationReport {
    let engine = PolicyEngine::new(RiskConfig::default());
    let mut rng = Lcg::new(0xF07E_55A5_D15C_A11E);
    let mut report = SimulationReport {
        iterations,
        ..SimulationReport::default()
    };

    for i in 0..iterations {
        let (proposal, binding, token, market, execution, portfolio) = valid_entry_fixture(&mut rng, i);

        report.valid_entry_checks += 1;
        if !engine
            .evaluate(
                &proposal,
                &binding,
                &token,
                &market,
                &execution,
                &portfolio,
            )
            .allowed
        {
            report.false_rejects += 1;
        }

        let mutation = rng.next_u64() % 31;
        let (bad_p, bad_b, bad_t, bad_m, bad_e, bad_s) = mutate_entry_to_fail(
            mutation,
            proposal,
            binding,
            token,
            market,
            execution,
            portfolio,
            &engine,
        );
        report.invalid_entry_checks += 1;
        if engine
            .evaluate(&bad_p, &bad_b, &bad_t, &bad_m, &bad_e, &bad_s)
            .allowed
        {
            report.false_accepts += 1;
        }

        let (exit_p, exit_b, exit_t, exit_m, exit_e, exit_s) =
            valid_exit_fixture(&mut rng, i, false);
        report.valid_exit_checks += 1;
        if !engine
            .evaluate(&exit_p, &exit_b, &exit_t, &exit_m, &exit_e, &exit_s)
            .allowed
        {
            report.false_rejects += 1;
        }

        let (em_p, em_b, em_t, em_m, em_e, em_s) = valid_exit_fixture(&mut rng, i, true);
        report.emergency_exit_checks += 1;
        if !engine
            .evaluate(&em_p, &em_b, &em_t, &em_m, &em_e, &em_s)
            .allowed
        {
            report.false_rejects += 1;
        }

        let mut stale_emergency = em_e;
        stale_emergency.timeline.quote_at_ms = stale_emergency.timeline.signal_at_ms;
        let stale_report = engine.evaluate(
            &em_p,
            &em_b,
            &em_t,
            &em_m,
            &stale_emergency,
            &em_s,
        );
        if stale_report.allowed {
            report.false_accepts += 1;
        } else if stale_report.reasons.contains(&Rejection::QuoteNotAfterSignal) {
            report.emergency_route_failures_caught += 1;
        }
    }

    report
}

fn valid_entry_fixture(rng: &mut Lcg, i: u64) -> Fixture {
    let nav = rng.range_u64(100_000, 10_000_000);
    let max_position = nav / 10;
    let notional = rng.range_u64(1, max_position.max(1));
    let max_total = nav.saturating_mul(4) / 10;
    let total_exposure = rng.range_u64(0, max_total.saturating_sub(notional));
    let market_cap = rng.range_u64(2_500_000, 30_000_000);
    let min_absorption = market_cap.saturating_mul(20) / 100;
    let volume = rng.range_u64(500_000.max(min_absorption), market_cap.max(500_000));
    let signal = 1_000_000u64.saturating_add(i.saturating_mul(100));
    let quote = signal.saturating_add(rng.range_u64(1, 5_000));
    let now = quote.saturating_add(rng.range_u64(0, 15_000));
    let proposal = ModelProposal {
        mint: format!("SimMint{i:020}"),
        purpose: Purpose::Entry,
        mode: match i % 3 {
            0 => ExecutionMode::Paper,
            1 => ExecutionMode::Shadow,
            _ => ExecutionMode::Devnet,
        },
        notional_cents: notional,
        phenotype: match i % 9 {
            0 => Phenotype::Builder,
            1 => Phenotype::Provenance,
            2 => Phenotype::ControllerBehavior,
            3 => Phenotype::GithubPreToken,
            4 => Phenotype::PhoenixMigration,
            5 => Phenotype::DexFirstLaunch,
            6 => Phenotype::CreatorApprovedMeme,
            7 => Phenotype::StructuralDemand,
            _ => Phenotype::ResidualValue,
        },
        reason: "simulation fixture".into(),
    };
    let binding = EvidenceBinding::new(&proposal.mint, &proposal.mint, &proposal.mint);

    (
        proposal,
        binding,
        TokenEvidence {
            exact_mint_verified: true,
            security_gate_passed: true,
            dex_first_verified: true,
            entry_trigger_confirmed: true,
        },
        MarketEvidence {
            liquidity_cents: rng.range_u64(1_000_000, 20_000_000),
            volume_24h_cents: volume,
            market_cap_cents: market_cap,
            geckoterminal_score: rng.range_u64(56, 100) as u8,
            independent_price_sources: rng.range_u64(2, 5) as u8,
        },
        ExecutionEvidence {
            route_verified: true,
            slippage_bps: rng.range_u32(0, 300),
            price_impact_bps: rng.range_u32(0, 200),
            route_fee_bps: rng.range_u32(0, 500),
            price_divergence_bps: rng.range_u32(0, 500),
            timeline: TimelineEvidence {
                observed_at_ms: signal.saturating_sub(10_000),
                armed_at_ms: signal.saturating_sub(5_000),
                signal_at_ms: signal,
                quote_at_ms: quote,
                now_ms: now,
            },
        },
        PortfolioState {
            nav_cents: nav,
            available_cash_cents: nav.saturating_sub(total_exposure),
            total_exposure_cents: total_exposure,
            daily_realized_loss_cents: rng.range_u64(
                0,
                (nav.saturating_mul(5) / 100).saturating_sub(1),
            ),
            open_positions: rng.range_u64(0, 4) as u16,
            current_position_value_cents: notional,
            entry_halt_active: false,
            global_kill_switch_active: false,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn mutate_entry_to_fail(
    mutation: u64,
    mut p: ModelProposal,
    mut b: EvidenceBinding,
    mut t: TokenEvidence,
    mut m: MarketEvidence,
    mut e: ExecutionEvidence,
    mut s: PortfolioState,
    engine: &PolicyEngine,
) -> Fixture {
    let c = engine.config();
    match mutation {
        0 => s.global_kill_switch_active = true,
        1 => s.entry_halt_active = true,
        2 => e.route_verified = false,
        3 => e.timeline.quote_at_ms = e.timeline.signal_at_ms,
        4 => e.timeline.quote_at_ms = e.timeline.now_ms.saturating_add(1),
        5 => {
            e.timeline.now_ms = e
                .timeline
                .quote_at_ms
                .saturating_add(c.max_quote_age_ms)
                .saturating_add(1)
        }
        6 => e.timeline.armed_at_ms = e.timeline.signal_at_ms.saturating_add(1),
        7 => e.price_divergence_bps = c.max_price_divergence_bps.saturating_add(1),
        8 => m.independent_price_sources = c.min_independent_price_sources.saturating_sub(1),
        9 => b.token_evidence_mint.push('X'),
        10 => b.market_evidence_mint.push('X'),
        11 => b.execution_evidence_mint.push('X'),
        12 => t.exact_mint_verified = false,
        13 => t.security_gate_passed = false,
        14 => t.dex_first_verified = false,
        15 => t.entry_trigger_confirmed = false,
        16 => m.liquidity_cents = c.min_liquidity_cents.saturating_sub(1),
        17 => m.volume_24h_cents = c.min_volume_24h_cents.saturating_sub(1),
        18 => m.market_cap_cents = 0,
        19 => m.market_cap_cents = c.max_market_cap_cents.saturating_add(1),
        20 => {
            m.market_cap_cents = c.max_market_cap_cents.max(10_000);
            m.volume_24h_cents = m.market_cap_cents.saturating_mul(19) / 100;
        }
        21 => m.geckoterminal_score = c.min_geckoterminal_score.saturating_sub(1),
        22 => e.slippage_bps = c.max_slippage_bps.saturating_add(1),
        23 => e.price_impact_bps = c.max_price_impact_bps.saturating_add(1),
        24 => e.route_fee_bps = c.max_route_fee_bps.saturating_add(1),
        25 => s.daily_realized_loss_cents = s.nav_cents.saturating_mul(5) / 100,
        26 => p.notional_cents = (s.nav_cents / 10).saturating_add(1),
        27 => {
            p.notional_cents = 1;
            s.total_exposure_cents = s.nav_cents.saturating_mul(4) / 10;
        }
        28 => s.open_positions = c.max_open_positions,
        29 => s.available_cash_cents = p.notional_cents.saturating_sub(1),
        _ => p.notional_cents = 0,
    }
    (p, b, t, m, e, s)
}

fn valid_exit_fixture(rng: &mut Lcg, i: u64, emergency: bool) -> Fixture {
    let signal = 10_000_000u64.saturating_add(i.saturating_mul(100));
    let quote = signal.saturating_add(rng.range_u64(1, 2_000));
    let position = rng.range_u64(1_000, 100_000);
    let amount = rng.range_u64(1, position);
    let purpose = if emergency {
        Purpose::EmergencyExit
    } else {
        Purpose::Exit
    };
    let (slippage_cap, impact_cap, fee_cap, divergence_cap, sources) = if emergency {
        (1_500, 1_200, 1_000, 2_000, 1)
    } else {
        (300, 200, 500, 500, 2)
    };
    let proposal = ModelProposal {
        mint: format!("ExitMint{i:019}"),
        purpose,
        mode: if i % 2 == 0 {
            ExecutionMode::Paper
        } else {
            ExecutionMode::Shadow
        },
        notional_cents: amount,
        phenotype: Phenotype::Other,
        reason: if emergency {
            "emergency risk reduction"
        } else {
            "risk reduction"
        }
        .into(),
    };
    let binding = EvidenceBinding::new(&proposal.mint, &proposal.mint, &proposal.mint);

    (
        proposal,
        binding,
        TokenEvidence {
            exact_mint_verified: false,
            security_gate_passed: false,
            dex_first_verified: false,
            entry_trigger_confirmed: false,
        },
        MarketEvidence {
            liquidity_cents: 0,
            volume_24h_cents: 0,
            market_cap_cents: 0,
            geckoterminal_score: 0,
            independent_price_sources: sources,
        },
        ExecutionEvidence {
            route_verified: true,
            slippage_bps: rng.range_u32(0, slippage_cap),
            price_impact_bps: rng.range_u32(0, impact_cap),
            route_fee_bps: rng.range_u32(0, fee_cap),
            price_divergence_bps: rng.range_u32(0, divergence_cap),
            timeline: TimelineEvidence {
                observed_at_ms: signal.saturating_sub(5_000),
                armed_at_ms: signal.saturating_add(100),
                signal_at_ms: signal,
                quote_at_ms: quote,
                now_ms: quote.saturating_add(rng.range_u64(0, 15_000)),
            },
        },
        PortfolioState {
            nav_cents: 100_000,
            available_cash_cents: 0,
            total_exposure_cents: position,
            daily_realized_loss_cents: 100_000,
            open_positions: 5,
            current_position_value_cents: position,
            entry_halt_active: true,
            global_kill_switch_active: false,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adversarial_smoke_test_has_no_policy_misses() {
        let report = run_adversarial_simulation(20_000);
        assert!(report.passed(), "simulation failures: {report:?}");
        assert_eq!(report.total_checks(), 80_000);
        assert_eq!(report.emergency_route_failures_caught, 20_000);
    }
}
