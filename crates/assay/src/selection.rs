//! Risk-weighted, staleness-aware target selection.
//!
//! Deliberately pure: every function here takes signals as plain data and
//! returns a decision. Sourcing those signals from a live checkout of the
//! target repo (git diff, doc-comment scan, dependency fan-in, commit
//! authorship) is wiring work for the weekly job, not this module's concern
//! — see `docs/maintainer-assay.md`, "Cadence and selection".

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::Concept;

/// The signals `docs/maintainer-assay.md`'s "Cadence and selection" names,
/// roughly in the order it ranks them. Each is a yes/no or a count because
/// the source data (a doc-comment, a commit trailer, a test-file listing) is
/// itself categorical — a false precision of continuous scores would just be
/// noise dressed up as rigor.
// Eight independent, orthogonal signals — not a state machine, so a
// two-variant-enum-per-flag refactor (clippy's suggested fix) would only add
// ceremony without expressing anything real about the domain.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct RiskSignals {
	pub has_stated_invariant: bool,
	pub concurrency_or_await_boundary: bool,
	pub persistence_or_transaction_boundary: bool,
	pub single_owner_assumption: bool,
	pub commit_churn: u32,
	pub fan_in: u32,
	pub missing_tests: bool,
	pub agent_coauthored: bool,
}

/// A weighted sum, not a model — deliberately legible over accurate. Anyone
/// disputing a selection should be able to check this function by eye
/// against the signals it was given, the same way the rest of this system
/// insists an assertion be checkable rather than trusted.
#[must_use]
pub fn risk_weight(signals: &RiskSignals) -> f64 {
	let mut score = 0.0;
	if signals.has_stated_invariant {
		score += 3.0;
	}
	if signals.concurrency_or_await_boundary {
		score += 2.5;
	}
	if signals.persistence_or_transaction_boundary {
		score += 2.5;
	}
	if signals.single_owner_assumption {
		score += 2.0;
	}
	if signals.missing_tests {
		score += 1.5;
	}
	if signals.agent_coauthored {
		score += 1.0;
	}
	score += f64::from(signals.commit_churn).min(10.0) * 0.1;
	score += f64::from(signals.fan_in).min(10.0) * 0.1;
	score
}

/// A concept paired with the signals behind its risk weight, plus when it
/// was last demonstrated (if ever) — the two inputs staleness needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
	pub concept: Concept,
	pub signals: RiskSignals,
	pub last_demonstrated_at: Option<DateTime<Utc>>,
}

/// A concept whose supporting source changed since it was last demonstrated
/// is stale by definition and should out-compete an unprobed concept with
/// the same raw risk weight — see the spec's freshness requirement. Modeled
/// as a flat multiplier rather than a decay curve: legibility over realism,
/// same reasoning as `risk_weight`.
const STALE_MULTIPLIER: f64 = 1.5;

/// Ranks candidates by risk weight, boosted for staleness, descending.
/// Never-probed concepts (`last_demonstrated_at: None`) are staleness-boosted
/// too — an unprobed concept is maximally stale by construction.
#[must_use]
pub fn rank(mut candidates: Vec<Candidate>, source_changed_since: impl Fn(&Concept, DateTime<Utc>) -> bool) -> Vec<(Candidate, f64)> {
	let scored: Vec<(Candidate, f64)> = candidates
		.drain(..)
		.map(|c| {
			let base = risk_weight(&c.signals);
			let stale = match c.last_demonstrated_at {
				None => true,
				Some(at) => source_changed_since(&c.concept, at),
			};
			let score = if stale { base * STALE_MULTIPLIER } else { base };
			(c, score)
		})
		.collect();

	let mut scored = scored;
	scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	scored
}

#[cfg(test)]
mod tests {
	use super::*;

	fn concept(id: &str) -> Concept {
		Concept {
			id: id.to_string(),
			description: String::new(),
			source_anchors: vec![],
		}
	}

	#[test]
	fn stated_invariant_outweighs_bare_churn() {
		let with_invariant = RiskSignals {
			has_stated_invariant: true,
			..Default::default()
		};
		let just_churn = RiskSignals {
			commit_churn: 50,
			..Default::default()
		};
		assert!(risk_weight(&with_invariant) > risk_weight(&just_churn));
	}

	#[test]
	fn never_probed_ranks_above_equal_risk_recently_probed() {
		let never = Candidate {
			concept: concept("a"),
			signals: RiskSignals {
				has_stated_invariant: true,
				..Default::default()
			},
			last_demonstrated_at: None,
		};
		let recent = Candidate {
			concept: concept("b"),
			signals: RiskSignals {
				has_stated_invariant: true,
				..Default::default()
			},
			last_demonstrated_at: Some(Utc::now()),
		};
		let ranked = rank(vec![recent, never], |_, _| false);
		assert_eq!(ranked[0].0.concept.id, "a");
	}

	#[test]
	fn stale_probed_concept_outranks_fresh_lower_risk_one() {
		let stale_but_risky = Candidate {
			concept: concept("risky"),
			signals: RiskSignals {
				has_stated_invariant: true,
				concurrency_or_await_boundary: true,
				..Default::default()
			},
			last_demonstrated_at: Some(Utc::now()),
		};
		let fresh_low_risk = Candidate {
			concept: concept("low-risk"),
			signals: RiskSignals::default(),
			last_demonstrated_at: None,
		};
		// source_changed_since always true: everything demonstrated is stale.
		let ranked = rank(vec![fresh_low_risk, stale_but_risky], |_, _| true);
		assert_eq!(ranked[0].0.concept.id, "risky");
	}
}
