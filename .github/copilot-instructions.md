# Rust PR Review Instructions
CI overview:
* CI will build the project and run `cargo test` and `cargo clippy`.
* Feature combinations are checked with `cargo hack`.
* Do not comment on compile errors, compiler warnings, or clippy warnings.

Pay special attention to...
* code that uses async selection APIs such as `select`, `selectN`, `select_array`, `select_slice`, or is marked with a drop safety comment. These functions drop the futures that don't finish. Check that values are not lost when this happens.
* code that could possibly panic or is marked with a panic safety comment.

## Commit Messages
This repository uses [Conventional Commits](https://www.conventionalcommits.org/). This is **mandatory**, not stylistic: `release-plz` parses commit subjects to pick the next version number and to write `CHANGELOG.md`. A commit that ignores the convention lands in the changelog under "Other" and contributes nothing to the version decision.

Subject line:
```
<type>[optional scope][!]: <description>
```
- `<type>` is one of `feat`, `fix`, `perf`, `refactor`, `docs`, `test`, `build`, `ci`, `chore`, `style`, `revert`
- `<description>` is lowercase, imperative mood, and carries no trailing period (e.g. `fix: correct the I²C address table`, not `Fixed the table.`)
- Keep the whole subject to 72 characters or less
- Append `!` before the colon for a breaking change, or add a `BREAKING CHANGE:` footer

Body:
- Separate subject from body with a blank line
- Wrap body text at 72 characters
- Use the body to explain *what* and *why*, not *how*

One logical change per commit: a refactor and a feature go in separate commits even when pushed together.

The pull request title must follow the convention too. Squash-merging a pull request with more than one commit uses the **PR title** as the subject line, and that is the subject release-plz reads.

### Version bumps on `0.x`
Cargo's SemVer rules make the *minor* digit the breaking axis below `1.0`, so `feat:` does **not** bump the minor version while this crate is pre-1.0:

| Commit | Bump |
| --- | --- |
| `feat:`, `fix:`, `perf:`, `refactor:`, `docs:`, `chore:`, … | patch (`0.1.0` → `0.1.1`) |
| `feat!:`, `refactor!:`, or any `BREAKING CHANGE:` footer | minor (`0.1.0` → `0.2.0`) |

`cargo-semver-checks` runs on the release pull request. If it finds an API break that no commit declared with `!`, release-plz promotes the bump anyway — but declare it yourself, so the changelog says so too.

Use `!` only when the change breaks something that has actually shipped in a tagged release. Renaming an item introduced *after* the last `vX.Y.Z` tag breaks no users; there are none to break. Check with `git log vLAST..HEAD -- <files>` before reaching for it.

## Releases
Releases are fully automated by `release-plz` (see `release-plz.toml` and `.github/workflows/release-plz.yml`):

1. Conventional commits merged to `main` cause release-plz to open or update a `chore: release` pull request that bumps the version and rewrites `CHANGELOG.md`.
2. Merging that pull request tags `vX.Y.Z`, creates the GitHub release, and publishes to crates.io through Trusted Publishing.

You **must not** edit `version` in `Cargo.toml`, write `CHANGELOG.md` entries by hand, or push tags. Those are release-plz's outputs, and hand-editing them desynchronises the tool from the registry.

## AI Attribution
Every commit that includes AI-generated or AI-assisted work **must** contain an `Assisted-by` trailer in the commit message:
```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```
Where:
- `AGENT_NAME` is the name of the AI tool or framework (e.g., `GitHub Copilot`)
- `MODEL_VERSION` is the specific model version used (e.g., `claude-opus-4.6`)
- `[TOOL1] [TOOL2]` are optional specialized analysis tools used (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`)
Basic development tools (git, cargo, editors) should not be listed.
AI agents **must** verify their own identity (agent name and model version) before composing the `Assisted-by` trailer — do not assume or hard-code a model name from a previous session.
AI agents **MUST NOT** add `Signed-off-by` tags. Only humans can certify the Developer Certificate of Origin.
