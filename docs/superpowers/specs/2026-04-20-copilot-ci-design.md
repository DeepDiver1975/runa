# Copilot & CI Design — Runa

Date: 2026-04-20

Summary
- Formalize Copilot instructions and CI into a consistent spec. Include release pipeline for platform binaries uploaded to GitHub Releases (draft + manual publish).

Scope
- Update .github/copilot-instructions.md with concrete commands and CI guidance (done).
- Add a pinned GitHub Actions workflow to run fmt, clippy, build, test, and to prepare release artifacts.
- Add Dependabot config for actions and cargo.
- Add additional workflow steps to package per-platform binaries, generate checksums, and create a draft GitHub Release with assets.

Architecture
- Build matrix across runners: ubuntu-latest, macos-latest, windows-latest.
- For each matrix entry: cargo build --release, run tests, collect binary artifact.
- Packaging: zip/tar the binary with a small manifest.json including: commit, tag (if present), sha256 checksum, build timestamp, platform.
- Checksums: generate SHA256 for each artifact and a top-level checksums.txt listing all artifacts and checksums.
- Release: Create draft release (tag handling: if a tag push started the workflow, use that tag; otherwise create a semver draft tag e.g., vX.Y.Z-draft-<sha>) and upload assets.
- Publish: Manual approval step (workflow_dispatch) or a separate `publish-release` workflow that requires a maintainer to run to publish the draft.

Jobs & Steps (high-level)
1. Prepare
   - Checkout (pinned)
   - Setup Rust toolchain (stable)
   - Cache cargo
2. Build & Test (matrix)
   - cargo build --release
   - cargo test --no-fail-fast
   - Upload per-job artifact (binary)
3. Package & Checksum
   - Download artifacts
   - Package per-platform archives
   - Generate checksums and manifest
   - Upload package artifacts
4. Create Draft Release
   - Create or update a draft GitHub Release for the current tag/commit
   - Upload assets from packaging step
5. Publish (manual)
   - Maintainer triggers publish workflow (or uses GitHub UI) which publishes the draft and optionally creates a GitHub Release note.

Secrets and Tokens
- GITHUB_TOKEN: used for typical actions; ensure release creation uploads use a token with repo scope when necessary (the built-in GITHUB_TOKEN can create releases but cannot create release notes for forks in PRs). For cross-repo publishing, use a PAT stored in repo secrets as PUBLISH_TOKEN.

Artifacts retention & signing
- Artifacts attached to releases are long-lived; set retention policy per organization.
- Optionally sign release artifacts (GPG/Minisign) — out of scope for initial spec; can be added later.

Testing & Validation
- Add a `release-smoke` job that downloads the produced artifacts from the draft release and runs a minimal smoke check (binary --version, run a small built-in self-check).

Docs & Copilot guidance
- Update .github/copilot-instructions.md to include:
  - Explicit commands to run packaging and release-local (scripts to build and package locally)
  - How to run a single test and how to reproduce CI locally
  - Notes on the pinned actions and dependabot configuration

Implementation notes
- Use `actions/checkout`, `actions/cache`, and `actions-rs/toolchain` pinned to SHAs (already added in workflow). Also pin `actions/create-release` and `actions/upload-release-asset` when used; record the exact tag commit SHA in comments next to the pinned references in workflow files.
- For packaging across OSes, use `zip` on Windows and tar.gz on Unix where appropriate. Artifact naming convention: `runa-<platform>-<arch>-<version|commit>.zip` (or `.tar.gz`). Include a small `manifest.json` alongside each archive with fields: name, version (or tag), commit, platform, arch, rustc_version, build_timestamp, checksum_file.
- For reproducibility, record `rustc --version --verbose` into manifest.json and include build flags.
- Retention and signing: specify artifact retention in org settings or workflow `upload-artifact` retention. Optionally add GPG/Minisign signing step in the future.
- Release upload notes: use `actions/upload-release-asset` with the release draft created by `actions/create-release` (both pinned). When uploading, also create a top-level `checksums.txt` and upload it as a release asset.

Files to add or update
- .github/workflows/ci.yml (update to include matrix and packaging steps)
- .github/workflows/publish.yml (manual publish workflow)
- .github/dependabot.yml (already added)
- .github/copilot-instructions.md (already updated)
- docs/superpowers/specs/2026-04-20-copilot-ci-design.md (this file)

Next steps
- Review this spec. On approval, generate the CI workflow updates and the publish workflow, plus local packaging scripts (scripts/package-release.sh) and update copilot-instructions.md with local packaging commands.

---

Spec self-review checklist
- No placeholders remain in the doc (all high-level steps explicit)
- Scope is focused on Copilot instructions + CI + Releases
- Ambiguities: specific semver/tagging strategy not fully prescriptive — decision deferred to maintainers
