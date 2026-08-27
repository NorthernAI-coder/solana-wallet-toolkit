use autotrader_fortress::{
    EvidenceBinding, ExecutionEvidence, ExecutionMode, MarketEvidence, ModelProposal, Phenotype,
    PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig, TimelineEvidence, TokenEvidence,
    TokenProgram,
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
        token_program: TokenProgram::Token2022,
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
        current_transfer_fee_bps: 500,
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
        preflight_simulation_passed: true,
        reverse_sell_simulation_passed: true,
        estimated_network_fee_cents: 100,
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
    let (position_mint, position_value, open_positions) = if purpose == Purpose::Entry {
        (None, 0, 4)
    } else {
        (Some(proposal.mint.clone()), 20_000, 5)
    };
    let portfolio = PortfolioState {
        nav_cents: 100_000,
        risk_reference_nav_cents: 100_000,
        available_cash_cents: 70_000,
        total_exposure_cents: 30_000,
        daily_realized_loss_cents: if purpose == Purpose::Entry {
            4_999
        } else {
            100_000
        },
        open_positions,
        current_position_mint: position_mint,
        current_position_value_cents: position_value,
        entry_halt_active: purpose != Purpose::Entry,
        global_kill_switch_active: false,
    };
    (proposal, binding, token, market, execution, portfolio)
}

fn set_notional(proposal: &mut ModelProposal, execution: &mut ExecutionEvidence, cents: u64) {
    proposal.notional_cents = cents;
    execution.quoted_notional_cents = cents;
}

fn evaluate(f: &Fixture) -> autotrader_fortress::DecisionReport {
    PolicyEngine::new(RiskConfig::default()).evaluate(&f.0, &f.1, &f.2, &f.3, &f.4, &f.5)
}

#[test]
fn exact_entry_boundaries_are_allowed() {
    let f = fixture(Purpose::Entry);
    let r = evaluate(&f);
    assert!(r.allowed, "boundary fixture rejected: {:?}", r.reasons);
}

#[test]
fn evidence_for_another_mint_is_denied() {
    let mut f = fixture(Purpose::Entry);
    f.1.execution_evidence_mint = "OtherMint1111111111111111111111111111111".into();
    let r = evaluate(&f);
    assert!(r.reasons.contains(&Rejection::EvidenceMintMismatch));
}

#[test]
fn execution_mode_direction_and_amount_are_bound_to_proposal() {
    let mut f = fixture(Purpose::Entry);
    f.4.mode = ExecutionMode::Shadow;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::ExecutionModeMismatch));

    f.4.mode = f.0.mode;
    f.4.purpose = Purpose::Exit;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::ExecutionPurposeMismatch));

    f.4.purpose = f.0.purpose;
    f.4.quoted_notional_cents = f.0.notional_cents + 1;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::QuoteNotionalMismatch));
}

#[test]
fn cumulative_same_token_position_cap_cannot_be_evaded_by_scaling() {
    let mut f = fixture(Purpose::Entry);
    f.5.current_position_mint = Some(f.0.mint.clone());
    f.5.current_position_value_cents = 9_000;
    f.5.open_positions = 5;
    set_notional(&mut f.0, &mut f.4, 1_000);
    assert!(evaluate(&f).allowed, "exact cumulative cap should pass");

    f.5.current_position_value_cents = 9_001;
    let r = evaluate(&f);
    assert!(r.reasons.contains(&Rejection::PositionSizeTooLarge));
}

#[test]
fn max_open_positions_blocks_only_a_new_position() {
    let mut f = fixture(Purpose::Entry);
    f.5.open_positions = 5;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::OpenPositionLimitReached));

    f.5.current_position_mint = Some(f.0.mint.clone());
    f.5.current_position_value_cents = 1;
    set_notional(&mut f.0, &mut f.4, 9_999);
    let r = evaluate(&f);
    assert!(
        !r.reasons.contains(&Rejection::OpenPositionLimitReached),
        "existing-position scale was treated as a new position"
    );
}

#[test]
fn portfolio_identity_and_accounting_are_fail_closed() {
    let mut f = fixture(Purpose::Entry);
    f.5.available_cash_cents += 1;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::PortfolioStateInvalid));

    let mut f = fixture(Purpose::Entry);
    f.5.current_position_mint = Some("DifferentMint".into());
    f.5.current_position_value_cents = 1;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::PositionMintMismatch));
}

#[test]
fn invalid_risk_configuration_fails_closed() {
    let f = fixture(Purpose::Entry);
    let mut config = RiskConfig::default();
    config.max_position_bps_of_nav = 0;
    let r = PolicyEngine::new(config).evaluate(&f.0, &f.1, &f.2, &f.3, &f.4, &f.5);
    assert!(r.reasons.contains(&Rejection::RiskConfigInvalid));
}

#[test]
fn daily_loss_uses_fixed_reference_nav_not_moving_current_nav() {
    let mut f = fixture(Purpose::Entry);
    f.5.nav_cents = 80_000;
    f.5.risk_reference_nav_cents = 100_000;
    f.5.total_exposure_cents = 20_000;
    f.5.available_cash_cents = 60_000;
    f.5.daily_realized_loss_cents = 5_000;
    set_notional(&mut f.0, &mut f.4, 1_000);
    let r = evaluate(&f);
    assert!(r.reasons.contains(&Rejection::DailyLossLimitReached));
}

#[test]
fn explicit_solana_authority_and_extension_hazards_are_denied() {
    let base = fixture(Purpose::Entry);
    let mut cases = Vec::new();

    let mut t = base.2;
    t.mint_authority_present = true;
    cases.push((t, Rejection::MintAuthorityPresent));
    let mut t = base.2;
    t.freeze_authority_present = true;
    cases.push((t, Rejection::FreezeAuthorityPresent));
    let mut t = base.2;
    t.permanent_delegate_present = true;
    cases.push((t, Rejection::PermanentDelegatePresent));
    let mut t = base.2;
    t.non_transferable = true;
    cases.push((t, Rejection::NonTransferableToken));
    let mut t = base.2;
    t.default_account_state_frozen = true;
    cases.push((t, Rejection::DefaultAccountStateFrozen));
    let mut t = base.2;
    t.transfer_hook_present = true;
    t.transfer_hook_program_verified = true;
    cases.push((t, Rejection::TransferHookNotAllowed));
    let mut t = base.2;
    t.pausable = true;
    cases.push((t, Rejection::PausableToken));
    let mut t = base.2;
    t.confidential_transfer_enabled = true;
    cases.push((t, Rejection::ConfidentialTransferEnabled));
    let mut t = base.2;
    t.scaled_ui_amount_enabled = true;
    cases.push((t, Rejection::ScaledUiAmountEnabled));
    let mut t = base.2;
    t.current_transfer_fee_bps = 501;
    cases.push((t, Rejection::TokenTransferFeeTooHigh));

    for (token, expected) in cases {
        let r = PolicyEngine::new(RiskConfig::default())
            .evaluate(&base.0, &base.1, &token, &base.3, &base.4, &base.5);
        assert!(r.reasons.contains(&expected), "missing {expected:?}");
    }
}

#[test]
fn allowed_transfer_hook_still_requires_verified_program() {
    let mut f = fixture(Purpose::Entry);
    f.2.transfer_hook_present = true;
    f.2.transfer_hook_program_verified = false;
    let mut config = RiskConfig::default();
    config.allow_transfer_hook = true;
    let r = PolicyEngine::new(config).evaluate(&f.0, &f.1, &f.2, &f.3, &f.4, &f.5);
    assert!(r.reasons.contains(&Rejection::TransferHookUnverified));
}

#[test]
fn exact_buy_and_reverse_sell_preflights_are_mandatory_for_entry() {
    let mut f = fixture(Purpose::Entry);
    f.4.preflight_simulation_passed = false;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::PreflightSimulationFailed));

    let mut f = fixture(Purpose::Entry);
    f.4.reverse_sell_simulation_passed = false;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::ReverseSellSimulationFailed));
}

#[test]
fn network_fee_and_total_cash_requirement_are_enforced() {
    let mut f = fixture(Purpose::Entry);
    f.4.estimated_network_fee_cents = f.5.available_cash_cents + 1;
    let r = evaluate(&f);
    assert!(r.reasons.contains(&Rejection::NetworkFeeUnaffordable));
    assert!(r.reasons.contains(&Rejection::InsufficientCash));
}

#[test]
fn exact_thresholds_and_market_quality_gates_hold() {
    let mut f = fixture(Purpose::Entry);
    f.3.geckoterminal_score = 55;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::GeckoTerminalScoreTooLow));

    let mut f = fixture(Purpose::Entry);
    f.3.market_cap_cents = 30_000_001;
    f.3.volume_24h_cents = 6_000_001;
    assert!(evaluate(&f).reasons.contains(&Rejection::MarketCapTooHigh));

    let mut f = fixture(Purpose::Entry);
    f.3.market_cap_cents = 10_000_000;
    f.3.volume_24h_cents = 2_000_000;
    assert!(evaluate(&f).allowed, "exact 20% absorption should pass");
}

#[test]
fn post_decision_quote_and_evidence_freshness_are_strict() {
    let mut f = fixture(Purpose::Entry);
    f.4.timeline.quote_at_ms = f.4.timeline.decision_recorded_at_ms;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::QuoteNotAfterDecision));

    let mut f = fixture(Purpose::Entry);
    f.4.timeline.now_ms = f.4.timeline.quote_at_ms + 15_001;
    assert!(evaluate(&f).reasons.contains(&Rejection::QuoteStale));

    let mut f = fixture(Purpose::Entry);
    f.3.as_of_ms = f.4.timeline.decision_recorded_at_ms + 1;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::MarketEvidenceFromFuture));

    let mut f = fixture(Purpose::Entry);
    f.2.checked_at_ms = f.4.timeline.decision_recorded_at_ms - 300_001;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::TokenEvidenceStale));
}

#[test]
fn zero_market_cap_is_invalid_not_a_divide_by_zero() {
    let mut f = fixture(Purpose::Entry);
    f.3.market_cap_cents = 0;
    assert!(evaluate(&f).reasons.contains(&Rejection::MarketCapInvalid));
}

#[test]
fn exits_require_the_exact_held_position() {
    let mut f = fixture(Purpose::Exit);
    f.5.current_position_mint = Some("WrongMint".into());
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::PositionMintMismatch));

    let mut f = fixture(Purpose::Exit);
    f.5.current_position_mint = None;
    f.5.current_position_value_cents = 0;
    assert!(evaluate(&f).reasons.contains(&Rejection::PositionMissing));
}

#[test]
fn exit_cannot_claim_more_than_current_position() {
    let mut f = fixture(Purpose::Exit);
    f.5.current_position_value_cents = 5_000;
    set_notional(&mut f.0, &mut f.4, 5_001);
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::ExitAmountExceedsPosition));
}

#[test]
fn entry_only_gates_never_trap_normal_exit() {
    let mut f = fixture(Purpose::Exit);
    set_notional(&mut f.0, &mut f.4, 5_000);
    f.5.entry_halt_active = true;
    f.5.daily_realized_loss_cents = 100_000;
    f.2.exact_mint_verified = false;
    f.2.mint_authority_present = true;
    f.2.freeze_authority_present = true;
    f.2.permanent_delegate_present = true;
    f.2.non_transferable = true;
    f.2.default_account_state_frozen = true;
    f.2.transfer_hook_present = true;
    f.2.pausable = true;
    f.2.confidential_transfer_enabled = true;
    f.2.scaled_ui_amount_enabled = true;
    f.2.current_transfer_fee_bps = 10_000;
    f.2.dex_first_verified = false;
    f.2.entry_trigger_confirmed = false;
    f.3.liquidity_cents = 0;
    f.3.volume_24h_cents = 0;
    f.3.market_cap_cents = 0;
    f.3.geckoterminal_score = 0;
    f.4.reverse_sell_simulation_passed = false;
    let r = evaluate(&f);
    assert!(
        r.allowed,
        "exit trapped by entry-only gates: {:?}",
        r.reasons
    );
}

#[test]
fn emergency_exit_survives_secondary_data_outage_but_not_bad_execution() {
    let mut f = fixture(Purpose::EmergencyExit);
    set_notional(&mut f.0, &mut f.4, 5_000);
    f.3.as_of_ms = 0;
    f.3.independent_price_sources = 0;
    f.4.price_divergence_bps = 10_000;
    f.4.slippage_bps = 1_500;
    f.4.price_impact_bps = 1_200;
    f.4.route_fee_bps = 1_000;
    let r = evaluate(&f);
    assert!(r.allowed, "emergency exit was trapped: {:?}", r.reasons);

    f.4.route_verified = false;
    assert!(evaluate(&f).reasons.contains(&Rejection::RouteUnverified));

    f.4.route_verified = true;
    f.4.preflight_simulation_passed = false;
    assert!(evaluate(&f)
        .reasons
        .contains(&Rejection::PreflightSimulationFailed));
}

#[test]
fn global_kill_switch_blocks_every_purpose() {
    for purpose in [Purpose::Entry, Purpose::Exit, Purpose::EmergencyExit] {
        let mut f = fixture(purpose);
        if purpose != Purpose::Entry {
            set_notional(&mut f.0, &mut f.4, 5_000);
        }
        f.5.global_kill_switch_active = true;
        assert!(evaluate(&f).reasons.contains(&Rejection::GlobalKillSwitch));
    }
}

#[test]
fn checked_cash_arithmetic_detects_overflow() {
    let mut f = fixture(Purpose::Entry);
    f.4.estimated_network_fee_cents = u64::MAX;
    let r = evaluate(&f);
    assert!(r.reasons.contains(&Rejection::ArithmeticOverflow));
}
