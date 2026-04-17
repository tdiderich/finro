# Security Policy

## Reporting a vulnerability

**Please do not open a public issue for security reports.** Use GitHub's private disclosure channel:

- Go to https://github.com/tdiderich/finro/security/advisories/new
- Describe the issue, reproduction steps, and impact

Expect an initial response within 7 days. We'll coordinate disclosure with you before the fix and advisory go public.

## Scope

**In scope:**
- `finro` CLI source code (`src/**`)
- Build/test infrastructure (`.github/workflows/**`)
- Default theme CSS, bundled scripts, and scaffolded templates in the release binary

**Out of scope:**
- Content authored by end users in their own `.yaml` files. finro renders user-provided markdown/HTML — if you inject a `<script>` into your own site via `type: markdown`, that's your site, not a finro bug.
- Bugs in transitive Cargo dependencies. Report those upstream; we'll track the advisory via `cargo-audit` and bump once a patched version is available.

## Supply-chain protections

`cargo install --git https://github.com/tdiderich/finro` fetches and compiles whatever is on `main` at the moment you run it. That means a malicious commit to `main` would ship to every installer. The repo protects against that with:

### Repo-level (GitHub)

- **Branch protection on `main`** — no force-pushes, no deletion, no direct pushes. All changes land via PR.
- **Required PR review** from a CODEOWNER (see `CODEOWNERS`) before merge.
- **Required status checks** — PRs must pass CI (`cargo test`, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo-audit`) before merge.
- **Limited write access.** Only maintainers listed in `CODEOWNERS` can push to `main` or approve PRs for merge.
- **Signed commits encouraged for maintainers.** Tags on releases are signed.
- **Dependabot** for weekly Cargo dep updates — security advisories land as PRs you can review and merge.

### Dependency hygiene

- `Cargo.lock` is committed, so every build from a given commit resolves to the exact same transitive dep graph.
- `cargo-audit` runs in CI. A new RustSec advisory against any transitive dep fails the build.
- **New dependencies require justification in the PR.** finro's ~10 direct crates are deliberate; a PR that adds a crate for a minor convenience will typically be pushed back.
- **No build scripts that reach the network.** Dependencies that do will be rejected or vendored.

### What contributors should not do

These will get a PR closed immediately and may be reported:

- Bundle a binary blob (e.g. a pre-compiled font, a minified JS library) without source and a verifiable build recipe.
- Add a dependency from a source other than crates.io without discussion.
- Introduce code that makes network calls at build time or in scaffolded templates.
- Obfuscate any part of a diff — encoded strings, base64 blobs in source, unusual whitespace. Every line in a PR should be readable.
- Modify `.github/workflows/**` to weaken CI gates (remove a required check, skip tests conditionally, etc.) without explicit discussion.

### What users can do

- Pin a specific commit: `cargo install --git https://github.com/tdiderich/finro --rev <sha>`. That locks you to a version you can audit.
- Check releases (when we start tagging them) for signed tags before installing.
- Run `cargo audit` against your own Cargo lockfile if you embed finro into a larger workflow.

## Responsible disclosure

We appreciate reports via the private channel above. Public reproducers on Twitter / Mastodon / etc. before a fix is out will not speed things up — the reverse.
