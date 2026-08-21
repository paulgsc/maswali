# maswali

*maswali* (Swahili: "questions") holds a maintainer-competency harness — not a
tutoring tool, a repository-grounded evidence system.

## What this is

Software increasingly gets substantially produced through agentic coding. That
raises a real question for whoever is named the principal maintainer of such a
repository: are they still a load-bearing reasoning component of the system, or
just downstream of whatever the agent emits?

This repo builds the instrument that answers that question with evidence
instead of assertion. It generates repository-grounded challenges against a
target codebase — currently [`paulgsc/server`](https://github.com/paulgsc/server)
— derives them from the actual source, invariants, tests, and deployment
assumptions, and keeps an append-only, revision-linked ledger of what the
maintainer independently demonstrated and what they didn't.

It does not ask "did you write this code." It asks whether the maintainer can
predict, interrogate, reject, repair, and evolve the system with the
implementation agent removed from the loop — on both directions of that claim:
whether a proposed change should be rejected, and whether a design's
simplifications remain valid as the deployment's assumptions are perturbed.

The full design rationale lives in [`docs/maintainer-assay.md`](docs/maintainer-assay.md).

## Status

Scaffolding in progress. The harness crate, ledger schema, and weekly cadence
are being built out; see open PRs and `docs/maintainer-assay.md` for the
current state of the design.

## Why this repo

This used to be a loose collection of LeetCode solutions, DSA exercises, and
tutorial follow-alongs — useful in the moment, not worth carrying forward as
the repo's identity. That content is retired; this repo's purpose going
forward is the harness above.
