# boincrs docs site

This directory is the [Docusaurus](https://docusaurus.io) site that builds the
public documentation at
[jakenherman.github.io/boincrs](https://jakenherman.github.io/boincrs).

- Landing page source: `src/pages/index.jsx`
- Guide pages (sidebar docs): `guide/**/*.md`
- Theme tokens: `src/css/custom.css`
- Static assets: `static/`

## Local development

```bash
cd docs
npm install
npm run start    # http://localhost:3000/boincrs/
```

## Production build

```bash
cd docs
npm install
npm run build    # output in docs/build
npm run serve    # preview the built site
```

## Deployment

`main` branch pushes that touch `docs/**`, any top-level `*.md`, or
`.github/workflows/docs.yml` trigger the `docs` GitHub Actions workflow, which
builds and publishes the site to GitHub Pages.

## What lives here vs. at the repo root

| File                               | Home                              |
| ---------------------------------- | --------------------------------- |
| Polished long-form docs            | `docs/guide/**`                   |
| Landing page                       | `docs/src/pages/index.jsx`        |
| Short repo entry + install summary | `README.md` (repo root)           |
| Contribution flow                  | `CONTRIBUTING.md` (repo root)     |
| Release notes                      | `CHANGELOG.md` (repo root)        |

The root-level Markdown files stay deliberately short — they point readers to
the full docs site for anything long-form.
