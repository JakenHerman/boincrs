import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import styles from './index.module.css';

function TerminalPreview() {
  // Column widths (monospace cells between border characters):
  //   col1 = 22, col2 = 22, col3 = 22
  // Frame width: `│` + 22 + `│` + 22 + `│` + 22 + `│` = 70 cells
  // Spans inside must not add horizontal padding (see .t-focus in custom.css).
  return (
    <div className="boincrs-terminal" aria-label="boincrs terminal preview">
      <div className="boincrs-terminal__chrome">
        <span />
        <span />
        <span />
        <strong>boincrs · 127.0.0.1:31416</strong>
      </div>
      <pre>
{`┌ Selected Task ─────────────────────────────────────────────────────┐
│ PrimeGrid · llrTRP · 68% · `}<span className="t-accent">[RUN]</span>{` · 01:12 / 00:34                   │
│ deadline 2026-04-28 14:20 · chkpt 01:08 · exit —                   │
└────────────────────────────────────────────────────────────────────┘
┌ `}<span className="t-focus">[focus]</span>{` Projects ────┬ Tasks ───────────────┬ Transfers ───────────┐
│ `}<span className="t-accent">&gt;&gt;</span>{` PrimeGrid         │ READY TO REPORT      │ ↑ 12.3 MB   74%      │
│    Asteroids@home    │ RUNNING (by %)       │ ↓  4.1 MB   `}<span className="t-warn">retry</span>{`    │
│    World Community   │ WAITING / READY      │ ↓  2.8 MB   done     │
└──────────────────────┴──────────────────────┴──────────────────────┘
`}<span className="t-dim">j/k move · tab switch pane · r refresh · q quit</span>
      </pre>
    </div>
  );
}

function Hero() {
  return (
    <header className={styles.hero}>
      <div className={clsx('container', styles.heroInner)}>
        <div>
          <span className={styles.badge}>Beta · BOINC GUI RPC</span>
          <h1 className={styles.title}>
            A terminal you can actually{' '}
            <span className={styles.titleMono}>boinc</span> from.
          </h1>
          <p className={styles.subtitle}>
            <strong>boincrs</strong> is a fast, keyboard-first Rust TUI for your
            local BOINC client. Attach projects, track tasks, monitor transfers,
            and toggle run modes — without leaving the terminal.
          </p>
          <div className={styles.ctas}>
            <Link
              className={clsx('button button--lg', styles.ctaPrimary)}
              to="/guide/getting-started">
              Get started →
            </Link>
            <Link
              className={clsx('button button--lg', styles.ctaGhost)}
              to="/guide/why-boincrs">
              Why boincrs?
            </Link>
            <Link
              className={clsx('button button--lg', styles.ctaGhost)}
              href="https://github.com/jakenherman/boincrs">
              View on GitHub
            </Link>
          </div>
          <div className={styles.metaRow}>
            <span><strong>Rust</strong> · ratatui · tokio</span>
            <span><strong>BOINC</strong> · 7.16 / 7.20 / 8.2</span>
            <span><strong>Targets</strong> · Linux · macOS · Windows</span>
            <span><strong>License</strong> · MIT</span>
          </div>
        </div>
        <TerminalPreview />
      </div>
    </header>
  );
}

const features = [
  {
    glyph: '01 / Local-first',
    title: 'Talks to your local BOINC',
    body: 'Connects to the GUI RPC endpoint on 127.0.0.1:31416 by default. No remote accounts, no cloud middleman — just your daemon.',
  },
  {
    glyph: '02 / Three panes',
    title: 'Projects · Tasks · Transfers',
    body: 'Task groups (READY TO REPORT → RUNNING → WAITING/READY) match how you actually think about BOINC workloads.',
  },
  {
    glyph: '03 / Auto-attach',
    title: 'PrimeGrid + Asteroids@home',
    body: 'Drop an account key into .env and boincrs attaches and updates the project on startup.',
  },
  {
    glyph: '04 / Accessible',
    title: 'No color-only UI',
    body: 'State is always labeled in text — [RUN], [REPORT], [ACTIVE] — and NO_COLOR=1 keeps the semantics intact.',
  },
  {
    glyph: '05 / Resilient',
    title: 'Reconnect with backoff',
    body: 'Transient RPC failures trigger bounded exponential backoff (1 s → 30 s, ±25% jitter) instead of crashing.',
  },
  {
    glyph: '06 / Typed errors',
    title: 'No .unwrap() in src/**',
    body: 'Errors flow through AppResult / AppError with thiserror. CI refuses new unwrap or expect in production code.',
  },
];

function Features() {
  return (
    <section className={styles.features}>
      <div className="container">
        <h2 className={styles.sectionTitle}>Why the terminal?</h2>
        <p className={styles.sectionLede}>
          BOINC already runs as a background daemon on your machine. A TUI is
          the shortest path between "what is my client doing?" and an answer —
          readable over SSH, fast on old hardware, and friendly to keyboards
          and screen readers alike.
        </p>
        <div className={styles.featureGrid}>
          {features.map((f) => (
            <div className={styles.featureCard} key={f.title}>
              <div className={styles.featureGlyph}>{f.glyph}</div>
              <h3 className={styles.featureTitle}>{f.title}</h3>
              <p className={styles.featureBody}>{f.body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function Explain() {
  return (
    <section className={styles.explain}>
      <div className="container">
        <div className={styles.explainGrid}>
          <div className={styles.explainCard}>
            <h3>What you get</h3>
            <ul>
              <li>Multi-pane TUI for projects, tasks, and transfers.</li>
              <li>Selected-task header with progress, deadline, checkpoint, and exit status.</li>
              <li>Keyboard navigation (<code>tab</code>, <code>j/k</code>, arrows) across every pane.</li>
              <li>Action keys for project/task/transfer operations and client run modes.</li>
              <li>Diagnostics bundle export (<code>D</code>) for bug reports.</li>
              <li>BOINC compatibility matrix validated in CI: 7.16.x, 7.20.x, 8.2.x.</li>
            </ul>
          </div>
          <div className={styles.explainCard}>
            <h3>What it is not</h3>
            <ul>
              <li>Not a remote BOINC dashboard — local GUI RPC only, for now.</li>
              <li>Not a project chooser UI — you still pick projects you trust.</li>
              <li>Not a replacement for BOINC Manager — it's a complement for keyboard workflows.</li>
              <li>Not a monitoring cloud — your account keys live in <code>.env</code> on your box.</li>
            </ul>
          </div>
        </div>
      </div>
    </section>
  );
}

function CallToAction() {
  return (
    <section className={styles.cta}>
      <div className="container">
        <h2>Ready to drive BOINC from the keyboard?</h2>
        <p>
          Clone the repo, point at your local daemon, and you'll be attaching
          projects in under five minutes.
        </p>
        <Link
          className={clsx('button button--lg', styles.ctaPrimary)}
          to="/guide/getting-started">
          Getting started →
        </Link>
      </div>
    </section>
  );
}

export default function Home() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description={siteConfig.tagline}>
      <Hero />
      <main>
        <Features />
        <Explain />
        <CallToAction />
      </main>
    </Layout>
  );
}
