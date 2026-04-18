# Skill notes — interviewing a person to build their kazam site

Raw friction log from rebuilding tylerdiderich.com. These are the decisions an agent had to make or ask about. They become the questions for a future `kazam:init-personal` skill (and siblings for landing / docs).

## Questions the interview should ask

**Site basics**
- What's the site for? (personal, landing page, docs, knowledge base) → picks template
- Site name / owner name?
- Dark, light, or auto mode?
- Favicon / logo? (skip if none — kazam default is fine)

**Nav structure**
- What pages do you want? (e.g. Home / Resume / Projects / Blogs)
- One of them is the home page — which?

**Per-page intent (for personal site)**
- Home: one-line intro, contact info (email / LinkedIn / location)?
- Resume: roles (title, company, years, one-line)? skills? about-me paragraph?
- Projects: manual list or "link to GitHub"?
- Blogs: pull from Medium/Substack (static link), or self-hosted markdown?

**Deploy**
- Firebase / GitHub Pages / Cloudflare Pages / Netlify / Vercel?
- If Firebase: existing project ID? (reuse `.firebaserc`)

**Data liveness (the sneaky one)**
- Any section pulling live data from an API today? (Old tylerdiderich.com had `/api/projects` + Medium RSS feed — both got dropped.) If yes: keep it live (skill scaffolds a fetcher that writes yaml), or freeze to static?

## Friction I hit during the build

1. **`kazam serve` vs `kazam dev`** — I guessed `serve`, real subcommand is `dev`. Skill should emit exact command.
2. **`kazam dev` crashed with "Permission denied" on a repo containing `.git`.** Root cause: build walked hidden dirs into `_site/`, second build failed to re-copy. Fixed in `src/build.rs` by filtering hidden entries in `WalkDir`. (This was the biggest friction — the skill should just work from a fresh `git init`ed directory.)
3. **Title doubling.** I wrote `title: Resume — Tyler Diderich` in resume.yaml, kazam concatenates `<page> — <site>`, so HTML `<title>` became `Resume — Tyler Diderich — Tyler Diderich`. Skill should explain: page title is just the page name.
4. **Home page title.** `title: Home` renders `Home — Tyler Diderich`. Fine but slightly redundant. Skill could suggest `title: <your name>` for the home page and no site-name concatenation — not currently possible without changing kazam. Minor.
5. **Firebase rewrites.** Old config had `**` → `index.html` SPA fallback. kazam sites don't need it (real HTML files). Skill should emit a minimal `firebase.json` without rewrites.
6. **`cleanUrls: true`** is a nice default for firebase — strips `.html` from URLs. Worth baking into the deploy template.

## Template differences (personal vs landing vs docs)

Shared:
- site name, mode, favicon, nav

Personal:
- prompts for: intro, role, contact, experience cards, skills, hobbies
- default pages: Home, Resume, Projects, Blogs

Landing:
- prompts for: headline, subhead, feature list, CTA, social proof, contact CTA
- default pages: one page (index)
- component bias: stat_grid, card_grid with badges, callouts, blockquote for testimonials

Docs:
- prompts for: product name, table of contents, code-heavy pages
- default pages: index, guide/*, reference/*
- component bias: code blocks, tabs for language variants, breadcrumb on every page

All three should emit an `AGENTS.md` so future edits stay on-style.
