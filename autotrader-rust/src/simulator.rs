use crate::policy::{
    EvidenceBinding, ExecutionEvidence, ExecutionMode, MarketEvidence, ModelProposal, Phenotype,
    PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig, TimelineEvidence, TokenEvidence,
    TokenProgram,
};

const ENTRY_MUTATION_CLASSES: u64 = 57;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationReport {
    pub iterations: u64,
    pub entry_mutation_classes: u64,
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
        entry_mutation_classes: ENTRY_MUTATION_CLASSES,
        ..SimulationReport::default()
    };

    for i in 0..iterations {
        let (proposal, binding, token, market, execution, portfolio) =
            valid_entry_fixture(&mut rng, i);
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

        let mutation = i % ENTRY_MUTATION_CLASSES;
        let (bad_p, bad_b, bad_t, bad_m, bad_e, bad_s) = mutate_entry_to_fail(
            mutation, proposal, binding, token, market, execution, portfolio, &engine,
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
        stale_emergency.timeline.quote_at_ms = stale_emergency.timeline.decision_recorded_at_ms;
        let stale_report =
            engine.evaluate(&em_p, &em_b, &em_t, &em_m, &stale_emergency, &em_s);
        if stale_report.allowed {
            report.false_accepts += 1;
        } else if stale_report
            .reasons
            .contains(&Rejection::QuoteNotAfterDecision)
        {
            report.emergency_route_failures_caught += 1;
        }
    }

    report
}

fn valid_entry_fixture(rng: &mut Lcg, i: u64) -> Fixture {
    let nav = rng.range_u64(100_000, 10_000_000);
    let max_position = nav / 10;
    let current_position = if i.is_multiple_of(3) {
        rng.range_u64(1, (max_position / 2).max(1))
    } else {
        0
    };
    let notional = rng.range_u64(1, max_position.saturating_sub(current_position).max(1));
    let max_total = nav.saturating_mul(4) / 10;
    let max_existing_exposure = max_total.saturating_sub(notional);
    let total_exposure = rng.range_u64(current_position, max_existing_exposure);
    let market_cap = rng.range_u64(2_500_000, 30_000_000);
    let min_absorption = market_cap.saturating_mul(20) / 100;
    let volume = rng.range_u64(500_000.max(min_absorption), market_cap.max(500_000));
    let signal = 1_000_000u64.saturating_add(i.saturating_mul(100));
    let decision = signal.saturating_add(rng.range_u64(0, 2_000));
    let quote = decision.saturating_add(rng.range_u64(1, 5_000));
    let now = quote.saturating_add(rng.range_u64(0, 15_000));
    let network_fee = rng.range_u64(0, 100);
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
    let binding = EvidenceBinding::new(
        proposal.mint.as_str(),
        proposal.mint.as_str(),
        proposal.mint.as_str(),
    );
    let execution_mode = proposal.mode;
    let execution_purpose = proposal.purpose;
    let quoted_notional_cents = proposal.notional_cents;
    let position_mint = (current_position > 0).then(|| proposal.mint.clone());
    let open_positions = if current_position > 0 {
        rng.range_u64(1, 5) as u16
    } else {
        rng.range_u64(0, 4) as u16
    };

    (
        proposal,
        binding,
        TokenEvidence {
            checked_at_ms: decision.saturating_sub(rng.range_u64(0, 300_000)),
            exact_mint_verified: true,
            token_program: if i.is_multiple_of(2) {
                TokenProgram::Legacy
            } else {
                TokenProgram::Token2022
            },
            mint_authority_present: false,
            freeze_authority_present: false,
            permanent_delegate_present: false,
            non_transferable: false,
            default_account_state_frozen: false,
            transfer_hook_present: false,
            transfer_hook_program_verified: false,
            pausable: false,
            confidential_transfer_enabled: false,
            scaled_ui_amount_enabled: false,
            current_transfer_fee_bps: rng.range_u32(0, 500),
            dex_first_verified: true,
            entry_trigger_confirmed: true,
        },
        MarketEvidence {
            as_of_ms: decision.saturating_sub(rng.range_u64(0, 60_000)),
            liquidity_cents: rng.range_u64(1_000_000, 20_000_000),
            volume_24h_cents: volume,
            market_cap_cents: market_cap,
            geckoterminal_score: rng.range_u64(56, 100) as u8,
            independent_price_sources: rng.range_u64(2, 5) as u8,
        },
        ExecutionEvidence {
            mode: execution_mode,
            purpose: execution_purpose,
            quoted_notional_cents,
            route_verified: true,
            preflight_simulation_passed: true,
            reverse_sell_simulation_passed: true,
            estimated_network_fee_cents: network_fee,
            slippage_bps: rng.range_u32(0, 300),
            price_impact_bps: rng.range_u32(0, 200),
            route_fee_bps: rng.range_u32(0, 500),
            price_divergence_bps: rng.range_u32(0, 500),
            timeline: TimelineEvidence {
                observed_at_ms: signal.saturating_sub(10_000),
                armed_at_ms: signal.saturating_sub(5_000),
                signal_at_ms: signal,
                decision_recorded_at_ms: decision,
                quote_at_ms: quote,
                now_ms: now,
            },
        },
        PortfolioState {
            nav_cents: nav,
            risk_reference_nav_cents: nav,
            available_cash_cents: nav.saturating_sub(total_exposure),
            total_exposure_cents: total_exposure,
            daily_realized_loss_cents: rng
                .range_u64(0, (nav.saturating_mul(5) / 100).saturating_sub(1)),
            open_positions,
            current_position_mint: position_mint,
            current_position_value_cents: current_position,
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
        3 => e.preflight_simulation_passed = false,
        4 => e.timeline.quote_at_ms = e.timeline.signal_at_ms,
        5 => e.timeline.quote_at_ms = e.timeline.decision_recorded_at_ms,
        6 => e.timeline.quote_at_ms = e.timeline.now_ms.saturating_add(1),
        7 => {
            e.timeline.now_ms = e
                .timeline
                .quote_at_ms
                .saturating_add(c.max_quote_age_ms)
                .saturating_add(1)
        }
        8 => e.timeline.armed_at_ms = e.timeline.signal_at_ms.saturating_add(1),
        9 => m.as_of_ms = e.timeline.decision_recorded_at_ms.saturating_add(1),
        10 => {
            m.as_of_ms = e
                .timeline
                .decision_recorded_at_ms
                .saturating_sub(c.max_market_evidence_age_ms.saturating_add(1))
        }
        11 => t.checked_at_ms = e.timeline.decision_recorded_at_ms.saturating_add(1),
        12 => {
            t.checked_at_ms = e
                .timeline
                .decision_recorded_at_ms
                .saturating_sub(c.max_token_evidence_age_ms.saturating_add(1))
        }
        13 => e.price_divergence_bps = c.max_price_divergence_bps.saturating_add(1),
        14 => m.independent_price_sources = c.min_independent_price_sources.saturating_sub(1),
        15 => b.token_evidence_mint.push('X'),
        16 => b.market_evidence_mint.push('X'),
        17 => b.execution_evidence_mint.push('X'),
        18 => {
            e.mode = if p.mode == ExecutionMode::Paper {
                ExecutionMode::Shadow
            } else {
                ExecutionMode::Paper
            }
        }
        19 => e.purpose = Purpose::Exit,
        20 => e.quoted_notional_cents = p.notional_cents.saturating_add(1),
        21 => t.exact_mint_verified = false,
        22 => t.mint_authority_present = true,
        23 => t.freeze_authority_present = true,
        24 => t.permanent_delegate_present = true,
        25 => t.non_transferable = true,
        26 => t.default_account_state_frozen = true,
        27 => {
            t.transfer_hook_present = true;
            t.transfer_hook_program_verified = true;
        }
        28 => t.pausable = true,
        29 => t.confidential_transfer_enabled = true,
        30 => t.scaled_ui_amount_enabled = true,
        31 => t.current_transfer_fee_bps = c.max_token_transfer_fee_bps.saturating_add(1),
        32 => e.reverse_sell_simulation_passed = false,
        33 => t.dex_first_verified = false,
        34 => t.entry_trigger_confirmed = false,
        35 => m.liquidity_cents = c.min_liquidity_cents.saturating_sub(1),
        36 => m.volume_24h_cents = c.min_volume_24h_cents.saturating_sub(1),
        37 => m.market_cap_cents = 0,
        38 => m.market_cap_cents = c.max_market_cap_cents.saturating_add(1),
        39 => {
            m.market_cap_cents = c.max_market_cap_cents.max(10_000);
            m.volume_24h_cents = m.market_cap_cents.saturating_mul(19) / 100;
        }
        40 => m.geckoterminal_score = c.min_geckoterminal_score.saturating_sub(1),
        41 => e.slippage_bps = c.max_slippage_bps.saturating_add(1),
        42 => e.price_impact_bps = c.max_price_impact_bps.saturating_add(1),
        43 => e.route_fee_bps = c.max_route_fee_bps.saturating_add(1),
        44 => {
            s.daily_realized_loss_cents =
                s.risk_reference_nav_cents.saturating_mul(5) / 100
        }
        45 => {
            s.current_position_mint = None;
            s.current_position_value_cents = 0;
            s.total_exposure_cents = 0;
            s.available_cash_cents = s.nav_cents;
            p.notional_cents = pct_for_sim(s.nav_cents, c.max_position_bps_of_nav).saturating_add(1);
            e.quoted_notional_cents = p.notional_cents;
        }
        46 => {
            let max_position = pct_for_sim(s.nav_cents, c.max_position_bps_of_nav);
            s.current_position_mint = Some(p.mint.clone());
            s.current_position_value_cents = max_position;
            s.total_exposure_cents = max_position;
            s.available_cash_cents = s.nav_cents.saturating_sub(max_position);
            s.open_positions = 1;
            p.notional_cents = 1;
            e.quoted_notional_cents = 1;
        }
        47 => {
            let max_total = pct_for_sim(s.nav_cents, c.max_total_exposure_bps_of_nav);
            s.current_position_mint = None;
            s.current_position_value_cents = 0;
            s.total_exposure_cents = max_total;
            s.available_cash_cents = s.nav_cents.saturating_sub(max_total);
            s.open_positions = 1;
            p.notional_cents = 1;
            e.quoted_notional_cents = 1;
        }
        48 => {
            s.current_position_mint = None;
            s.current_position_value_cents = 0;
            s.open_positions = c.max_open_positions;
        }
        49 => e.estimated_network_fee_cents = s.available_cash_cents.saturating_add(1),
        50 => {
            p.notional_cents = 0;
            e.quoted_notional_cents = 0;
        }
        51 => e.timeline.decision_recorded_at_ms = e.timeline.signal_at_ms.saturating_sub(1),
        52 => s.available_cash_cents = s.available_cash_cents.saturating_add(1),
        53 => {
            s.current_position_value_cents = 1;
            s.current_position_mint = Some("DifferentPositionMint".into());
            s.total_exposure_cents = s.total_exposure_cents.max(1);
            s.available_cash_cents = s.nav_cents.saturating_sub(s.total_exposure_cents);
        }
        54 => {
            s.current_position_mint = Some(p.mint.clone());
            s.current_position_value_cents = s.total_exposure_cents.saturating_add(1);
        }
        55 => s.risk_reference_nav_cents = 0,
        _ => {
            s.current_position_value_cents = 1;
            s.current_position_mint = None;
            s.total_exposure_cents = s.total_exposure_cents.max(1);
            s.available_cash_cents = s.nav_cents.saturating_sub(s.total_exposure_cents);
        }
    }
    (p, b, t, m, e, s)
}

fn valid_exit_fixture(rng: &mut Lcg, i: u64, emergency: bool) -> Fixture {
    let nav = 100_000;
    let total_exposure = rng.range_u64(1_000, 90_000);
    let position = rng.range_u64(1, total_exposure);
    let cash = nav - total_exposure;
    let signal = 10_000_000u64.saturating_add(i.saturating_mul(100));
    let decision = signal.saturating_add(rng.range_u64(0, 1_000));
    let quote = decision.saturating_add(rng.range_u64(1, 2_000));
    let amount = rng.range_u64(1, position);
    let purpose = if emergency {
        Purpose::EmergencyExit
    } else {
        Purpose::Exit
    };
    let proposal = ModelProposal {
        mint: format!("ExitMint{i:019}"),
        purpose,
        mode: if i.is_multiple_of(2) {
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
    let binding = EvidenceBinding::new(
        proposal.mint.as_str(),
        proposal.mint.as_str(),
        proposal.mint.as_str(),
    );
    let execution_mode = proposal.mode;
    let execution_purpose = proposal.purpose;
    let quoted_notional_cents = proposal.notional_cents;

    let (market_as_of, sources, divergence, slippage_cap, impact_cap, fee_cap) = if emergency {
        (0, 0, 9_999, 1_500, 1_200, 1_000)
    } else {
        (
            decision.saturating_sub(rng.range_u64(0, 60_000)),
            2,
            rng.range_u32(0, 500),
            300,
            200,
            500,
        )
    };

    (
        proposal,
        binding,
        TokenEvidence {
            checked_at_ms: decision,
            exact_mint_verified: false,
            token_program: TokenProgram::Token2022,
            mint_authority_present: true,
            freeze_authority_present: true,
            permanent_delegate_present: true,
            non_transferable: true,
            default_account_state_frozen: true,
            transfer_hook_present: true,
            transfer_hook_program_verified: false,
            pausable: true,
            confidential_transfer_enabled: true,
            scaled_ui_amount_enabled: true,
            current_transfer_fee_bps: 10_000,
            dex_first_verified: false,
            entry_trigger_confirmed: false,
        },
        MarketEvidence {
            as_of_ms: market_as_of,
            liquidity_cents: 0,
            volume_24h_cents: 0,
            market_cap_cents: 0,
            geckoterminal_score: 0,
            independent_price_sources: sources,
        },
        ExecutionEvidence {
            mode: execution_mode,
            purpose: execution_purpose,
            quoted_notional_cents,
            route_verified: true,
            preflight_simulation_passed: true,
            reverse_sell_simulation_passed: false,
            estimated_network_fee_cents: rng.range_u64(0, cash),
            slippage_bps: rng.range_u32(0, slippage_cap),
            price_impact_bps: rng.range_u32(0, impact_cap),
            route_fee_bps: rng.range_u32(0, fee_cap),
            price_divergence_bps: divergence,
            timeline: TimelineEvidence {
                observed_at_ms: signal.saturating_sub(5_000),
                armed_at_ms: signal.saturating_add(100),
                signal_at_ms: signal,
                decision_recorded_at_ms: decision,
                quote_at_ms: quote,
                now_ms: quote.saturating_add(rng.range_u64(0, 15_000)),
            },
        },
        PortfolioState {
            nav_cents: nav,
            risk_reference_nav_cents: nav,
            available_cash_cents: cash,
            total_exposure_cents: total_exposure,
            daily_realized_loss_cents: nav,
            open_positions: 5,
            current_position_mint: Some(format!("ExitMint{i:019}")),
            current_position_value_cents: position,
            entry_halt_active: true,
            global_kill_switch_active: false,
        },
    )
}

fn pct_for_sim(value: u64, bps: u32) -> u64 {
    ((value as u128 * bps as u128) / 10_000u128).min(u64::MAX as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adversarial_smoke_test_has_no_policy_misses() {
        let report = run_adversarial_simulation(20_000);
        assert!(report.passed(), "simulation failures: {report:?}");
        assert_eq!(report.entry_mutation_classes, 57);
        assert_eq!(report.total_checks(), 80_000);
        assert_eq!(report.emergency_route_failures_caught, 20_000);
    }
}
