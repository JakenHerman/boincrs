//! Startup-time project attach and preset profile orchestration.
//!
//! The bootstrap layer reads user configuration from environment variables
//! (and, optionally, a preset profile file) and applies it against a live
//! BOINC RPC client. It is intentionally tolerant: malformed entries in a
//! multi-attach list are skipped with a reason, so one bad line does not
//! prevent the rest of the bootstrap from running.
//!
//! The same parsing helpers are exposed as `pub` so tests and future UI code
//! can validate inputs without requiring a live BOINC daemon.

use std::path::PathBuf;

use crate::boinc::api::write::BoincWriteApi;
use crate::boinc::profiles::{load_profile, PresetProfile, ProfileError};
use crate::boinc::rpc_client::BoincRpcClient;
use crate::boinc::templates::{self, TemplateError};
use crate::error::{AppError, AppResult};

/// One concrete attach request resolved against the templates registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachProject {
    /// Canonical BOINC master URL to pass to `project_attach`.
    pub url: String,
    /// Project authenticator / account key.
    pub account_key: String,
}

/// Summary of what the bootstrap step actually did. Used by callers that want
/// to surface a status line to the user (main loop, tests).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BootstrapReport {
    /// URLs that were successfully sent through `project_attach` + `project_update`.
    pub attached: Vec<String>,
    /// Entries that were skipped, paired with a human-readable reason
    /// (`(input, reason)`). Skips are non-fatal by design.
    pub skipped: Vec<(String, String)>,
    /// Name of the loaded preset profile, if any. `None` means no
    /// `BOINCRS_PROFILE_FILE` was provided.
    pub profile_name: Option<String>,
    /// `true` if the CPU run-mode override from the profile was applied.
    pub applied_run_mode: bool,
    /// `true` if the network-mode override from the profile was applied.
    pub applied_network_mode: bool,
    /// `true` if the GPU-mode override from the profile was applied.
    pub applied_gpu_mode: bool,
}

/// Applies any attach/profile configuration found in the environment.
///
/// Discovery order:
/// 1. `BOINCRS_PROFILE_FILE` — path to a preset profile file. All attach and
///    mode settings from the profile are applied.
/// 2. `BOINCRS_PRIMEGRID_ACCOUNT_KEY`, `BOINCRS_ASTEROIDS_ACCOUNT_KEY`
///    — original env vars, preserved for backwards compatibility.
/// 3. `BOINCRS_ATTACH_TEMPLATES` — semicolon-separated `slug|key` pairs,
///    e.g. `primegrid|abc;rosetta|def`.
/// 4. `BOINCRS_ATTACH_PROJECTS` — semicolon-separated `url|key` pairs (legacy).
pub async fn attach_projects_from_env(rpc: &mut BoincRpcClient) -> AppResult<BootstrapReport> {
    let env = EnvInputs::from_env();
    run_bootstrap(rpc, env).await
}

/// Applies a prepared set of [`EnvInputs`] against a live RPC client.
///
/// Extracted from [`attach_projects_from_env`] so tests can exercise the
/// discovery → dedupe → attach pipeline without mutating process env vars.
async fn run_bootstrap(rpc: &mut BoincRpcClient, inputs: EnvInputs) -> AppResult<BootstrapReport> {
    let mut report = BootstrapReport::default();
    let mut attach_list: Vec<AttachProject> = Vec::new();

    let profile = match inputs.profile_path.as_ref() {
        Some(path) => match load_profile(path) {
            Ok(p) => {
                report.profile_name = Some(p.name.clone());
                Some(p)
            }
            Err(e) => {
                return Err(AppError::Config(format!(
                    "failed to load profile {}: {e}",
                    path.display()
                )))
            }
        },
        None => None,
    };

    if let Some(ref p) = profile {
        for entry in &p.attach {
            attach_list.push(AttachProject {
                url: entry.url.clone(),
                account_key: entry.account_key.clone(),
            });
        }
    }

    for (slug, key) in &inputs.well_known {
        match templates::resolve_template(slug) {
            Ok(url) => attach_list.push(AttachProject {
                url,
                account_key: key.clone(),
            }),
            Err(e) => report.skipped.push((slug.clone(), e.to_string())),
        }
    }

    for raw in &inputs.template_pairs {
        match parse_template_pair(raw) {
            Ok(ap) => attach_list.push(ap),
            Err(e) => report.skipped.push((raw.clone(), e)),
        }
    }

    for raw in &inputs.legacy_url_pairs {
        match parse_url_pair(raw) {
            Ok(ap) => attach_list.push(ap),
            Err(e) => report.skipped.push((raw.clone(), e)),
        }
    }

    dedupe_by_url(&mut attach_list);

    let mut write_api = BoincWriteApi::new(rpc);
    for project in &attach_list {
        let _ = write_api
            .project_attach(project.url.as_str(), project.account_key.as_str())
            .await?;
        let _ = write_api.project_update(project.url.as_str()).await?;
        report.attached.push(project.url.clone());
    }

    if let Some(p) = profile.as_ref() {
        apply_profile_modes(&mut write_api, p, &mut report).await?;
    }

    Ok(report)
}

/// Applies any run-mode overrides declared by `profile`.
///
/// Each `Some(mode)` is forwarded to the corresponding BOINC `set_*_mode`
/// RPC with a `duration_secs` of 0 (i.e. "permanent"). The supplied `report`
/// is updated so callers can tell the user which modes were actually changed.
async fn apply_profile_modes(
    write_api: &mut BoincWriteApi<'_>,
    profile: &PresetProfile,
    report: &mut BootstrapReport,
) -> AppResult<()> {
    if let Some(mode) = profile.run_mode {
        let _ = write_api.set_run_mode(mode, 0).await?;
        report.applied_run_mode = true;
    }
    if let Some(mode) = profile.network_mode {
        let _ = write_api.set_network_mode(mode, 0).await?;
        report.applied_network_mode = true;
    }
    if let Some(mode) = profile.gpu_mode {
        let _ = write_api.set_gpu_mode(mode, 0).await?;
        report.applied_gpu_mode = true;
    }
    Ok(())
}

/// Collected configuration discovered from process environment variables.
///
/// This is the sole structured input to [`run_bootstrap`]; tests build
/// `EnvInputs` directly rather than mutating global env state.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct EnvInputs {
    /// Path to a preset profile to load, if any.
    profile_path: Option<PathBuf>,
    /// (slug, account_key) from dedicated env vars (`BOINCRS_<SLUG>_ACCOUNT_KEY`).
    well_known: Vec<(String, String)>,
    /// Raw `slug|key[;…]` values from `BOINCRS_ATTACH_TEMPLATES`.
    template_pairs: Vec<String>,
    /// Raw `url|key[;…]` values from `BOINCRS_ATTACH_PROJECTS` (legacy).
    legacy_url_pairs: Vec<String>,
}

impl EnvInputs {
    /// Populates `EnvInputs` by reading the documented `BOINCRS_*` env vars.
    ///
    /// Missing or empty variables are simply left unset; nothing about the
    /// return value is fatal on its own — downstream logic decides whether
    /// an empty configuration is acceptable.
    fn from_env() -> Self {
        let mut out = Self::default();
        if let Some(path) = read_env_trimmed("BOINCRS_PROFILE_FILE") {
            out.profile_path = Some(PathBuf::from(path));
        }
        if let Some(key) = read_env_trimmed("BOINCRS_PRIMEGRID_ACCOUNT_KEY") {
            out.well_known.push(("primegrid".to_string(), key));
        }
        if let Some(key) = read_env_trimmed("BOINCRS_ASTEROIDS_ACCOUNT_KEY") {
            out.well_known.push(("asteroids".to_string(), key));
        }
        if let Some(spec) = read_env_trimmed("BOINCRS_ATTACH_TEMPLATES") {
            out.template_pairs = split_pairs(&spec);
        }
        if let Some(spec) = read_env_trimmed("BOINCRS_ATTACH_PROJECTS") {
            out.legacy_url_pairs = split_pairs(&spec);
        }
        out
    }
}

/// Splits a `a|1;b|2; ;c|3` style specification into trimmed, non-empty entries.
///
/// Whitespace around entries is stripped, and empty segments (produced by
/// trailing or doubled `;`) are dropped, so tolerant hand-edited env values
/// still parse cleanly.
fn split_pairs(spec: &str) -> Vec<String> {
    spec.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parses `slug_or_url|account_key` against the template registry.
pub fn parse_template_pair(pair: &str) -> Result<AttachProject, String> {
    let (left, right) = pair
        .split_once('|')
        .ok_or_else(|| "expected 'slug|account_key'".to_string())?;
    let slug = left.trim();
    let key = right.trim();
    if slug.is_empty() || key.is_empty() {
        return Err("slug and account_key must be non-empty".to_string());
    }
    let url = templates::resolve_template(slug).map_err(|e| match e {
        TemplateError::UnknownSlug(_, known) => {
            format!("unknown template {slug:?}. Known: {known}")
        }
        other => other.to_string(),
    })?;
    Ok(AttachProject {
        url,
        account_key: key.to_string(),
    })
}

/// Parses `url|account_key` without template resolution (legacy).
pub fn parse_url_pair(pair: &str) -> Result<AttachProject, String> {
    let (left, right) = pair
        .split_once('|')
        .ok_or_else(|| "expected 'url|account_key'".to_string())?;
    let url = left.trim();
    let key = right.trim();
    templates::validate_project_url(url).map_err(|e| e.to_string())?;
    if key.is_empty() {
        return Err("account_key must be non-empty".to_string());
    }
    Ok(AttachProject {
        url: url.to_string(),
        account_key: key.to_string(),
    })
}

/// Removes duplicate attach entries, keeping the first occurrence of each URL.
///
/// Profile attachments take precedence over env-var shortcuts simply because
/// they are pushed into the list first. The stable ordering makes the
/// resulting behavior predictable for users.
fn dedupe_by_url(list: &mut Vec<AttachProject>) {
    let mut seen: Vec<String> = Vec::new();
    list.retain(|p| {
        if seen.iter().any(|u| u == &p.url) {
            false
        } else {
            seen.push(p.url.clone());
            true
        }
    });
}

/// Reads an environment variable and returns `Some(trimmed_value)` only when
/// the variable is set and not empty after trimming whitespace.
fn read_env_trimmed(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Surfaces a profile-load error as an `AppError::Config`.
impl From<ProfileError> for AppError {
    fn from(value: ProfileError) -> Self {
        AppError::Config(value.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_template_pair_resolves_slug() {
        let ap = parse_template_pair("primegrid|ABC").expect("valid");
        assert_eq!(ap.url, "https://www.primegrid.com/");
        assert_eq!(ap.account_key, "ABC");
    }

    #[test]
    fn parse_template_pair_rejects_unknown_slug_with_hint() {
        let err = parse_template_pair("not-a-project|KEY").expect_err("unknown");
        assert!(err.contains("Known:"));
    }

    #[test]
    fn parse_template_pair_rejects_missing_parts() {
        assert!(parse_template_pair("primegrid").is_err());
        assert!(parse_template_pair("|KEY").is_err());
        assert!(parse_template_pair("primegrid|").is_err());
    }

    #[test]
    fn parse_url_pair_validates_url() {
        let ap = parse_url_pair("https://example.org/boinc/|KEY").expect("valid");
        assert_eq!(ap.url, "https://example.org/boinc/");
        assert!(parse_url_pair("not-a-url|KEY").is_err());
        assert!(parse_url_pair("https://example.org/|").is_err());
    }

    #[test]
    fn split_pairs_trims_and_ignores_empty() {
        let v = split_pairs("a|1 ; ; b|2 ;");
        assert_eq!(v, vec!["a|1".to_string(), "b|2".to_string()]);
    }

    #[test]
    fn dedupe_by_url_keeps_first_occurrence() {
        let mut list = vec![
            AttachProject {
                url: "u1".to_string(),
                account_key: "k1".to_string(),
            },
            AttachProject {
                url: "u1".to_string(),
                account_key: "k2".to_string(),
            },
            AttachProject {
                url: "u2".to_string(),
                account_key: "k3".to_string(),
            },
        ];
        dedupe_by_url(&mut list);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].account_key, "k1");
        assert_eq!(list[1].url, "u2");
    }
}
