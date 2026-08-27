#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    Entry,
    Exit,
    EmergencyExit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Paper,
    Shadow,
    Devnet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phenotype {
    Builder,
    Provenance,
    ControllerBehavior,
    GithubPreToken,
    PhoenixMigration,
    DexFirstLaunch,
    CreatorApprovedMeme,
    StructuralDemand,
    ResidualValue,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelProposal {
    pub mint: String,
    pub purpose: Purpose,
    pub mode: ExecutionMode,
    pub notional_cents: u64,
    pub phenotype: Phenotype,
    pub reason: String,
}

/// Exact asset identities reported independently by token, market, and execution adapters.
/// Do not construct these values by copying `ModelProposal::mint` without source verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceBinding {
    pub token_evidence_mint: String,
    pub market_evidence_mint: String,
    pub execution_evidence_mint: String,
}

impl EvidenceBinding {
    pub fn new(
        token_evidence_mint: impl Into<String>,
        market_evidence_mint: impl Into<String>,
        execution_evidence_mint: impl Into<String>,
    ) -> Self {
        Self {
            token_evidence_mint: token_evidence_mint.into(),
            market_evidence_mint: market_evidence_mint.into(),
            execution_evidence_mint: execution_evidence_mint.into(),
        }
    }

    fn matches(&self, proposal_mint: &str) -> bool {
        !proposal_mint.is_empty()
            && self.token_evidence_mint == proposal_mint
            && self.market_evidence_mint == proposal_mint
            && self.execution_evidence_mint == proposal_mint
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimelineEvidence {
    pub observed_at_ms: u64,
    pub armed_at_ms: u64,
    pub signal_at_ms: u64,
    /// Trusted orchestrator timestamp taken only after the model proposal is fully recorded.
    pub decision_recorded_at_ms: u64,
    pub quote_at_ms: u64,
    pub now_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MarketEvidence {
    pub as_of_ms: u64,
    pub liquidity_cents: u64,
    pub volume_24h_cents: u64,
    pub market_cap_cents: u64,
    pub geckoterminal_score: u8,
    pub independent_price_sources: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenEvidence {
    pub checked_at_ms: u64,
    pub exact_mint_verified: bool,
    pub security_gate_passed: bool,
    pub dex_first_verified: bool,
    pub entry_trigger_confirmed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionEvidence {
    pub mode: ExecutionMode,
    pub purpose: Purpose,
    pub quoted_notional_cents: u64,
    pub route_verified: bool,
    pub slippage_bps: u32,
    pub price_impact_bps: u32,
    pub route_fee_bps: u32,
    pub price_divergence_bps: u32,
    pub timeline: TimelineEvidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortfolioState {
    pub nav_cents: u64,
    pub available_cash_cents: u64,
    pub total_exposure_cents: u64,
    pub daily_realized_loss_cents: u64,
    pub open_positions: u16,
    pub current_position_value_cents: u64,
    /// Blocks new entries while preserving risk-reducing exits.
    pub entry_halt_active: bool,
    /// Freezes every simulated/shadow action for suspected compromise or manual pause.
    pub global_kill_switch_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RiskConfig {
    pub max_position_bps_of_nav: u32,
    pub max_total_exposure_bps_of_nav: u32,
    pub max_daily_loss_bps_of_nav: u32,
    pub max_slippage_bps: u32,
    pub max_price_impact_bps: u32,
    pub emergency_exit_max_slippage_bps: u32,
    pub emergency_exit_max_price_impact_bps: u32,
    pub max_route_fee_bps: u32,
    pub emergency_exit_max_route_fee_bps: u32,
    pub max_price_divergence_bps: u32,
    pub emergency_exit_max_price_divergence_bps: u32,
    pub min_liquidity_cents: u64,
    pub min_volume_24h_cents: u64,
    pub max_market_cap_cents: u64,
    pub min_volume_to_market_cap_bps: u32,
    pub min_geckoterminal_score: u8,
    pub min_independent_price_sources: u8,
    pub emergency_exit_min_independent_price_sources: u8,
    pub max_quote_age_ms: u64,
    pub max_market_evidence_age_ms: u64,
    pub emergency_exit_max_market_evidence_age_ms: u64,
    pub max_token_evidence_age_ms: u64,
    pub max_open_positions: u16,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_position_bps_of_nav: 1_000,
            max_total_exposure_bps_of_nav: 4_000,
            max_daily_loss_bps_of_nav: 500,
            max_slippage_bps: 300,
            max_price_impact_bps: 200,
            emergency_exit_max_slippage_bps: 1_500,
            emergency_exit_max_price_impact_bps: 1_200,
            max_route_fee_bps: 500,
            emergency_exit_max_route_fee_bps: 1_000,
            max_price_divergence_bps: 500,
            emergency_exit_max_price_divergence_bps: 2_000,
            min_liquidity_cents: 1_000_000,
            min_volume_24h_cents: 500_000,
            max_market_cap_cents: 30_000_000,
            min_volume_to_market_cap_bps: 2_000,
            min_geckoterminal_score: 56,
            min_independent_price_sources: 2,
            emergency_exit_min_independent_price_sources: 1,
            max_quote_age_ms: 15_000,
            max_market_evidence_age_ms: 60_000,
            emergency_exit_max_market_evidence_age_ms: 300_000,
            max_token_evidence_age_ms: 300_000,
            max_open_positions: 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    EvidenceMintMismatch,
    ExecutionModeMismatch,
    ExecutionPurposeMismatch,
    QuoteNotionalMismatch,
    GlobalKillSwitch,
    EntryHalt,
    RouteUnverified,
    QuoteNotAfterSignal,
    QuoteNotAfterDecision,
    QuoteFromFuture,
    QuoteStale,
    InvalidSignalOrder,
    MarketEvidenceFromFuture,
    MarketEvidenceStale,
    TokenEvidenceFromFuture,
    TokenEvidenceStale,
    PriceSourceDivergence,
    TooFewIndependentPriceSources,
    ExactMintUnverified,
    SecurityGateFailed,
    DexFirstUnverified,
    EntryTriggerMissing,
    LiquidityTooLow,
    VolumeTooLow,
    MarketCapInvalid,
    MarketCapTooHigh,
    VolumeMarketCapAbsorptionTooLow,
    GeckoTerminalScoreTooLow,
    SlippageTooHigh,
    PriceImpactTooHigh,
    RouteFeeTooHigh,
    DailyLossLimitReached,
    PositionSizeInvalid,
    PositionSizeTooLarge,
    InsufficientCash,
    TotalExposureTooHigh,
    OpenPositionLimitReached,
    ExitAmountInvalid,
    ExitAmountExceedsPosition,
    ArithmeticOverflow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionReport {
    pub allowed: bool,
    pub reasons: Vec<Rejection>,
}

impl DecisionReport {
    fn from_reasons(reasons: Vec<Rejection>) -> Self {
        Self {
            allowed: reasons.is_empty(),
            reasons,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PolicyEngine {
    config: RiskConfig,
}

impl PolicyEngine {
    pub const fn new(config: RiskConfig) -> Self {
        Self { config }
    }

    pub const fn config(&self) -> &RiskConfig {
        &self.config
    }

    pub fn evaluate(
        &self,
        proposal: &ModelProposal,
        binding: &EvidenceBinding,
        token: &TokenEvidence,
        market: &MarketEvidence,
        execution: &ExecutionEvidence,
        portfolio: &PortfolioState,
    ) -> DecisionReport {
        let mut reasons = Vec::new();

        if !binding.matches(&proposal.mint) {
            reasons.push(Rejection::EvidenceMintMismatch);
        }
        if execution.mode != proposal.mode {
            reasons.push(Rejection::ExecutionModeMismatch);
        }
        if execution.purpose != proposal.purpose {
            reasons.push(Rejection::ExecutionPurposeMismatch);
        }
        if execution.quoted_notional_cents != proposal.notional_cents {
            reasons.push(Rejection::QuoteNotionalMismatch);
        }
        if portfolio.global_kill_switch_active {
            reasons.push(Rejection::GlobalKillSwitch);
        }

        self.evaluate_execution_common(proposal, market, execution, &mut reasons);

        match proposal.purpose {
            Purpose::Entry => {
                if portfolio.entry_halt_active {
                    reasons.push(Rejection::EntryHalt);
                }
                self.evaluate_entry(proposal, token, market, execution, portfolio, &mut reasons);
            }
            Purpose::Exit => {
                self.evaluate_exit(proposal, execution, portfolio, false, &mut reasons);
            }
            Purpose::EmergencyExit => {
                self.evaluate_exit(proposal, execution, portfolio, true, &mut reasons);
            }
        }

        DecisionReport::from_reasons(reasons)
    }

    fn evaluate_execution_common(
        &self,
        proposal: &ModelProposal,
        market: &MarketEvidence,
        execution: &ExecutionEvidence,
        reasons: &mut Vec<Rejection>,
    ) {
        let t = execution.timeline;
        let emergency = proposal.purpose == Purpose::EmergencyExit;

        if !execution.route_verified {
            reasons.push(Rejection::RouteUnverified);
        }
        if t.quote_at_ms <= t.signal_at_ms {
            reasons.push(Rejection::QuoteNotAfterSignal);
        }
        if t.quote_at_ms <= t.decision_recorded_at_ms {
            reasons.push(Rejection::QuoteNotAfterDecision);
        }
        if t.quote_at_ms > t.now_ms {
            reasons.push(Rejection::QuoteFromFuture);
        } else if t.now_ms.saturating_sub(t.quote_at_ms) > self.config.max_quote_age_ms {
            reasons.push(Rejection::QuoteStale);
        }

        let valid_order = match proposal.purpose {
            Purpose::Entry => {
                t.observed_at_ms <= t.armed_at_ms
                    && t.armed_at_ms <= t.signal_at_ms
                    && t.signal_at_ms <= t.decision_recorded_at_ms
                    && t.decision_recorded_at_ms < t.quote_at_ms
            }
            Purpose::Exit | Purpose::EmergencyExit => {
                t.observed_at_ms <= t.signal_at_ms
                    && t.signal_at_ms <= t.decision_recorded_at_ms
                    && t.decision_recorded_at_ms < t.quote_at_ms
            }
        };
        if !valid_order {
            reasons.push(Rejection::InvalidSignalOrder);
        }

        if market.as_of_ms > t.decision_recorded_at_ms {
            reasons.push(Rejection::MarketEvidenceFromFuture);
        } else {
            let age = t.decision_recorded_at_ms.saturating_sub(market.as_of_ms);
            let cap = if emergency {
                self.config.emergency_exit_max_market_evidence_age_ms
            } else {
                self.config.max_market_evidence_age_ms
            };
            if age > cap {
                reasons.push(Rejection::MarketEvidenceStale);
            }
        }

        let divergence_cap = if emergency {
            self.config.emergency_exit_max_price_divergence_bps
        } else {
            self.config.max_price_divergence_bps
        };
        if execution.price_divergence_bps > divergence_cap {
            reasons.push(Rejection::PriceSourceDivergence);
        }

        let required_sources = if emergency {
            self.config.emergency_exit_min_independent_price_sources
        } else {
            self.config.min_independent_price_sources
        };
        if market.independent_price_sources < required_sources {
            reasons.push(Rejection::TooFewIndependentPriceSources);
        }

        let fee_cap = if emergency {
            self.config.emergency_exit_max_route_fee_bps
        } else {
            self.config.max_route_fee_bps
        };
        if execution.route_fee_bps > fee_cap {
            reasons.push(Rejection::RouteFeeTooHigh);
        }
    }

    fn evaluate_entry(
        &self,
        proposal: &ModelProposal,
        token: &TokenEvidence,
        market: &MarketEvidence,
        execution: &ExecutionEvidence,
        portfolio: &PortfolioState,
        reasons: &mut Vec<Rejection>,
    ) {
        let decision_at = execution.timeline.decision_recorded_at_ms;
        if token.checked_at_ms > decision_at {
            reasons.push(Rejection::TokenEvidenceFromFuture);
        } else if decision_at.saturating_sub(token.checked_at_ms)
            > self.config.max_token_evidence_age_ms
        {
            reasons.push(Rejection::TokenEvidenceStale);
        }

        if !token.exact_mint_verified {
            reasons.push(Rejection::ExactMintUnverified);
        }
        if !token.security_gate_passed {
            reasons.push(Rejection::SecurityGateFailed);
        }
        if !token.dex_first_verified {
            reasons.push(Rejection::DexFirstUnverified);
        }
        if !token.entry_trigger_confirmed {
            reasons.push(Rejection::EntryTriggerMissing);
        }
        if market.liquidity_cents < self.config.min_liquidity_cents {
            reasons.push(Rejection::LiquidityTooLow);
        }
        if market.volume_24h_cents < self.config.min_volume_24h_cents {
            reasons.push(Rejection::VolumeTooLow);
        }
        if market.market_cap_cents == 0 {
            reasons.push(Rejection::MarketCapInvalid);
        } else {
            if market.market_cap_cents > self.config.max_market_cap_cents {
                reasons.push(Rejection::MarketCapTooHigh);
            }
            if ratio_bps(market.volume_24h_cents, market.market_cap_cents)
                < self.config.min_volume_to_market_cap_bps
            {
                reasons.push(Rejection::VolumeMarketCapAbsorptionTooLow);
            }
        }
        if market.geckoterminal_score < self.config.min_geckoterminal_score {
            reasons.push(Rejection::GeckoTerminalScoreTooLow);
        }
        if execution.slippage_bps > self.config.max_slippage_bps {
            reasons.push(Rejection::SlippageTooHigh);
        }
        if execution.price_impact_bps > self.config.max_price_impact_bps {
            reasons.push(Rejection::PriceImpactTooHigh);
        }

        let daily_loss_limit = pct_of(portfolio.nav_cents, self.config.max_daily_loss_bps_of_nav);
        if portfolio.daily_realized_loss_cents >= daily_loss_limit {
            reasons.push(Rejection::DailyLossLimitReached);
        }
        if proposal.notional_cents == 0 {
            reasons.push(Rejection::PositionSizeInvalid);
        }

        let max_position = pct_of(portfolio.nav_cents, self.config.max_position_bps_of_nav);
        if proposal.notional_cents > max_position {
            reasons.push(Rejection::PositionSizeTooLarge);
        }
        if proposal.notional_cents > portfolio.available_cash_cents {
            reasons.push(Rejection::InsufficientCash);
        }

        let max_total_exposure = pct_of(
            portfolio.nav_cents,
            self.config.max_total_exposure_bps_of_nav,
        );
        match portfolio
            .total_exposure_cents
            .checked_add(proposal.notional_cents)
        {
            Some(projected) if projected > max_total_exposure => {
                reasons.push(Rejection::TotalExposureTooHigh)
            }
            None => reasons.push(Rejection::ArithmeticOverflow),
            _ => {}
        }

        if portfolio.open_positions >= self.config.max_open_positions {
            reasons.push(Rejection::OpenPositionLimitReached);
        }
    }

    fn evaluate_exit(
        &self,
        proposal: &ModelProposal,
        execution: &ExecutionEvidence,
        portfolio: &PortfolioState,
        emergency: bool,
        reasons: &mut Vec<Rejection>,
    ) {
        if proposal.notional_cents == 0 {
            reasons.push(Rejection::ExitAmountInvalid);
        }
        if proposal.notional_cents > portfolio.current_position_value_cents {
            reasons.push(Rejection::ExitAmountExceedsPosition);
        }

        let slippage_cap = if emergency {
            self.config.emergency_exit_max_slippage_bps
        } else {
            self.config.max_slippage_bps
        };
        let impact_cap = if emergency {
            self.config.emergency_exit_max_price_impact_bps
        } else {
            self.config.max_price_impact_bps
        };
        if execution.slippage_bps > slippage_cap {
            reasons.push(Rejection::SlippageTooHigh);
        }
        if execution.price_impact_bps > impact_cap {
            reasons.push(Rejection::PriceImpactTooHigh);
        }
    }
}

fn pct_of(value: u64, bps: u32) -> u64 {
    let result = (value as u128 * bps as u128) / 10_000u128;
    result.min(u64::MAX as u128) as u64
}

fn ratio_bps(numerator: u64, denominator: u64) -> u32 {
    if denominator == 0 {
        return 0;
    }
    let result = (numerator as u128 * 10_000u128) / denominator as u128;
    result.min(u32::MAX as u128) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(
        purpose: Purpose,
    ) -> (
        ModelProposal,
        EvidenceBinding,
        TokenEvidence,
        MarketEvidence,
        ExecutionEvidence,
        PortfolioState,
    ) {
        let proposal = ModelProposal {
            mint: "ExampleMint111111111111111111111111111111".into(),
            purpose,
            mode: ExecutionMode::Paper,
            notional_cents: 10_000,
            phenotype: Phenotype::DexFirstLaunch,
            reason: "fixture".into(),
        };
        let binding = EvidenceBinding::new(
            proposal.mint.as_str(),
            proposal.mint.as_str(),
            proposal.mint.as_str(),
        );
        let decision = 995_500;
        let token = TokenEvidence {
            checked_at_ms: decision - 10_000,
            exact_mint_verified: true,
            security_gate_passed: true,
            dex_first_verified: true,
            entry_trigger_confirmed: true,
        };
        let market = MarketEvidence {
            as_of_ms: decision - 1_000,
            liquidity_cents: 2_000_000,
            volume_24h_cents: 1_000_000,
            market_cap_cents: 4_000_000,
            geckoterminal_score: 70,
            independent_price_sources: 2,
        };
        let execution = ExecutionEvidence {
            mode: proposal.mode,
            purpose,
            quoted_notional_cents: proposal.notional_cents,
            route_verified: true,
            slippage_bps: 100,
            price_impact_bps: 50,
            route_fee_bps: 30,
            price_divergence_bps: 100,
            timeline: TimelineEvidence {
                observed_at_ms: 990_000,
                armed_at_ms: 993_000,
                signal_at_ms: 995_000,
                decision_recorded_at_ms: decision,
                quote_at_ms: 996_000,
                now_ms: 1_000_000,
            },
        };
        let portfolio = PortfolioState {
            nav_cents: 100_000,
            available_cash_cents: 50_000,
            total_exposure_cents: 0,
            daily_realized_loss_cents: 0,
            open_positions: 0,
            current_position_value_cents: 20_000,
            entry_halt_active: false,
            global_kill_switch_active: false,
        };
        (proposal, binding, token, market, execution, portfolio)
    }

    #[test]
    fn clean_entry_is_allowed() {
        let (p, b, t, m, e, s) = fixture(Purpose::Entry);
        let d = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(d.allowed, "unexpected rejections: {:?}", d.reasons);
    }

    #[test]
    fn mismatched_execution_context_is_rejected() {
        let (p, b, t, m, mut e, s) = fixture(Purpose::Entry);
        e.quoted_notional_cents += 1;
        let d = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(d.reasons.contains(&Rejection::QuoteNotionalMismatch));
    }

    #[test]
    fn post_decision_quote_is_required() {
        let (p, b, t, m, mut e, s) = fixture(Purpose::Entry);
        e.timeline.quote_at_ms = e.timeline.decision_recorded_at_ms;
        let d = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(d.reasons.contains(&Rejection::QuoteNotAfterDecision));
    }

    #[test]
    fn future_market_evidence_is_rejected() {
        let (p, b, t, mut m, e, s) = fixture(Purpose::Entry);
        m.as_of_ms = e.timeline.decision_recorded_at_ms + 1;
        let d = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(d.reasons.contains(&Rejection::MarketEvidenceFromFuture));
    }

    #[test]
    fn entry_halt_does_not_trap_exit() {
        let (mut p, b, mut t, mut m, mut e, mut s) = fixture(Purpose::Exit);
        p.notional_cents = 5_000;
        e.quoted_notional_cents = p.notional_cents;
        s.entry_halt_active = true;
        t.exact_mint_verified = false;
        t.security_gate_passed = false;
        t.dex_first_verified = false;
        t.entry_trigger_confirmed = false;
        m.liquidity_cents = 0;
        m.volume_24h_cents = 0;
        m.market_cap_cents = 0;
        m.geckoterminal_score = 0;
        let d = PolicyEngine::new(RiskConfig::default()).evaluate(&p, &b, &t, &m, &e, &s);
        assert!(d.allowed, "exit trapped: {:?}", d.reasons);
    }
}
