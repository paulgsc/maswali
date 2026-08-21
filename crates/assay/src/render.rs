//! Derives a queryable snapshot from the ledger.
//!
//! This module does not format a report. Presentation is a client's job —
//! concretely, a dedicated route in the maintainer's own site — and baking a
//! bespoke report format into the harness would be exactly the kind of
//! idiosyncratic indirection this repo exists to avoid. What the harness
//! owes any client is a clean, typed, stable data contract: this snapshot,
//! serialized as JSON.
//!
//! The one opinion this module does hold: "never probed" stays a list of
//! concept ids, not a percentage. A percentage would treat a `Declined`
//! probe and a `Proficient` one as equally "covering" a concept, which is
//! false, and would reopen the "gamified coverage counter" anti-goal. A
//! consuming UI is free to compute and display a ratio from the two list
//! lengths below if it wants one — that's a presentation choice, not a claim
//! this module should make on its behalf.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::schema::{BloomOp, Concept, DreyfusLevel, EvidenceRecord, ProbeOutcome};

/// One row of the probed-concepts view: the latest record for a concept,
/// reduced to what a client needs without re-deriving ledger semantics
/// itself.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConceptStatus {
	pub concept: Concept,
	pub dreyfus_assessed: Option<DreyfusLevel>,
	pub bloom_targeted: BloomOp,
	pub outcome_summary: String,
	pub target_revision: String,
	pub last_probed_at: chrono::DateTime<chrono::Utc>,
}

/// The full snapshot a client renders from. Everything in here is derived
/// and safe to regenerate at any time from the ledger plus the current
/// concept inventory — nothing here is itself a source of truth.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompetencySnapshot {
	pub probed: Vec<ConceptStatus>,
	pub never_probed: Vec<Concept>,
	pub generated_at: chrono::DateTime<chrono::Utc>,
}

fn describe_outcome(outcome: &ProbeOutcome) -> String {
	match outcome {
		ProbeOutcome::Hazard { status } => format!("{status:?}"),
		ProbeOutcome::Provisioning { status } => format!("{status:?}"),
		ProbeOutcome::Mutation { accepted, amended } => match (accepted, amended) {
			(true, _) => "accepted".to_string(),
			(false, true) => "rejected, amended".to_string(),
			(false, false) => "rejected".to_string(),
		},
		ProbeOutcome::Declined => "declined".to_string(),
	}
}

/// The latest record per concept id, by timestamp. Earlier records for the
/// same concept remain in the ledger untouched — this is a read-time
/// reduction, not a compaction.
fn latest_per_concept(records: &[EvidenceRecord]) -> Vec<&EvidenceRecord> {
	let mut latest: HashMap<&str, &EvidenceRecord> = HashMap::new();
	for record in records {
		latest
			.entry(record.concept.id.as_str())
			.and_modify(|existing| {
				if record.timestamp > existing.timestamp {
					*existing = record;
				}
			})
			.or_insert(record);
	}
	let mut rows: Vec<&EvidenceRecord> = latest.into_values().collect();
	rows.sort_by(|a, b| a.concept.id.cmp(&b.concept.id));
	rows
}

/// Builds the snapshot. `all_concepts` should be every concept currently
/// known to the selector, so `never_probed` can name what the ledger has
/// never touched at all, not just what it happens to contain.
#[must_use]
pub fn snapshot(all_concepts: &[Concept], records: &[EvidenceRecord]) -> CompetencySnapshot {
	let latest = latest_per_concept(records);
	let probed_ids: std::collections::HashSet<&str> = latest.iter().map(|r| r.concept.id.as_str()).collect();

	let probed = latest
		.into_iter()
		.map(|r| ConceptStatus {
			concept: r.concept.clone(),
			dreyfus_assessed: r.dreyfus_assessed,
			bloom_targeted: r.bloom_op_targeted,
			outcome_summary: describe_outcome(&r.outcome),
			target_revision: r.target_revision.clone(),
			last_probed_at: r.timestamp,
		})
		.collect();

	let never_probed = all_concepts.iter().filter(|c| !probed_ids.contains(c.id.as_str())).cloned().collect();

	CompetencySnapshot {
		probed,
		never_probed,
		generated_at: chrono::Utc::now(),
	}
}

#[cfg(test)]
mod tests {
	use chrono::Utc;

	use super::*;
	use crate::schema::ProbeFamily;

	fn concept(id: &str) -> Concept {
		Concept {
			id: id.to_string(),
			description: String::new(),
			source_anchors: vec![format!("{id}.rs")],
		}
	}

	fn record(concept_id: &str, timestamp_offset_secs: i64, dreyfus: DreyfusLevel) -> EvidenceRecord {
		EvidenceRecord {
			id: format!("{concept_id}-{timestamp_offset_secs}"),
			target_revision: "0123456789abcdef".to_string(),
			concept: concept(concept_id),
			probe_family: ProbeFamily::ImplementationPerturbation,
			bloom_op_targeted: BloomOp::Analyze,
			dreyfus_level_targeted: DreyfusLevel::Competent,
			prompt: String::new(),
			response: Some(String::new()),
			outcome: ProbeOutcome::Mutation { accepted: false, amended: false },
			dreyfus_assessed: Some(dreyfus),
			demonstrated: vec![],
			gaps: vec![],
			timestamp: Utc::now() + chrono::Duration::seconds(timestamp_offset_secs),
		}
	}

	#[test]
	fn latest_record_wins_when_a_concept_has_several() {
		let older = record("a", 0, DreyfusLevel::Novice);
		let newer = record("a", 3600, DreyfusLevel::Competent);
		let snap = snapshot(&[concept("a")], &[older, newer]);
		assert_eq!(snap.probed.len(), 1);
		assert_eq!(snap.probed[0].dreyfus_assessed, Some(DreyfusLevel::Competent));
	}

	#[test]
	fn unprobed_concept_lands_in_never_probed_not_probed() {
		let snap = snapshot(&[concept("a"), concept("b")], &[record("a", 0, DreyfusLevel::Competent)]);
		assert_eq!(snap.probed.len(), 1);
		assert_eq!(snap.never_probed, vec![concept("b")]);
	}

	#[test]
	fn fully_covered_concept_set_has_empty_never_probed() {
		let snap = snapshot(&[concept("a")], &[record("a", 0, DreyfusLevel::Competent)]);
		assert!(snap.never_probed.is_empty());
	}
}
