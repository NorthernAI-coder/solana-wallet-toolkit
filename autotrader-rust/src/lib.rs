#![forbid(unsafe_code)]

pub mod policy;
pub mod simulator;

pub use policy::{
    compile_mainnet_enabled, DecisionReport, ExecutionEvidence, ExecutionMode, MarketEvidence,
    ModelProposal, Phenotype, PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig,
    TimelineEvidence, TokenEvidence,
};
pub use simulator::{run_adversarial_simulation, SimulationReport};
