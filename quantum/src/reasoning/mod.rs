// Reasoning — elimination + semantic graph + convergence + territories.
// Teorite 27, 30, 31, 34.
pub mod elimination;
pub mod semantic_graph;
pub mod convergence;
pub mod territories;

pub use elimination::{Elimination, ElimCandidate, THRESHOLD_LOCAL, THRESHOLD_REGIONAL, THRESHOLD_GLOBAL};
pub use semantic_graph::{SemanticGraph, SemanticNode, SemanticEdge, Relation};
pub use convergence::{Convergence, BestCandidate, SelectionMethod, Transfer, FinalOutput};
pub use territories::{ReasoningTerritories, Territory, EliminationMode, ContradictionPolicy};
