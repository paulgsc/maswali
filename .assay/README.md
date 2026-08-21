# `.assay/`

Working state for the maintainer-competency harness, distinct from
`crates/assay` (the code that produces and reads this state).

- `challenge.md` — the current week's challenge. Rewritten weekly; not
  meant to be read historically. If it's missing, no challenge is open.
- `selection.json` — the `SelectionBrief` that `challenge.md` was generated
  from: which concept, which probe family, why, against which revision of
  `paulgsc/server`.
- `ledger/*.json` — append-only. One file per completed (or declined) probe.
  This is the actual deliverable; everything else here is scratch state.

See `docs/maintainer-assay.md` for what all of this means and why it's
shaped this way.
