# Contributing to Metten

Metten is in early development. The current foundation is non-trading; paper and live trading are not available. Read the [README](README.md) for setup and the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for intended behavior.

## Find work

Check [open issues](https://github.com/moultonlavah/metten/issues) and their blockers before starting. Discuss substantial changes in an issue first. The [implementation plan](docs/implementation-plan.md) describes the planned sequence, and [ADR 0001](docs/adr/0001-rust-engine-and-tauri-boundaries.md) explains the engine and desktop boundary.

## Set up and check changes

Use macOS, Node.js 24 or later, Apple's Xcode Command Line Tools, and the Rust toolchain pinned in `rust-toolchain.toml`. Run `npm ci` and `npm run tauri dev` to start the local foundation. The README explains its test session and isolated workspace.

Run checks relevant to your change. The current automated workflow runs these on macOS:

```sh
python3 tests/credential-runner.test.py
python3 tests/check-staged.test.py
npm run lint
npm run check
npm run build
cargo fmt --all --check
cargo test --workspace --locked
cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run tauri build -- --bundles app
```

Describe what you changed, link the issue, and report checks and any limitations in your pull request. Do not present paper or simulated results as proof of live behavior.

## Protect users and their data

Use controlled inputs and dummy credentials for tests. Never commit secrets, private runtime records, or account data. An issue or contribution does not authorize access to real credentials or accounts, paid services, deployment, or real-money validation. Do not operate another trading application or import its code or strategy thresholds. See the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for the intended safety boundaries.

Report vulnerabilities privately using the instructions in [SECURITY.md](SECURITY.md). Do not post secrets or exploitable vulnerability details in a public issue.

## Maintain build-action pins

External actions in `.github/workflows/checks.yml` use full commit SHAs, with
upstream versions or branch names in comments. `.github/dependabot.yml` requests
monthly update proposals for the repository’s GitHub Actions. This
configuration takes effect on the default branch;
no proposal from it has yet been observed. Proposals require source review and
passing Foundation checks before merge; they are never automatic approval.

Connor Moulton (@moultonlavah) owns a manual review on the first of each month
for `dtolnay/rust-toolchain`'s `master`-derived pin, and checks that the other two
pins have received proposals or manually reviews them if automation is silent.
Dependabot branch-derived SHA updates are not relied on: its current updater
returns without updating when no version tag is available. See the
[issue #28 evidence](docs/verification/issue-28.md) for the source and limitation.

For each update, resolve the version/ref in the upstream repository, review the
old-to-new source diff and action entry points (including nested action refs),
and record the upstream ref, full SHA and review findings in the update issue or
PR. Change the SHA and readable comment together. Preserve Node 24, restricted
`contents: read` permissions, and every existing check, including the release
full-hour tests and packaged Mac build. Rust **1.100.0** stays fixed: a compiler
upgrade needs a separate issue and reassessment of the engine's
compiler-dependent resource proof. Run the complete Foundation checks workflow
on the exact proposed revision before accepting any pin update.

## Source integration checks

Use the published maintainability standard from freshly resolved default-branch guidance even in this older worktree. `npm run lint` runs pinned Oxlint correctness rules and type-aware `typescript/no-floating-promises` with zero warnings. `lint:fix` adds safe fixes to the same configuration; inspect any edits. Actual maintained paths include src/view.mjs, view.d.mts and all tests JavaScript; browser and Node globals are separate. Lint does not replace TypeScript, behavior tests or rejection handling at explicit-void sites.

Run `cargo test -p metten --locked` and the public engine source tests for changed native/source boundaries. Run `python3 tests/credential-runner.test.py` after shared bootstrap/runner changes. The isolated packaged commands and proof limits are in [scripts](scripts/README.md#isolated-sources-walkthrough-step-5). Calibration/qualification belongs to separately authorized step6; no test grants provider/account/ordinary credential access or rights. Reuse unchanged release-hour arithmetic evidence only within its named version/assumptions; the complete Foundation job retains that gate.
