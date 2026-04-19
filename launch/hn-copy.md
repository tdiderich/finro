# Hacker News — Show HN

Drafts for the Show HN post. Pick one title; copy the body into the URL field's description box. Post from the submitter's account, expect to engage in comments for the first few hours (that's when the algorithm decides).

---

## Title options

Front-runners (ranked):

1. **Show HN: Kazam – I replaced my 1,722-line React site with 210 lines of YAML**
   - Personal, specific, has a number. HN loves this shape.
2. **Show HN: Kazam – YAML to themed static sites, one Rust binary, zero npm**
   - Tool-forward. More abstract but clear on what it is.
3. **Show HN: Kazam – Static sites designed to be written by LLMs**
   - The most differentiated hook. Risk: "designed by LLMs" triggers a subset of HN readers negatively.

I'd lead with #1. It's the story, not the pitch.

---

## Body

```
Hi HN — I built kazam after watching my Create-React-App personal site
turn into a maintenance chore for a thing that renders a dozen pages
once a year.

The rebuild is a handful of YAML files, zero npm dependencies, and
ships as static HTML and CSS. One Rust binary turns the YAML into the
site. No framework, no webpack, no runtime JS. `kazam dev` reloads on
save; `kazam build --release` emits minified output you drop on any
static host.

The interesting bit (for me at least): the YAML format is deliberately
boring — typed components, narrow schemas, no prose rules to interpret.
That's exactly what large language models produce correctly on the
first try. Every page of the docs site was written by Claude following
the bundled `AGENTS.md` authoring guide. I expect agents to be the
primary contributors to sites built with kazam going forward.

Why not Markdown or Mintlify or Next.js: Markdown renders prose, but
wastes what the web can actually do (cards, stats, interactive tables,
side-by-side layouts). Next.js / Mintlify / Docusaurus give you the
rich output but drag in a thousand-package npm tree and a client
runtime for a site that doesn't need either.

Docs + live examples + source: https://tdiderich.github.io/kazam/
Repo: https://github.com/tdiderich/kazam

Feedback on the format, the components, or the ethos very welcome.
Happy to dig into any of the tradeoffs.
```

---

## Anticipated comments + stub replies

### "Why YAML and not [TOML / JSON / S-exprs / custom DSL]?"

> YAML is what LLMs produce correctly on the first try — they generate
> it more reliably than TOML or custom DSLs. The tradeoff: YAML has
> sharp edges (YAML 1.1 vs 1.2, the Norway problem). kazam sidesteps
> those by using a strongly-typed deserializer (`serde_yaml`) that
> fails loud on ambiguous input.

### "Doesn't [Astro / Zola / Hugo / 11ty] already do this?"

> Astro and 11ty are great and have more features. kazam is narrower:
> no templates, no partials, no includes, no plugins. One format, one
> binary, no runtime. If you need a Svelte island in your page, use
> Astro. If you need a themed static page composed from typed blocks,
> try kazam.

### "No JS? What about search / interactive table / form?"

> Client-side table filter/sort is native to the table component (it's
> ~20 lines of vanilla JS that ships with the output). Everything else
> intentionally out of scope — kazam doesn't do forms, auth, or real-
> time. It's for static content.

### "How is this different from writing HTML?"

> You could write the HTML. You'd also redo the themed CSS, the
> print styling, the deck keyboard nav, the sortable table, and the
> live-reload dev server. And re-themed it every time you changed
> brand colors. kazam is the middle path — less expressive than raw
> HTML, much less work.
