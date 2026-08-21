//! The maintainer-assay harness.
//!
//! Deterministic operations only — schema, ledger I/O, risk-weighted
//! selection, and rendering. Challenge prose, rubrics, and mutation diffs are
//! generated elsewhere, by an agent confined to a [`schema::SelectionBrief`]
//! this crate produced. See `docs/maintainer-assay.md`, "Architecture": the
//! crate owns selection and the ledger, the agent owns the prose.

pub mod ledger;
pub mod render;
pub mod schema;
pub mod selection;
