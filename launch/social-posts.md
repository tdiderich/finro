# Social posts — launch

Short-form copy for Twitter/X, Bluesky, LinkedIn. Pair with an OG unfurl or attach the `docs/assets/og.svg` (converted to PNG if needed).

---

## Twitter / Bluesky (≤280 chars)

### Option A — the numbers

> I rebuilt my 5-year-old Create-React-App personal site as YAML.
>
> 1,722 → 210 lines. 1,000+ npm deps → 0.
>
> One Rust binary turns the YAML into a themed static site. No framework, no runtime JS, built so AI agents can author the whole thing.
>
> https://github.com/tdiderich/kazam

### Option B — the thesis

> Markdown renders plain prose. Next.js drags in 1,000+ npm packages. Neither is the right tool for most static sites.
>
> kazam is the middle path: typed YAML components, one Rust binary, zero runtime JS. Designed so LLMs write the content.
>
> https://github.com/tdiderich/kazam

### Option D — the weekend project

> Spent a weekend rewriting my React personal site as YAML.
>
> 88% less code. Zero npm deps. One Rust binary that turns YAML into themed static HTML.
>
> Now my landing page updates in 10 seconds instead of 10 minutes.
>
> https://github.com/tdiderich/kazam

### Option C — the dev-y one

> `cargo install --git https://github.com/tdiderich/kazam`
>
> `kazam init my-site && kazam dev my-site`
>
> Edit YAML. Save. Browser reloads. Static HTML out, zero npm, no framework.

---

## LinkedIn (slightly longer, less dev-slang)

> Spent the weekend rebuilding my personal site, which had grown into a small React app for what's ultimately a handful of pages describing who I am and what I've built.
>
> The new version is a dozen files of YAML. Zero npm dependencies. Ships as plain HTML and CSS. A single Rust binary turns the YAML into the site.
>
> I wrote the binary — it's called kazam — because the static-site space has calcified around two bad options: plain Markdown (wastes what the web can do) or framework-backed tools like Next.js and Mintlify (drag in a thousand-package npm tree for sites that don't need one).
>
> The format is deliberately shaped so LLMs produce it correctly on the first try. I expect agents to be the primary contributors going forward.
>
> Docs and examples: https://tdiderich.github.io/kazam/
> Repo: https://github.com/tdiderich/kazam

---

## Follow-up tweets (thread-style)

If the first lands, follow with:

> The receipts, since "I moved off React" is the most overused claim on this site:
>
> • Source LOC: −88%
> • npm deps: ~1,000 transitive → 0
> • Build tool: webpack → one Rust binary
> • Deploy: Firebase Hosting + 2 cloud functions → Firebase Hosting static

> Why YAML: it's the format LLMs generate correctly on the first try. Every page of the docs site was written by Claude following a bundled AGENTS.md guide.

> Why no JavaScript on the output: static sites shouldn't carry a framework-sized attack surface. The whole supply chain is ~10 direct Rust crates, Cargo.lock committed, cargo-audit in CI, branch-protected main.

---

## Hashtags (LinkedIn only; HN/Twitter skip)

`#opensource #rust #staticsites #developertools #AI`
