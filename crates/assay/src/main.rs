//! CLI entry point. Thin by design — each subcommand is a few lines of glue
//! over `assay::{ledger, render, selection}`; the logic worth trusting lives
//! in the library, where it's tested.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use assay::schema::{Concept, EvidenceRecord, SelectionBrief};
use assay::selection::Candidate;
use assay::{ledger, render, selection};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "assay", about = "Repository-grounded maintainer-competency harness")]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	/// Rank candidates and emit the top pick as a `SelectionBrief`, printed
	/// to stdout as JSON. Does not generate challenge prose — that's the
	/// agent's job, confined to the brief this prints.
	Select {
		/// Path to a JSON file containing a `Vec<selection::Candidate>`.
		#[arg(long)]
		candidates: PathBuf,
		/// The target repo's revision this selection is being made against.
		#[arg(long)]
		revision: String,
		/// Newline-separated file paths changed since the last challenge,
		/// e.g. from `git diff --name-only <last>..HEAD` against the target
		/// repo. A concept is stale if any of its source anchors appears
		/// here, or if it has never been demonstrated.
		#[arg(long)]
		changed_files: Option<PathBuf>,
	},
	/// Append one evidence record to the ledger.
	Record {
		#[arg(long)]
		ledger: PathBuf,
		/// Path to a JSON file containing one `EvidenceRecord`.
		#[arg(long)]
		input: PathBuf,
	},
	/// Derive a `CompetencySnapshot` from the ledger and write it as JSON —
	/// the data contract a client (e.g. a website route) renders from. This
	/// crate does not format a report; see `render`'s module docs.
	Render {
		#[arg(long)]
		ledger: PathBuf,
		/// Path to a JSON file containing a `Vec<Concept>` — the full known
		/// inventory, so `never_probed` can name what the ledger has no
		/// records for at all.
		#[arg(long)]
		concepts: PathBuf,
		#[arg(long)]
		out: PathBuf,
	},
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	match cli.command {
		Command::Select {
			candidates,
			revision,
			changed_files,
		} => run_select(&candidates, &revision, changed_files.as_deref()),
		Command::Record { ledger, input } => run_record(&ledger, &input),
		Command::Render { ledger, concepts, out } => run_render(&ledger, &concepts, &out),
	}
}

fn run_select(candidates_path: &std::path::Path, revision: &str, changed_files_path: Option<&std::path::Path>) -> Result<()> {
	let raw = fs::read_to_string(candidates_path).with_context(|| format!("reading {}", candidates_path.display()))?;
	let candidates: Vec<Candidate> = serde_json::from_str(&raw).context("parsing candidates JSON")?;

	let changed: Vec<String> = match changed_files_path {
		Some(path) => {
			let contents = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
			contents.lines().map(str::to_string).collect()
		}
		None => Vec::new(),
	};

	let ranked = selection::rank(candidates, |concept, _last_demonstrated| {
		concept
			.source_anchors
			.iter()
			.any(|anchor| changed.iter().any(|c| anchor.starts_with(c.as_str()) || c.starts_with(anchor.as_str())))
	});

	let Some((top, score)) = ranked.into_iter().next() else {
		anyhow::bail!("no candidates given, nothing to select");
	};

	let brief = SelectionBrief {
		concept: top.concept,
		probe_family: assay::schema::ProbeFamily::EnvironmentPerturbation {
			direction: assay::schema::HazardDirection::HazardFirst,
		},
		bloom_op: assay::schema::BloomOp::Analyze,
		risk_score: score,
		rationale: format!("highest risk-weighted, staleness-boosted score among {} candidate(s)", changed.len()),
		target_revision: revision.to_string(),
		generated_at: chrono::Utc::now(),
	};

	println!("{}", serde_json::to_string_pretty(&brief)?);
	Ok(())
}

fn run_record(ledger_dir: &std::path::Path, input_path: &std::path::Path) -> Result<()> {
	let raw = fs::read_to_string(input_path).with_context(|| format!("reading {}", input_path.display()))?;
	let record: EvidenceRecord = serde_json::from_str(&raw).context("parsing evidence record JSON")?;
	let path = ledger::append(ledger_dir, &record)?;
	println!("appended {}", path.display());
	Ok(())
}

fn run_render(ledger_dir: &std::path::Path, concepts_path: &std::path::Path, out_path: &std::path::Path) -> Result<()> {
	let raw = fs::read_to_string(concepts_path).with_context(|| format!("reading {}", concepts_path.display()))?;
	let concepts: Vec<Concept> = serde_json::from_str(&raw).context("parsing concepts JSON")?;
	let records = ledger::read_all(ledger_dir)?;
	let snap = render::snapshot(&concepts, &records);
	let json = serde_json::to_string_pretty(&snap).context("serializing competency snapshot")?;
	fs::write(out_path, json).with_context(|| format!("writing {}", out_path.display()))?;
	println!("wrote {}", out_path.display());
	Ok(())
}
