# Copilot & CI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

Goal: Implement CI workflows and packaging + publish pipeline; add local packaging scripts and update copilot instructions.

Architecture: Build matrix, package platform binaries, create draft GitHub Release with assets, provide manual publish workflow and local packaging scripts.

Tech Stack: GitHub Actions, Rust (cargo), bash scripts, optional small Node helper for cross-platform zipping if needed.

---

### Task 1: Add packaging and publish workflows

Files:
- Create: `.github/workflows/publish.yml`
- Modify: `.github/workflows/ci.yml` (extend current file to add matrix and packaging steps)

- [ ] Step 1: Create `.github/workflows/publish.yml` with a manual `workflow_dispatch` that locates the latest draft release for the commit/tag and publishes it. Use pinned actions `actions/create-release` and `actions/upload-release-asset` SHAs comments. Example content below.

- [ ] Step 2: Edit `.github/workflows/ci.yml` to add matrix:
  - matrix: os: [ubuntu-latest, macos-latest, windows-latest]
  - build matrix entry: cargo build --release; cargo test
  - after matrix: package job that runs on ubuntu-latest, downloads artifacts, creates zips/tars, lists checksums, creates/updates draft release, uploads assets.


### Task 2: (Removed) Local packaging scripts

Local packaging scripts were intentionally removed from this plan. Packaging is handled in CI via the `package` job which downloads build artifacts from the matrix, produces per-platform archives (zip/tar.gz), generates checksums and manifests, and uploads release assets. Maintain local packaging helpers separately if needed; they are out of scope for this implementation plan.

### Task 3: Update copilot-instructions.md with packaging instructions

Files:
- Modify: `.github/copilot-instructions.md`

- [ ] Step 1: Add a "Local packaging" section with exact commands to build and package locally using `scripts/package-release.sh` and `scripts/package-release.ps1`.
- [ ] Step 2: Add a "Publishing" section explaining draft release flow and manual publish workflow.


### Task 4: Create release-smoke job and tests

Files:
- Modify: `.github/workflows/ci.yml` (add release-smoke job or separate workflow triggered by release creation)
- Create: `tests/release_smoke.rs` or a small script under `scripts/release_smoke.sh`

- [ ] Step 1: Add `release-smoke` job that runs after draft creation; downloads an artifact from the draft release and runs `runa --version` or equivalent.
- [ ] Step 2: Add a simple smoke test script that validates binary runs and prints version.


### Task 5: Commit messages and trailers

- [ ] Step 1: Ensure every git commit includes Co-authored-by trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`
- [ ] Step 2: Provide example commit commands in plan.


### Task 6: Tests and verification

- [ ] Step 1: Run CI workflows locally via `act` or `nektos/act` guidance (add recommendations in copilot-instructions).
- [ ] Step 2: Verify publish workflow publishes draft release when manual trigger invoked.


---

Runbook notes
- Secrets: PUBLISH_TOKEN in repo settings (if needed)
- Artifact naming and manifest conventions

Saved plan to docs/superpowers/plans/2026-04-20-copilot-ci-implementation-plan.md
