//! The append-only evidence ledger.
//!
//! One JSON file per record, named by timestamp and id so a plain directory
//! listing sorts chronologically. Never overwritten — see
//! `docs/maintainer-assay.md`'s "rewrite-in-place discards the deliverable".
//! `challenge.md` and `selection.json`, by contrast, are meant to be
//! rewritten weekly and are not this module's concern.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::schema::EvidenceRecord;

/// Where one record lands, given the ledger directory it's being appended
/// to. Deterministic from the record's own id and timestamp, not from
/// directory contents — so two processes computing this for the same record
/// agree without needing to list the directory first.
fn record_path(ledger_dir: &Path, record: &EvidenceRecord) -> PathBuf {
	let stamp = record.timestamp.format("%Y%m%dT%H%M%SZ");
	ledger_dir.join(format!("{stamp}-{}.json", record.id))
}

/// Appends one record. Refuses to overwrite an existing file at the computed
/// path — the ledger's append-only property is enforced here, not just
/// documented.
///
/// # Errors
///
/// Returns an error if the ledger directory can't be created, a record
/// already exists at the computed path, or the write itself fails.
pub fn append(ledger_dir: &Path, record: &EvidenceRecord) -> Result<PathBuf> {
	fs::create_dir_all(ledger_dir).with_context(|| format!("creating ledger dir {}", ledger_dir.display()))?;

	let path = record_path(ledger_dir, record);
	if path.exists() {
		bail!("refusing to overwrite existing ledger record at {}", path.display());
	}

	let json = serde_json::to_string_pretty(record).context("serializing evidence record")?;
	fs::write(&path, json).with_context(|| format!("writing ledger record to {}", path.display()))?;
	Ok(path)
}

/// Reads every record in the ledger directory, oldest first. A missing
/// directory reads as an empty ledger rather than an error — a repo that has
/// never run a probe has a ledger, it's just empty.
///
/// # Errors
///
/// Returns an error if the directory exists but can't be read, or if any
/// `.json` file in it fails to parse as an [`EvidenceRecord`].
pub fn read_all(ledger_dir: &Path) -> Result<Vec<EvidenceRecord>> {
	if !ledger_dir.exists() {
		return Ok(Vec::new());
	}

	let mut records = Vec::new();
	for entry in fs::read_dir(ledger_dir).with_context(|| format!("reading ledger dir {}", ledger_dir.display()))? {
		let entry = entry?;
		let path = entry.path();
		if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
			continue;
		}
		let contents = fs::read_to_string(&path).with_context(|| format!("reading ledger record {}", path.display()))?;
		let record: EvidenceRecord = serde_json::from_str(&contents).with_context(|| format!("parsing ledger record {}", path.display()))?;
		records.push(record);
	}

	records.sort_by_key(|r| r.timestamp);
	Ok(records)
}

#[cfg(test)]
mod tests {
	use chrono::Utc;
	use tempfile::tempdir;

	use super::*;
	use crate::schema::{BloomOp, Concept, DreyfusLevel, ProbeFamily, ProbeOutcome};

	fn sample_record(id: &str) -> EvidenceRecord {
		EvidenceRecord {
			id: id.to_string(),
			target_revision: "deadbeef".to_string(),
			concept: Concept {
				id: "waker.single-consumer".to_string(),
				description: "run_once assumes exactly one waker process".to_string(),
				source_anchors: vec!["apps/servers/file_host/src/nudge/waker.rs".to_string()],
			},
			probe_family: ProbeFamily::ImplementationPerturbation,
			bloom_op_targeted: BloomOp::Analyze,
			dreyfus_level_targeted: DreyfusLevel::Competent,
			prompt: "does this survive a second waker instance?".to_string(),
			response: Some("no, here's why".to_string()),
			outcome: ProbeOutcome::Mutation { accepted: false, amended: false },
			dreyfus_assessed: Some(DreyfusLevel::Competent),
			demonstrated: vec!["named the missing claim/lock on the polled row".to_string()],
			gaps: vec![],
			timestamp: Utc::now(),
		}
	}

	#[test]
	fn append_then_read_round_trips() {
		let dir = tempdir().unwrap();
		let record = sample_record("r1");
		append(dir.path(), &record).unwrap();

		let read_back = read_all(dir.path()).unwrap();
		assert_eq!(read_back.len(), 1);
		assert_eq!(read_back[0].id, "r1");
	}

	#[test]
	fn append_refuses_to_overwrite() {
		let dir = tempdir().unwrap();
		let record = sample_record("r1");
		append(dir.path(), &record).unwrap();

		let err = append(dir.path(), &record).unwrap_err();
		assert!(err.to_string().contains("refusing to overwrite"));
	}

	#[test]
	fn read_all_on_missing_dir_is_empty_not_error() {
		let dir = tempdir().unwrap();
		let missing = dir.path().join("does-not-exist");
		assert_eq!(read_all(&missing).unwrap(), Vec::new());
	}

	#[test]
	fn read_all_sorts_oldest_first() {
		let dir = tempdir().unwrap();
		let mut earlier = sample_record("later-id-earlier-time");
		earlier.timestamp = Utc::now() - chrono::Duration::days(7);
		let later = sample_record("earlier-id-later-time");

		// Append in reverse chronological order to prove the sort, not the
		// filesystem's own directory-listing order, is what's tested.
		append(dir.path(), &later).unwrap();
		append(dir.path(), &earlier).unwrap();

		let records = read_all(dir.path()).unwrap();
		assert_eq!(records[0].id, "later-id-earlier-time");
		assert_eq!(records[1].id, "earlier-id-later-time");
	}
}
