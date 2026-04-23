//! Curated project attach templates.
//!
//! The TUI needs to support attaching BOINC projects beyond the initial
//! PrimeGrid / Asteroids@home pair. To keep onboarding fast and consistent, we
//! ship a small registry of **well-known projects** with their canonical master
//! URLs and human-readable metadata. Callers supply a **slug** (e.g.
//! `primegrid`) together with an account key, and the bootstrap layer uses
//! [`resolve_template`] to recover the canonical URL.
//!
//! This module is intentionally self-contained — no I/O, no async — so it can
//! be validated with fast unit tests and reused from both the bootstrap path
//! and eventually the interactive attach UI.

use thiserror::Error;

/// A documented BOINC project that users can attach by slug.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectTemplate {
    /// Short kebab-case identifier used in config (`primegrid`).
    pub slug: &'static str,
    /// Display name shown to the user (`PrimeGrid`).
    pub name: &'static str,
    /// Canonical BOINC master URL used in `project_attach`.
    pub url: &'static str,
    /// One-line description, shown in docs / future pickers.
    pub summary: &'static str,
    /// Category tags (`math`, `biology`, …) used for future filtering.
    pub categories: &'static [&'static str],
}

/// Errors produced when resolving or validating a template input.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TemplateError {
    /// The provided slug / URL does not match any well-known template.
    #[error("unknown project template: {0:?}. Known slugs: {1}")]
    UnknownSlug(
        /// User-supplied slug or URL that failed to resolve.
        String,
        /// Comma-separated list of known slugs, included in the message as a hint.
        String,
    ),
    /// The input string was empty after trimming.
    #[error("project template reference was empty")]
    EmptyInput,
    /// A free-form URL was supplied but it is not a valid BOINC master URL.
    #[error("invalid project URL {0:?}: {1}")]
    InvalidUrl(
        /// Offending URL, echoed back for debugging.
        String,
        /// Short reason describing which rule the URL violated.
        &'static str,
    ),
}

/// Returns the curated registry of well-known BOINC projects.
///
/// Entries are kept alphabetical so the list is easy to scan in docs and CLI
/// help output.
pub fn all_templates() -> &'static [ProjectTemplate] {
    REGISTRY
}

/// Looks up a template by exact slug match (case-insensitive).
pub fn find_template(slug: &str) -> Option<&'static ProjectTemplate> {
    let needle = slug.trim().to_ascii_lowercase();
    REGISTRY.iter().find(|t| t.slug == needle)
}

/// Resolves a user-supplied project reference to a canonical master URL.
///
/// Accepts either:
/// * a **slug** from [`all_templates`] (e.g. `primegrid`), or
/// * a fully qualified **master URL** (e.g. `https://www.primegrid.com/`),
///   which is validated with [`validate_project_url`].
///
/// # Examples
///
/// ```
/// use boincrs::boinc::templates::resolve_template;
///
/// assert_eq!(
///     resolve_template("primegrid").expect("known slug"),
///     "https://www.primegrid.com/"
/// );
/// assert_eq!(
///     resolve_template("https://boinc.example.org/").expect("valid url"),
///     "https://boinc.example.org/"
/// );
/// ```
pub fn resolve_template(input: &str) -> Result<String, TemplateError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(TemplateError::EmptyInput);
    }

    if looks_like_url(trimmed) {
        validate_project_url(trimmed)?;
        return Ok(trimmed.to_string());
    }

    if let Some(t) = find_template(trimmed) {
        return Ok(t.url.to_string());
    }

    let known: Vec<&'static str> = REGISTRY.iter().map(|t| t.slug).collect();
    Err(TemplateError::UnknownSlug(
        trimmed.to_string(),
        known.join(", "),
    ))
}

/// Validates that `url` looks like a reasonable BOINC project master URL.
///
/// We deliberately do not perform DNS resolution here — the actual attach call
/// will surface protocol errors. The aim is to catch obvious typos early so
/// the RPC never sees them.
pub fn validate_project_url(url: &str) -> Result<(), TemplateError> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err(TemplateError::EmptyInput);
    }
    if !looks_like_url(trimmed) {
        return Err(TemplateError::InvalidUrl(
            trimmed.to_string(),
            "must start with http:// or https://",
        ));
    }
    if trimmed.contains(' ') || trimmed.contains('\t') {
        return Err(TemplateError::InvalidUrl(
            trimmed.to_string(),
            "must not contain whitespace",
        ));
    }
    let after_scheme = trimmed
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or("");
    let host = after_scheme.split('/').next().unwrap_or("");
    if host.is_empty() || !host.contains('.') {
        return Err(TemplateError::InvalidUrl(
            trimmed.to_string(),
            "host must contain a dot (e.g. example.org)",
        ));
    }
    Ok(())
}

/// Returns `true` when `s` starts with `http://` or `https://`.
///
/// A deliberately cheap check — we do not parse the URL here. Full validation
/// is performed by [`validate_project_url`] when the string is actually used
/// as a master URL.
fn looks_like_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Registry of curated templates. Keep alphabetical by slug.
const REGISTRY: &[ProjectTemplate] = &[
    ProjectTemplate {
        slug: "asteroids",
        name: "Asteroids@home",
        url: "https://asteroidsathome.net/boinc/",
        summary: "Determines shapes and spin states of asteroids.",
        categories: &["astronomy", "physics"],
    },
    ProjectTemplate {
        slug: "einstein",
        name: "Einstein@Home",
        url: "https://einsteinathome.org/",
        summary: "Searches for neutron stars using gravitational-wave and radio data.",
        categories: &["astronomy", "physics"],
    },
    ProjectTemplate {
        slug: "gpugrid",
        name: "GPUGRID",
        url: "https://www.gpugrid.net/",
        summary: "Molecular dynamics simulations accelerated on GPUs.",
        categories: &["biology", "gpu"],
    },
    ProjectTemplate {
        slug: "lhc",
        name: "LHC@home",
        url: "https://lhcathome.cern.ch/lhcathome/",
        summary: "Simulations in support of CERN's Large Hadron Collider.",
        categories: &["physics"],
    },
    ProjectTemplate {
        slug: "milkyway",
        name: "Milkyway@Home",
        url: "https://milkyway.cs.rpi.edu/milkyway/",
        summary: "Models the 3D structure of the Milky Way galaxy.",
        categories: &["astronomy"],
    },
    ProjectTemplate {
        slug: "primegrid",
        name: "PrimeGrid",
        url: "https://www.primegrid.com/",
        summary: "Searches for large prime numbers of mathematical interest.",
        categories: &["math", "prime-numbers"],
    },
    ProjectTemplate {
        slug: "rosetta",
        name: "Rosetta@home",
        url: "https://boinc.bakerlab.org/rosetta/",
        summary: "Protein structure prediction for disease research.",
        categories: &["biology", "medicine"],
    },
    ProjectTemplate {
        slug: "seti",
        name: "SETI-like (Cosmology@Home)",
        url: "https://www.cosmologyathome.org/",
        summary: "Cosmological model parameter estimation.",
        categories: &["astronomy", "cosmology"],
    },
    ProjectTemplate {
        slug: "worldcommunitygrid",
        name: "World Community Grid",
        url: "https://www.worldcommunitygrid.org/",
        summary: "Humanitarian research spanning climate, health, and disease.",
        categories: &["biology", "climate", "medicine"],
    },
    ProjectTemplate {
        slug: "yoyo",
        name: "yoyo@home",
        url: "https://www.rechenkraft.net/yoyo/",
        summary: "Umbrella project hosting several scientific subprojects.",
        categories: &["umbrella"],
    },
];

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn registry_slugs_are_unique_and_kebab_case() {
        let mut seen: Vec<&str> = Vec::new();
        for t in all_templates() {
            assert!(
                !seen.contains(&t.slug),
                "duplicate slug in registry: {}",
                t.slug
            );
            seen.push(t.slug);
            assert!(
                t.slug.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
                "slug must be kebab-case ascii: {}",
                t.slug
            );
            assert!(
                t.url.starts_with("https://") || t.url.starts_with("http://"),
                "template {} has non-http URL {}",
                t.slug,
                t.url
            );
        }
        assert!(seen.len() >= 8, "need a meaningful set of templates");
    }

    #[test]
    fn find_template_is_case_insensitive_and_trims() {
        assert!(find_template("primegrid").is_some());
        assert!(find_template("  PRIMEGRID  ").is_some());
        assert!(find_template("unknownproject").is_none());
    }

    #[test]
    fn resolve_template_returns_url_for_known_slug() {
        let url = resolve_template("asteroids").expect("known slug");
        assert_eq!(url, "https://asteroidsathome.net/boinc/");
    }

    #[test]
    fn resolve_template_accepts_valid_url_passthrough() {
        let url = resolve_template("https://boinc.example.org/project/").expect("valid url");
        assert_eq!(url, "https://boinc.example.org/project/");
    }

    #[test]
    fn resolve_template_rejects_empty_input() {
        assert!(matches!(
            resolve_template("   "),
            Err(TemplateError::EmptyInput)
        ));
    }

    #[test]
    fn resolve_template_reports_unknown_slug_with_hint() {
        let err = resolve_template("not-a-real-project").expect_err("unknown");
        match err {
            TemplateError::UnknownSlug(input, known) => {
                assert_eq!(input, "not-a-real-project");
                assert!(known.contains("primegrid"));
            }
            other => panic!("expected UnknownSlug, got {other:?}"),
        }
    }

    #[test]
    fn validate_project_url_requires_scheme_and_host() {
        assert!(validate_project_url("https://boinc.example.org/").is_ok());
        assert!(matches!(
            validate_project_url("boinc.example.org"),
            Err(TemplateError::InvalidUrl(_, _))
        ));
        assert!(matches!(
            validate_project_url("https://nodothost/"),
            Err(TemplateError::InvalidUrl(_, _))
        ));
        assert!(matches!(
            validate_project_url("https://has space.org/"),
            Err(TemplateError::InvalidUrl(_, _))
        ));
        assert!(matches!(
            validate_project_url(""),
            Err(TemplateError::EmptyInput)
        ));
    }
}
