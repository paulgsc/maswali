//! Core types for the maintainer assay.
//!
//! See `docs/maintainer-assay.md` for the design these types encode. Nothing
//! here does I/O or makes a judgment call — that's `ledger`, `selection`, and
//! `render`. This module only says what a challenge, a probe family, and an
//! evidence record *are*.

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A repository concept this assay can probe: an architectural boundary, a
/// domain invariant, an API contract, a state machine, and so on. Always
/// grounded in named source anchors — a concept with no anchors is not
/// eligible for selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Concept {
	pub id: String,
	pub description: String,
	/// Repo-relative paths, ideally with a line range, e.g.
	/// `crates/intervention/src/charge.rs:33-40`.
	pub source_anchors: Vec<String>,
}

/// Which side of `(C, A)` a probe perturbs. See `docs/maintainer-assay.md`,
/// "Two probe families, not one".
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum ProbeFamily {
	/// `(C, A) -> (C', A)` — a plausible, invariant-breaking edit to real
	/// code. Requires a mutant that compiles and survives the target's test
	/// suite; see `docs/maintainer-assay.md`'s difficulty oracle.
	ImplementationPerturbation,
	/// `(C, A) -> (C, A')` — one deployment assumption changed, code
	/// untouched.
	EnvironmentPerturbation { direction: HazardDirection },
}

/// Which way an environment-perturbation probe runs the audit between a
/// hazard and the machinery that would (or wouldn't) address it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HazardDirection {
	/// "Is this real hazard handled?" — under-provisioning.
	HazardFirst,
	/// "Is this machinery earning its cost?" — over-provisioning.
	MechanismFirst,
}

/// Bloom's revised taxonomy, restricted to the six operations this system
/// tracks. Ordered low-to-high; `PartialOrd`'s derived order matches variant
/// declaration order.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BloomOp {
	Remember,
	Understand,
	Apply,
	Analyze,
	Evaluate,
	Create,
}

/// Dreyfus skill-acquisition level, per concept — never global. Ordered
/// low-to-high for the same reason as [`BloomOp`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DreyfusLevel {
	Novice,
	AdvancedBeginner,
	Competent,
	Proficient,
	Expert,
}

/// The classification a hazard-first probe asks the maintainer to produce
/// for a hazard `g` against implementation `I` under assumptions `A`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HazardStatus {
	ImpossibleUnderA,
	HandledByI,
	UnhandledButAcceptableUnderA,
	LatentDefect,
}

/// The classification a mechanism-first probe asks the maintainer to produce
/// for machinery `M` already present in `I`. The inverse of [`HazardStatus`]:
/// a defect here is paid for continuously, not waited on.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProvisioningStatus {
	Justified,
	ExplicitHedge,
	UnjustifiedTax,
}

/// Whether a completed probe found a hazard/mechanism admissible or not.
/// Distinct from [`HazardStatus`]/[`ProvisioningStatus`] because a mutation
/// probe's rubric is accept/reject/amend, not this four-way split — this is
/// the outcome shape shared across both probe families for ledger purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum ProbeOutcome {
	Hazard {
		status: HazardStatus,
	},
	Provisioning {
		status: ProvisioningStatus,
	},
	/// Accept, reject, or amend a mutation — the implementation-perturbation
	/// rubric shape.
	Mutation {
		accepted: bool,
		amended: bool,
	},
	/// The challenge reached its deadline with no maintainer response.
	Declined,
}

/// The brief a selection pass produces *before* generation — never the other
/// way around. See acceptance criterion 2 in `docs/maintainer-assay.md`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionBrief {
	pub concept: Concept,
	pub probe_family: ProbeFamily,
	pub bloom_op: BloomOp,
	pub risk_score: f64,
	pub rationale: String,
	/// The target repo's revision this brief was generated against —
	/// `paulgsc/server`, not this repo.
	pub target_revision: String,
	pub generated_at: DateTime<Utc>,
}

impl fmt::Display for BloomOp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Remember => "Remember",
			Self::Understand => "Understand",
			Self::Apply => "Apply",
			Self::Analyze => "Analyze",
			Self::Evaluate => "Evaluate",
			Self::Create => "Create",
		};
		f.write_str(s)
	}
}

impl fmt::Display for DreyfusLevel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Novice => "Novice",
			Self::AdvancedBeginner => "Advanced Beginner",
			Self::Competent => "Competent",
			Self::Proficient => "Proficient",
			Self::Expert => "Expert",
		};
		f.write_str(s)
	}
}

/// One append-only ledger entry. Never mutated after being written; a
/// re-evaluation of the same concept is a new record, not an edit to this
/// one. See `docs/maintainer-assay.md`'s evidence model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceRecord {
	pub id: String,
	/// The target repo's revision this record's challenge and evaluation are
	/// pinned to.
	pub target_revision: String,
	pub concept: Concept,
	pub probe_family: ProbeFamily,
	pub bloom_op_targeted: BloomOp,
	pub dreyfus_level_targeted: DreyfusLevel,
	pub prompt: String,
	/// `None` only when `outcome` is `ProbeOutcome::Declined`.
	pub response: Option<String>,
	pub outcome: ProbeOutcome,
	/// The evaluator's inferred level, distinct from `dreyfus_level_targeted`
	/// — the probe's aim and the response's actual level are different
	/// claims, and the spec's evidence model forbids conflating them.
	/// `None` only when `outcome` is `ProbeOutcome::Declined`; there is
	/// nothing to assess.
	pub dreyfus_assessed: Option<DreyfusLevel>,
	/// What the maintainer actually demonstrated, in the evaluator's words —
	/// kept distinct from what got them there, per the observed-evidence /
	/// evaluator-inference split `docs/maintainer-assay.md` insists on.
	pub demonstrated: Vec<String>,
	/// The reasoning node that was missing or wrong, if any. Empty only on a
	/// fully correct response.
	pub gaps: Vec<String>,
	pub timestamp: DateTime<Utc>,
}
