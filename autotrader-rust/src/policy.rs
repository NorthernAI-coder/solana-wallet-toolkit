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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenProgram {
    Legacy,
    Token2022,
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
/// Production adapters must derive these values from their own source responses rather than
/// copying the model proposal.
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

/// Independently collected Solana mint and Token-2022 security facts.
/// These are facts, not a single adapter-provided "safe" verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenEvidence {
    pub checked_at_ms: u64,
    pub exact_mint_verified: bool,
    pub token_program: TokenProgram,
    pub mint_authority_present: bool,
    pub freeze_authority_present: bool,
    pub permanent_delegate_present: bool,
    pub non_transferable: bool,
    pub default_account_state_frozen: bool,
    pub transfer_hook_present: bool,
    pub transfer_hook_program_verified: bool,
    pub pausable: bool,
    pub confidential_transfer_enabled: bool,
    pub scaled_ui_amount_enabled: bool,
    pub current_transfer_fee_bps: u32,
    pub dex_first_verified: bool,
    pub entry_trigger_confirmed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionEvidence {
    pub mode: ExecutionMode,
    pub purpose: Purpose,
    pub quoted_notional_cents: u64,
    pub route_verified: bool,
    /// Exact proposed transaction or route simulation succeeded.
    pub preflight_simulation_passed: bool,
    /// For entries, a reverse sell of the acquired asset also simulated successfully.
    pub reverse_sell_simulation_passed: bool,
    pub estimated_network_fee_cents: u64,
    pub slippage_bps: u32,
    pub price_impact_bps: u32,
    pub route_fee_bps: u32,
    pub price_divergence_bps: u32,
    pub timeline: TimelineEvidence,
}

/// Portfolio snapshot for the proposal mint plus aggregate portfolio values.
/// `current_position_value_cents` is the marked value of the proposal mint only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortfolioState {
    pub nav_cents: u64,
    /// Fixed start-of-risk-period NAV used for daily loss limits.
    pub risk_reference_nav_cents: u64,
    pub available_cash_cents: u64,
    pub total_exposure_cents: u64,
    pub daily_realized_loss_cents: u64,
    pub open_positions: u16,
    pub current_position_mint: Option<String>,
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
    pub max_daily_loss_bps_of_reference_nav: u32,
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
    pub max_quote_age_ms: u64,
    pub max_market_evidence_age_ms: u64,
    pub max_token_evidence_age_ms: u64,
    pub max_open_positions: u16,
    pub max_token_transfer_fee_bps: u32,
    pub allow_mint_authority: bool,
    pub allow_freeze_authority: bool,
    pub allow_permanent_delegate: bool,
    pub allow_transfer_hook: bool,
    pub allow_pausable: bool,
    pub allow_confidential_transfer: bool,
    pub allow_scaled_ui_amount: bool,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_position_bps_of_nav: 1_000,
            max_total_exposure_bps_of_nav: 4_000,
            max_daily_loss_bps_of_reference_nav: 500,
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
            max_quote_age_ms: 15_000,
            max_market_evidence_age_ms: 60_000,
            max_token_evidence_age_ms: 300_000,
            max_open_positions: 5,
            max_token_transfer_fee_bps: 500,
            allow_mint_authority: false,
            allow_freeze_authority: false,
            allow_permanent_delegate: false,
            allow_transfer_hook: false,
            allow_pausable: false,
            allow_confidential_transfer: false,
            allow_scaled_ui_amount: false,
        }
    }
}

impl RiskConfig {
    pub fn is_valid(&self) -> bool {
        self.max_position_bps_of_nav > 0
            && self.max_position_bps_of_nav <= self.max_total_exposure_bps_of_nav
            && self.max_total_exposure_bps_of_nav <= 10_000
            && self.max_daily_loss_bps_of_reference_nav > 0
            && self.max_daily_loss_bps_of_reference_nav <= 10_000
            && self.max_slippage_bps <= 10_000
            && self.max_price_impact_bps <= 10_000
            && self.max_route_fee_bps <= 10_000
            && self.max_price_divergence_bps <= 10_000
            && self.emergency_exit_max_slippage_bps >= self.max_slippage_bps
            && self.emergency_exit_max_slippage_bps <= 10_000
            && self.emergency_exit_max_price_impact_bps >= self.max_price_impact_bps
            && self.emergency_exit_max_price_impact_bps <= 10_000
            && self.emergency_exit_max_route_fee_bps >= self.max_route_fee_bps
            && self.emergency_exit_max_route_fee_bps <= 10_000
            && self.emergency_exit_max_price_divergence_bps >= self.max_price_divergence_bps
            && self.emergency_exit_max_price_divergence_bps <= 10_000
            && self.min_liquidity_cents > 0
            && self.min_volume_24h_cents > 0
            && self.max_market_cap_cents > 0
            && self.min_volume_to_market_cap_bps > 0
            && self.min_geckoterminal_score <= 100
            && self.min_independent_price_sources > 0
            && self.max_quote_age_ms > 0
            && self.max_market_evidence_age_ms > 0
            && self.max_token_evidence_age_ms > 0
            && self.max_open_positions > 0
            && self.max_token_transfer_fee_bps <= 10_000
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    RiskConfigInvalid,
    PortfolioStateInvalid,
    PositionMintMismatch,
    PositionMissing,
    EvidenceMintMismatch,
    ExecutionModeMismatch,
    ExecutionPurposeMismatch,
    QuoteNotionalMismatch,
    GlobalKillSwitch,
    EntryHalt,
    RouteUnverified,
    PreflightSimulationFailed,
    ReverseSellSimulationFailed,
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
    MintAuthorityPresent,
    FreezeAuthorityPresent,
    PermanentDelegatePresent,
    NonTransferableToken,
    DefaultAccountStateFrozen,
    TransferHookNotAllowed,
    TransferHookUnverified,
    PausableToken,
    ConfidentialTransferEnabled,
    ScaledUiAmountEnabled,
    TokenTransferFeeTooHigh,
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
    NetworkFeeUnaffordable,
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

        if !self.config.is_valid() {
            reasons.push(Rejection::RiskConfigInvalid);
        }
        self.validate_portfolio(proposal, portfolio, &mut reasons);

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

        self.evaluate_execution_common(proposal, market, execution, portfolio, &mut reasons);

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

    fn validate_portfolio(
        &self,
        proposal: &ModelProposal,
        portfolio: &PortfolioState,
        reasons: &mut Vec<Rejection>,
    ) {
        let accounting_valid = portfolio.nav_cents > 0
            && portfolio.risk_reference_nav_cents > 0
            && portfolio.total_exposure_cents <= portfolio.nav_cents
            && portfolio.current_position_value_cents <= portfolio.total_exposure_cents
            && portfolio
                .available_cash_cents
                .checked_add(portfolio.total_exposure_cents)
                == Some(portfolio.nav_cents);

        let position_identity_valid = match (
            portfolio.current_position_mint.as_deref(),
            portfolio.current_position_value_cents,
        ) {
            (None, 0) => true,
            (Some(mint), value) if value > 0 => mint == proposal.mint,
            _ => false,
        };

        if !accounting_valid {
            reasons.push(Rejection::PortfolioStateInvalid);
        }
        if !position_identity_valid {
            reasons.push(Rejection::PositionMintMismatch);
        }
        if proposal.purpose != Purpose::Entry && portfolio.current_position_value_cents == 0 {
            reasons.push(Rejection::PositionMissing);
        }
    }

    fn evaluate_execution_common(
        &self,
        proposal: &ModelProposal,
        market: &MarketEvidence,
        execution: &ExecutionEvidence,
        portfolio: &PortfolioState,
        reasons: &mut Vec<Rejection>,
    ) {
        let t = execution.timeline;
        let emergency = proposal.purpose == Purpose::EmergencyExit;

        if !execution.route_verified {
            reasons.push(Rejection::RouteUnverified);
        }
        if !execution.preflight_simulation_passed {
            reasons.push(Rejection::PreflightSimulationFailed);
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

        // Emergency exits remain causal but do not require fresh secondary market data.
        // A fresh executable route and successful preflight remain mandatory.
        if emergency {
            if market.independent_price_sources > 0 {
                if market.as_of_ms > t.decision_recorded_at_ms {
                    reasons.push(Rejection::MarketEvidenceFromFuture);
                }
                if execution.price_divergence_bps > self.config.emergency_exit_max_price_divergence_bps {
                    reasons.push(Rejection::PriceSourceDivergence);
                }
            }
        } else {
            if market.as_of_ms > t.decision_recorded_at_ms {
                reasons.push(Rejection::MarketEvidenceFromFuture);
            } else if t.decision_recorded_at_ms.saturating_sub(market.as_of_ms)
                > self.config.max_market_evidence_age_ms
            {
                reasons.push(Rejection::MarketEvidenceStale);
            }
            if execution.price_divergence_bps > self.config.max_price_divergence_bps {
                reasons.push(Rejection::PriceSourceDivergence);
            }
            if market.independent_price_sources < self.config.min_independent_price_sources {
                reasons.push(Rejection::TooFewIndependentPriceSources);
            }
        }

        let fee_cap = if emergency {
            self.config.emergency_exit_max_route_fee_bps
        } else {
            self.config.max_route_fee_bps
        };
        if execution.route_fee_bps > fee_cap {
            reasons.push(Rejection::RouteFeeTooHigh);
        }

        if execution.estimated_network_fee_cents > portfolio.available_cash_cents {
            reasons.push(Rejection::NetworkFeeUnaffordable);
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
        if token.mint_authority_present && !self.config.allow_mint_authority {
            reasons.push(Rejection::MintAuthorityPresent);
        }
        if token.freeze_authority_present && !self.config.allow_freeze_authority {
            reasons.push(Rejection::FreezeAuthorityPresent);
        }
        if token.permanent_delegate_present && !self.config.allow_permanent_delegate {
            reasons.push(Rejection::PermanentDelegatePresent);
        }
        if token.non_transferable {
            reasons.push(Rejection::NonTransferableToken);
        }
        if token.default_account_state_frozen {
            reasons.push(Rejection::DefaultAccountStateFrozen);
        }
        if token.transfer_hook_present {
            if !self.config.allow_transfer_hook {
                reasons.push(Rejection::TransferHookNotAllowed);
            }
            if !token.transfer_hook_program_verified {
                reasons.push(Rejection::TransferHookUnverified);
            }
        }
        if token.pausable && !self.config.allow_pausable {
            reasons.push(Rejection::PausableToken);
        }
        if token.confidential_transfer_enabled && !self.config.allow_confidential_transfer {
            reasons.push(Rejection::ConfidentialTransferEnabled);
        }
        if token.scaled_ui_amount_enabled && !self.config.allow_scaled_ui_amount {
            reasons.push(Rejection::ScaledUiAmountEnabled);
        }
        if token.current_transfer_fee_bps > self.config.max_token_transfer_fee_bps {
            reasons.push(Rejection::TokenTransferFeeTooHigh);
        }
        if !execution.reverse_sell_simulation_passed {
            reasons.push(Rejection::ReverseSellSimulationFailed);
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

        let daily_loss_limit = pct_of(
            portfolio.risk_reference_nav_cents,
            self.config.max_daily_loss_bps_of_reference_nav,
        );
        if portfolio.daily_realized_loss_cents >= daily_loss_limit {
            reasons.push(Rejection::DailyLossLimitReached);
        }
        if proposal.notional_cents == 0 {
            reasons.push(Rejection::PositionSizeInvalid);
        }

        let max_position = pct_of(portfolio.nav_cents, self.config.max_position_bps_of_nav);
        match portfolio
            .current_position_value_cents
            .checked_add(proposal.notional_cents)
        {
            Some(projected_position) if projected_position > max_position => {
                reasons.push(Rejection::PositionSizeTooLarge)
            }
            None => reasons.push(Rejection::ArithmeticOverflow),
            _ => {}
        }

        let entry_cash_needed = proposal
            .notional_cents
            .checked_add(execution.estimated_network_fee_cents);
        match entry_cash_needed {
            Some(required) if required > portfolio.available_cash_cents => {
                reasons.push(Rejection::InsufficientCash)
            }
            None => reasons.push(Rejection::ArithmeticOverflow),
            _ => {}
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

        if portfolio.current_position_value_cents == 0
            && portfolio.open_positions >= self.config.max_open_positions
        {
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
