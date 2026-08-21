# maswali

*maswali* (Swahili: "questions") is a maintainer-competency harness — not a
tutoring tool, a repository-grounded evidence system.

## The question this answers

Software increasingly gets substantially produced through agentic coding.
That raises a real question for whoever is named the principal maintainer of
such a repository: are they still a load-bearing reasoning component of the
system, or just downstream of whatever the agent emits?

This repo builds the instrument that answers that with evidence instead of
assertion — not "did you write this code" (increasingly meaningless — writing
code is close to a commodity now), but whether the maintainer can predict,
interrogate, reject, repair, and evolve the system with the implementation
agent taken out of the loop. Concretely, it generates challenges grounded in a
target codebase's actual source, invariants, tests, and deployment
assumptions, and keeps an append-only, revision-linked ledger of what got
independently demonstrated and what didn't.

**The one-sentence version:** *agents may supply implementations and
arguments; the maintainer must supply independent judgment* — and this repo
is the instrument that produces evidence of that, instead of a claim of it.

The full design rationale — why correctness is weaker evidence than
demonstrated reasoning, why probes come in two families (does the maintainer
reject a bad *change*, and does the maintainer notice when the *world* around
a design changes enough to invalidate it), and why over-engineering is graded
exactly as harshly as under-engineering — lives in
[`docs/maintainer-assay.md`](docs/maintainer-assay.md). Read that before
touching `crates/assay`; the code is a direct encoding of it, and half of it
won't make sense without the "why" the doc argues for at length.

## How it works, in one pass

```
paulgsc/server (the specimen)  ──read-only, pinned revision──▶  crates/assay (the harness, here)
                                                                        │
                                                    selects a concept, risk-weighted
                                                                        │
                                                                        ▼
                                            an agent writes challenge.md + rubric.md,
                                            confined to the concept's named source anchors
                                                                        │
                                                     maintainer responds; evaluated
                                                                        │
                                                                        ▼
                                              .assay/ledger/*.json  (append-only, the deliverable)
                                                                        │
                                                        crates/assay renders a JSON snapshot
                                                                        │
                                                                        ▼
                                        a route on the maintainer's own site (elsewhere, not this repo)
```

This repo never writes to `paulgsc/server`. It only reads it, at a pinned
revision, to ground challenges in real code. And this repo doesn't format a
report either — `crates/assay`'s job stops at a typed JSON snapshot
(`CompetencySnapshot`); presentation is a client's problem, deliberately kept
out of here so the harness can't quietly turn into a second, bespoke UI layer
nobody asked for.

## Repo layout

```
crates/assay/             the harness crate — the only code that matters here
  src/schema.rs             the vocabulary: Concept, ProbeFamily, HazardStatus,
                             ProvisioningStatus, EvidenceRecord, SelectionBrief
  src/ledger.rs              append-only read/write, refuses to overwrite a record
  src/selection.rs            risk-weighted, staleness-aware ranking (pure — no I/O)
  src/render.rs                ledger → CompetencySnapshot (JSON, not a report)
  src/main.rs                   CLI: `assay select|record|render`

.assay/                   working state (not code) — see .assay/README.md
docs/maintainer-assay.md  the design doc; read this first
.github/workflows/ci.yml  fmt + clippy + test on crates/assay
```

### CLI, roughly

```sh
# rank candidates, print the top pick as a SelectionBrief (JSON, stdout)
cargo run -p assay -- select --candidates candidates.json --revision <server-sha>

# append one evaluated probe to the ledger
cargo run -p assay -- record --ledger .assay/ledger --input record.json

# derive the CompetencySnapshot a client renders from
cargo run -p assay -- render --ledger .assay/ledger --concepts concepts.json --out snapshot.json
```

## Status

What exists, as of the PR that scaffolded it: `crates/assay`'s four modules
above, all tested (`cargo test -p assay`), clippy-clean under
`pedantic`+`all`, wired into CI. That's the deterministic half of the system —
schema, ledger, selection math, snapshot derivation — and it's real, not a
stub.

What does **not** exist yet, so don't assume it when picking this back up:

- No weekly job. Nothing currently calls `assay select`, generates a
  challenge, or opens a PR on a cadence. `candidates.json` and `concepts.json`
  are hand-fed inputs right now — the wiring that produces them from a live
  `git diff` / source scan of `paulgsc/server` doesn't exist.
- No agent-generation step. `challenge.md` / `rubric.md` authorship (confined
  to a `SelectionBrief`'s named anchors) is specified in the design doc but
  not implemented anywhere.
- No commit-before-critique protocol wired up (the mutation-family seal, or
  the environment-family "answer lands as a commit before the rubric is
  shown").
- No mutation-diff tooling for the implementation-perturbation probe family —
  nothing builds a mutant against `server`, runs its test suite, or checks
  survival.
- No consumer of `CompetencySnapshot`. The website route mentioned in "How it
  works" doesn't exist yet and lives in a different repo (`some-ui/apps/www`)
  when it does.

In short: the *type system and the ledger* for this idea are built and
trustworthy. The *loop that actually runs a challenge end to end* is not.
That's the next chunk of work, in roughly the order the bullets above are
listed.

## Why this repo, not `paulgsc/server`

This used to be a loose collection of LeetCode solutions, DSA exercises, and
tutorial follow-alongs — useful in the moment, not worth carrying forward as
the repo's identity. That content is retired (see the `chore: retire the
exercise graveyard` PR). The harness lives here rather than in `server`
because it's evidence *about* a maintainer's relationship to a codebase, not
part of the codebase — putting it in `server` would commingle live production
code with pedagogy tooling, which is exactly the thing `docs/maintainer-assay.md`
argues a maintainer needs to be able to keep straight.
