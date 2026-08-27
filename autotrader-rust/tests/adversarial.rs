use autotrader_fortress::{
    ExecutionEvidence, ExecutionMode, MarketEvidence, ModelProposal, Phenotype, PolicyEngine,
    PortfolioState, Purpose, Rejection, RiskConfig, TimelineEvidence, TokenEvidence,
};

fn fixture(purpose: Purpose) -> (
    ModelProposal,
    TokenEvidence,
    MarketEvidence,
    ExecutionEvidence,
    PortfolioState,
) {
    (
        ModelProposal {
            mint: "BoundaryMint11111111111111111111111111111".into(),
            purpose,
            mode: ExecutionMode::Paper,
            notional_cents: 10_000,
            phenotype: Phenotype::DexFirstLaunch,
            reason: "boundary test".into(),
        },
        TokenEvidence {
            exact_mint_verified: true,
            security_gate_passed: true,
            dex_first_verified: true,
            entry_trigger_confirmed: true,
        },
        MarketEvidence {
            liquidity_cents: 1_000_000,
            volume_24h_cents: 6_000_000,
            market_cap_cents: 30_000_000,
            geckoterminal_score: 56,
            independent_price_sources: 2,
        },
        ExecutionEvidence {
            route_verified: true,
            slippage_bps: 300,
            price_impact_bps: 200,
            route_fee_bps: 500,
            price_divergence_bps: 500,
            timeline: TimelineEvidence {
                observed_at_ms: 1_000,
                armed_at_ms: 2_000,
                signal_at_ms: 3_000,
                quote_at_ms: 3_001,
                now_ms: 18_001,
            },
        },
        PortfolioState {
            nav_cents: 100_000,
            total_exposure_cents: 30_000,
            daily_realized_loss_cents: 4_999,
            open_positions: 4,
            current_position_value_cents: 20_000,
            entry_halt_active: false,
            global_kill_switch_active: false,
        },
    )
}

#[test]
fn exact_entry_boundaries_are_allowed() {
    let (p, t, m, e, s) = fixture(Purpose::Entry);
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.allowed, "boundary fixture rejected: {:?}", r.reasons);
}

#[test]
fn position_one_cent_over_cap_is_denied() {
    let (mut p, t, m, e, s) = fixture(Purpose::Entry);
    p.notional_cents = 10_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::PositionSizeTooLarge));
}

#[test]
fn projected_exposure_one_cent_over_cap_is_denied() {
    let (mut p, t, m, e, mut s) = fixture(Purpose::Entry);
    p.notional_cents = 10_000;
    s.total_exposure_cents = 30_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::TotalExposureTooHigh));
}

#[test]
fn daily_loss_at_limit_blocks_new_entry() {
    let (p, t, m, e, mut s) = fixture(Purpose::Entry);
    s.daily_realized_loss_cents = 5_000;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::DailyLossLimitReached));
}

#[test]
fn geckoterminal_55_is_denied_but_56_passes() {
    let (p, t, mut m, e, s) = fixture(Purpose::Entry);
    m.geckoterminal_score = 55;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::GeckoTerminalScoreTooLow));
}

#[test]
fn market_cap_one_cent_over_limit_is_denied() {
    let (p, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 30_000_001;
    m.volume_24h_cents = 6_000_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketCapTooHigh));
}

#[test]
fn volume_market_cap_exactly_twenty_percent_is_allowed() {
    let (p, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 10_000_000;
    m.volume_24h_cents = 2_000_000;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.allowed, "20 percent absorption rejected: {:?}", r.reasons);
}

#[test]
fn quote_age_boundary_is_strict() {
    let (p, t, m, mut e, s) = fixture(Purpose::Entry);
    e.timeline.now_ms = e.timeline.quote_at_ms + 15_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::QuoteStale));
}

#[test]
fn zero_market_cap_is_invalid_not_a_divide_by_zero() {
    let (p, t, mut m, e, s) = fixture(Purpose::Entry);
    m.market_cap_cents = 0;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::MarketCapInvalid));
}

#[test]
fn max_open_positions_blocks_entry_but_not_exit() {
    let (p, t, m, e, mut s) = fixture(Purpose::Entry);
    s.open_positions = 5;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::OpenPositionLimitReached));

    let (mut p, mut t, mut m, e, mut s) = fixture(Purpose::Exit);
    p.notional_cents = 5_000;
    s.open_positions = 5;
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
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.allowed, "exit trapped by entry risk gates: {:?}", r.reasons);
}

#[test]
fn exit_cannot_claim_more_than_current_position() {
    let (mut p, t, m, e, mut s) = fixture(Purpose::Exit);
    s.current_position_value_cents = 5_000;
    p.notional_cents = 5_001;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ExitAmountExceedsPosition));
}

#[test]
fn global_kill_switch_blocks_every_purpose() {
    for purpose in [Purpose::Entry, Purpose::Exit, Purpose::EmergencyExit] {
        let (mut p, t, m, e, mut s) = fixture(purpose);
        if purpose != Purpose::Entry {
            p.notional_cents = 5_000;
        }
        s.global_kill_switch_active = true;
        let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
        assert!(r.reasons.contains(&Rejection::GlobalKillSwitch));
    }
}

#[test]
fn emergency_exit_accepts_distressed_data_but_not_bad_route() {
    let (mut p, mut t, mut m, mut e, mut s) = fixture(Purpose::EmergencyExit);
    p.notional_cents = 5_000;
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
    let r = engine.evaluate(&p, &t, &m, &e, &s);
    assert!(r.allowed, "distressed emergency exit rejected: {:?}", r.reasons);

    e.route_verified = false;
    let r = engine.evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::RouteUnverified));
}

#[test]
fn u64_exposure_overflow_is_detected() {
    let (mut p, t, m, e, mut s) = fixture(Purpose::Entry);
    s.nav_cents = u64::MAX;
    s.total_exposure_cents = u64::MAX - 5;
    p.notional_cents = 10;
    let r = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &t, &m, &e, &s);
    assert!(r.reasons.contains(&Rejection::ArithmeticOverflow));
}
