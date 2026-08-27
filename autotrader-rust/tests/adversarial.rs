use autotrader_fortress::{
    EvidenceBinding, ExecutionEvidence, ExecutionMode, MarketEvidence, ModelProposal, Phenotype,
    PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig, TimelineEvidence, TokenEvidence,
};

type Fixture = (
    ModelProposal,
    EvidenceBinding,
    TokenEvidence,
    MarketEvidence,
    ExecutionEvidence,
    PortfolioState,
);

fn fixture(purpose: Purpose) -> Fixture {
    let proposal = ModelProposal {
        mint: "BoundaryMint11111111111111111111111111111".into(),
        purpose,
        mode: ExecutionMode::Paper,
        notional_cents: 10_000,
        phenotype: Phenotype::DexFirstLaunch,
        reason: "boundary test".into(),
    };
    let binding = EvidenceBinding::new(
        proposal.mint.as_str(),
        proposal.mint.as_str(),
        proposal.mint.as_str(),
    );
    let decision = 995_000;
    let token = TokenEvidence {
        checked_at_ms: 990_000,
        exact_mint_verified: true,
        security_gate_passed: true,
        dex_first_verified: true,
        entry_trigger_confirmed: true,
    };
    let market = MarketEvidence {
        as_of_ms: 994_000,
        liquidity_cents: 1_000_000,
        volume_24h_cents: 6_000_000,
        market_cap_cents: 30_000_000,
        geckoterminal_score: 56,
        independent_price_sources: 2,
    };
    let execution = ExecutionEvidence {
        mode: proposal.mode,
        purpose: proposal.purpose,
        quoted_notional_cents: proposal.notional_cents,
        route_verified: true,
        slippage_bps: 300,
        price_impact_bps: 200,
        route_fee_bps: 500,
        price_divergence_bps: 500,
        timeline: TimelineEvidence {
            observed_at_ms: 900_000,
            armed_at_ms: 950_000,
            signal_at_ms: 990_000,
            decision_recorded_at_ms: decision,
            quote_at_ms: 996_000,
            now_ms: 1_011_000,
        },
    };
    let portfolio = PortfolioState {
        nav_cents: 100_000,
        available_cash_cents: 70_000,
        total_exposure_cents: 30_000,
        daily_realized_loss_cents: 4_999,
        open_positions: 4,
        current_position_value_cents: 20_000,
        entry_halt_active: false,
        global_kill_switch_active: false,
    };
    (proposal, binding, token, market, execution, portfolio)
}

fn set_notional(proposal: &mut ModelProposal, execution: &mut ExecutionEvidence, cents: u64) {
    proposal.notional_cents = cents;
    execution.quoted_notional_cents = cents;
}

#[test]
fn exact_entry_boundaries_are_allowed() {
    let (p, b, t, m, e, s) = fixture(Purpose::Entry);
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.allowed, "boundary fixture rejected: {:?}", r.reasons);
}

#[test]
fn evidence_for_another_mint_is_denied() {
    let (p, mut b, t, m, e, s) = fixture(Purpose::Entry);
    b.execution_evidence_mint = "OtherMint1111111111111111111111111111111".into();
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::EvidenceMintMismatch));
}

#[test]
fn execution_mode_direction_and_amount_are_bound_to_proposal() {
    let (p, b, t, m, mut e, s) = fixture(Purpose::Entry);
    let engine = PolicyEngine::new(RiskConfig::default());

    e.mode = ExecutionMode::Shadow;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ExecutionModeMismatch));

    e.mode = p.mode;
    e.purpose = Purpose::Exit;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ExecutionPurposeMismatch));

    e.purpose = p.purpose;
    e.quoted_notional_cents = p.notional_cents + 1;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::QuoteNotionalMismatch));
}

#[test]
fn position_one_cent_over_cap_is_denied() {
    let (mut p, b, t, m, mut e, s) = fixture(Purpose::Entry);
    set_notional(&mut p, &mut e, 10_001);
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::PositionSizeTooLarge));
}

#[test]
fn projected_exposure_one_cent_over_cap_is_denied() {
    let (p, b, t, m, e, mut s) = fixture(Purpose::Entry);
    s.total_exposure_cents = 30_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::TotalExposureTooHigh));
}

#[test]
fn insufficient_cash_is_denied_even_if_exposure_cap_allows_trade() {
    let (p, b, t, m, e, mut s) = fixture(Purpose::Entry);
    s.available_cash_cents = p.notional_cents - 1;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::InsufficientCash));
}

#[test]
fn daily_loss_at_limit_blocks_new_entry() {
    let (p, b, t, m, e, mut s) = fixture(Purpose::Entry);
    s.daily_realized_loss_cents = 5_000;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::DailyLossLimitReached));
}

#[test]
fn geckoterminal_55_is_denied_but_56_passes() {
    let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
    m.geckoterminal_score = 55;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::GeckoTerminalScoreTooLow));
}

#[test]
fn market_cap_one_cent_over_limit_is_denied() {
    let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 30_000_001;
    m.volume_24h_cents = 6_000_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketCapTooHigh));
}

#[test]
fn volume_market_cap_exactly_twenty_percent_is_allowed() {
    let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 10_000_000;
    m.volume_24h_cents = 2_000_000;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.allowed, "20 percent absorption rejected: {:?}", r.reasons);
}

#[test]
fn post_decision_quote_and_quote_age_are_strict() {
    let (p, b, t, m, mut e, s) = fixture(Purpose::Entry);
    let engine = PolicyEngine::new(RiskConfig::default());

    e.timeline.quote_at_ms = e.timeline.decision_recorded_at_ms;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::QuoteNotAfterDecision));

    e.timeline.quote_at_ms = 996_000;
    e.timeline.now_ms = 1_011_001;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::QuoteStale));
}

#[test]
fn future_and_stale_market_evidence_are_denied() {
    let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
    let engine = PolicyEngine::new(RiskConfig::default());

    m.as_of_ms = e.timeline.decision_recorded_at_ms + 1;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketEvidenceFromFuture));

    m.as_of_ms = e.timeline.decision_recorded_at_ms - 60_001;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketEvidenceStale));
}

#[test]
fn future_and_stale_token_evidence_are_denied_for_entries() {
    let (p, b, mut t, m, e, s) = fixture(Purpose::Entry);
    let engine = PolicyEngine::new(RiskConfig::default());

    t.checked_at_ms = e.timeline.decision_recorded_at_ms + 1;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::TokenEvidenceFromFuture));

    t.checked_at_ms = e.timeline.decision_recorded_at_ms - 300_001;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::TokenEvidenceStale));
}

#[test]
fn zero_market_cap_is_invalid_not_a_divide_by_zero() {
    let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 0;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketCapInvalid));
}

#[test]
fn max_open_positions_blocks_entry_but_not_exit() {
    let (p, b, t, m, e, mut s) = fixture(Purpose::Entry);
    s.open_positions = 5;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::OpenPositionLimitReached));

    let (mut p, b, mut t, mut m, mut e, mut s) = fixture(Purpose::Exit);
    set_notional(&mut p, &mut e, 5_000);
    s.open_positions = 5;
    s.available_cash_cents = 0;
    s.entry_halt_active = true;
    s.daily_realized_loss_cents = 100_000;
    t.exact_mint_verified = false;
    t.security_gate_passed = false;
    t.dex_first_verified = false;
    t.entry_trigger_confirmed = false;
    m.liquidity_cents = 0;
    m.volume_24h_cents = 0;
    m.market_cap_cents = 0;
    m.geckoterminal_score = 0;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(
        r.allowed,
        "exit trapped by entry risk gates: {:?}",
        r.reasons
    );
}

#[test]
fn exit_cannot_claim_more_than_current_position() {
    let (mut p, b, t, m, mut e, mut s) = fixture(Purpose::Exit);
    s.current_position_value_cents = 5_000;
    set_notional(&mut p, &mut e, 5_001);
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ExitAmountExceedsPosition));
}

#[test]
fn global_kill_switch_blocks_every_purpose() {
    for purpose in [Purpose::Entry, Purpose::Exit, Purpose::EmergencyExit] {
        let (mut p, b, t, m, mut e, mut s) = fixture(purpose);
        if purpose != Purpose::Entry {
            set_notional(&mut p, &mut e, 5_000);
        }
        s.global_kill_switch_active = true;
        let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(r.reasons.contains(&Rejection::GlobalKillSwitch));
    }
}

#[test]
fn emergency_exit_accepts_distressed_data_but_not_bad_route() {
    let (mut p, b, mut t, mut m, mut e, mut s) = fixture(Purpose::EmergencyExit);
    set_notional(&mut p, &mut e, 5_000);
    s.available_cash_cents = 0;
    s.entry_halt_active = true;
    s.daily_realized_loss_cents = 100_000;
    t.exact_mint_verified = false;
    t.security_gate_passed = false;
    t.dex_first_verified = false;
    t.entry_trigger_confirmed = false;
    m.liquidity_cents = 0;
    m.volume_24h_cents = 0;
    m.market_cap_cents = 0;
    m.geckoterminal_score = 0;
    m.independent_price_sources = 1;
    e.slippage_bps = 1_500;
    e.price_impact_bps = 1_200;
    e.route_fee_bps = 1_000;
    e.price_divergence_bps = 2_000;

    let engine = PolicyEngine::new(RiskConfig::default());
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(
        r.allowed,
        "distressed emergency exit rejected: {:?}",
        r.reasons
    );

    e.route_verified = false;
    let r = engine.evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::RouteUnverified));
}

#[test]
fn u64_exposure_overflow_is_detected() {
    let (mut p, b, t, m, mut e, mut s) = fixture(Purpose::Entry);
    s.nav_cents = u64::MAX;
    s.available_cash_cents = u64::MAX;
    s.total_exposure_cents = u64::MAX - 5;
    set_notional(&mut p, &mut e, 10);
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ArithmeticOverflow));
}
