#![forbid(unsafe_code)]

pub mod policy;
pub mod simulator;

pub use policy::{
    DecisionReport, EvidenceBinding, ExecutionEvidence, ExecutionMode, MarketEvidence,
    ModelProposal, Phenotype, PolicyEngine, PortfolioState, Purpose, Rejection, RiskConfig,
    TimelineEvidence, TokenEvidence,
};
pub use simulator::{run_adversarial_simulation, SimulationReport};
