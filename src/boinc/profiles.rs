//! Preset runtime profiles for common BOINC onboarding flows.
//!
//! A preset profile bundles the kinds of things a user typically reconfigures
//! right after installing `boincrs`:
//!
//! * which BOINC projects to attach (by template slug or explicit URL),
//! * the desired CPU/network/GPU run-modes.
//!
//! Profiles are persisted as a tiny, human-readable `key = value` text format
//! so we don't take a new TOML/JSON dependency. Any line beginning with `#`
//! is a comment. Unknown keys are rejected by [`parse_profile`] to keep typos
//! from silently disabling a preference.
//!
//! The parser is defensive: every invalid input yields a [`ProfileError`]
//! with a human-readable reason. That error is surfaced to the UI so the user
//! sees *why* a profile was rejected instead of a generic failure.

use std::path::Path;

use thiserror::Error;

use crate::boinc::models::RunMode;
use crate::boinc::templates::{self, TemplateError};

/// A single attach request within a profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachEntry {
    /// Canonical master URL (already resolved from a slug if applicable).
    pub url: String,
    /// Project authenticator / account key.
    pub account_key: String,
}

/// A persistent preset the user can re-apply at startup.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PresetProfile {
    /// User-chosen label, e.g. `desktop-workstation`.
    pub name: String,
    /// Optional CPU run-mode override applied after attach.
    pub run_mode: Option<RunMode>,
    /// Optional network activity mode override.
    pub network_mode: Option<RunMode>,
    /// Optional GPU run-mode override.
    pub gpu_mode: Option<RunMode>,
    /// Ordered list of projects to attach.
    pub attach: Vec<AttachEntry>,
}

/// Errors reported when loading or validating a profile.
#[derive(Debug, Error)]
pub enum ProfileError {
    /// Profile name was missing or empty after trimming.
    #[error("profile name must not be empty")]
    MissingName,
    /// Profile name contained characters that would break path / display use.
    #[error("invalid profile name {0:?}: only ASCII letters, digits, '-', '_' allowed")]
    InvalidName(
        /// The rejected name, echoed back verbatim so the user can see the typo.
        String,
    ),
    /// A line could not be parsed as `key = value` (after trimming/comments).
    #[error("line {line}: expected 'key = value', got {raw:?}")]
    MalformedLine {
        /// 1-based line number of the offending input.
        line: usize,
        /// Raw line text, preserved verbatim for error messages.
        raw: String,
    },
    /// Key was recognized but value was invalid for that key.
    #[error("line {line}: invalid value for {key}: {reason}")]
    InvalidValue {
        /// 1-based line number of the offending input.
        line: usize,
        /// Configuration key whose value could not be accepted.
        key: String,
        /// Human-readable explanation of why the value was rejected.
        reason: String,
    },
    /// Attach entry was missing either the project reference or account key.
    #[error("line {line}: attach entry must be 'slug_or_url|account_key'")]
    MalformedAttach {
        /// 1-based line number of the malformed `attach = ...` entry.
        line: usize,
    },
    /// Attach entry referenced a template slug / URL that could not be resolved.
    #[error("line {line}: {source}")]
    UnknownTemplate {
        /// 1-based line number of the offending `attach = ...` entry.
        line: usize,
        /// Underlying template-resolution error (kept in the chain via `#[source]`).
        #[source]
        source: TemplateError,
    },
    /// An unknown configuration key was supplied.
    #[error("line {line}: unknown profile key {key:?}")]
    UnknownKey {
        /// 1-based line number where the unknown key appeared.
        line: usize,
        /// The unrecognized key, lower-cased.
        key: String,
    },
    /// The profile parsed, but had neither attach entries nor mode overrides.
    #[error("profile {0:?} is empty: no attach entries or mode overrides")]
    EmptyProfile(
        /// The parsed profile name, echoed back for context.
        String,
    ),
    /// I/O failure while reading or writing a profile file.
    #[error("profile I/O error at {path:?}: {source}")]
    Io {
        /// Filesystem path the I/O was attempted against.
        path: String,
        /// Underlying `std::io::Error` preserved in the error chain.
        #[source]
        source: std::io::Error,
    },
}

impl PresetProfile {
    /// Builds an empty profile with just a validated name.
    pub fn new(name: impl Into<String>) -> Result<Self, ProfileError> {
        let name = name.into();
        validate_profile_name(&name)?;
        Ok(Self {
            name,
            ..Self::default()
        })
    }

    /// Serializes the profile back to the `key = value` text format.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# boincrs profile: {}\n", self.name));
        out.push_str(&format!("name = {}\n", self.name));
        if let Some(mode) = self.run_mode {
            out.push_str(&format!("run_mode = {}\n", mode.as_boinc_tag()));
        }
        if let Some(mode) = self.network_mode {
            out.push_str(&format!("network_mode = {}\n", mode.as_boinc_tag()));
        }
        if let Some(mode) = self.gpu_mode {
            out.push_str(&format!("gpu_mode = {}\n", mode.as_boinc_tag()));
        }
        for entry in &self.attach {
            out.push_str(&format!("attach = {}|{}\n", entry.url, entry.account_key));
        }
        out
    }
}

/// Parses a preset profile from text.
///
/// Recognized keys:
/// * `name` — profile name (required, letters/digits/`-`/`_`).
/// * `run_mode`, `network_mode`, `gpu_mode` — one of `always`, `auto`, `never`.
/// * `attach` — repeated, formatted as `slug_or_url|account_key`.
///
/// # Examples
///
/// ```
/// use boincrs::boinc::profiles::parse_profile;
///
/// let p = parse_profile("
///     name = desktop
///     run_mode = always
///     attach = primegrid|abc123
/// ").expect("valid profile");
/// assert_eq!(p.name, "desktop");
/// assert_eq!(p.attach.len(), 1);
/// assert_eq!(p.attach[0].url, "https://www.primegrid.com/");
/// ```
pub fn parse_profile(text: &str) -> Result<PresetProfile, ProfileError> {
    let mut profile = PresetProfile::default();

    for (idx, raw_line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let line = strip_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(ProfileError::MalformedLine {
                line: line_no,
                raw: raw_line.to_string(),
            });
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();

        match key.as_str() {
            "name" => {
                validate_profile_name(value)?;
                profile.name = value.to_string();
            }
            "run_mode" => profile.run_mode = Some(parse_run_mode(value, line_no, &key)?),
            "network_mode" => profile.network_mode = Some(parse_run_mode(value, line_no, &key)?),
            "gpu_mode" => profile.gpu_mode = Some(parse_run_mode(value, line_no, &key)?),
            "attach" => profile.attach.push(parse_attach(value, line_no)?),
            other => {
                return Err(ProfileError::UnknownKey {
                    line: line_no,
                    key: other.to_string(),
                });
            }
        }
    }

    if profile.name.is_empty() {
        return Err(ProfileError::MissingName);
    }
    if profile.attach.is_empty()
        && profile.run_mode.is_none()
        && profile.network_mode.is_none()
        && profile.gpu_mode.is_none()
    {
        return Err(ProfileError::EmptyProfile(profile.name));
    }

    Ok(profile)
}

/// Reads and parses a profile from `path`.
pub fn load_profile(path: impl AsRef<Path>) -> Result<PresetProfile, ProfileError> {
    let path_ref = path.as_ref();
    let text = std::fs::read_to_string(path_ref).map_err(|e| ProfileError::Io {
        path: path_ref.display().to_string(),
        source: e,
    })?;
    parse_profile(&text)
}

/// Serializes a profile and writes it to `path`.
pub fn save_profile(path: impl AsRef<Path>, profile: &PresetProfile) -> Result<(), ProfileError> {
    validate_profile_name(&profile.name)?;
    let path_ref = path.as_ref();
    std::fs::write(path_ref, profile.to_text()).map_err(|e| ProfileError::Io {
        path: path_ref.display().to_string(),
        source: e,
    })
}

/// Validates a profile name against the accepted character set.
///
/// Names must be non-empty after trimming and may only contain ASCII
/// letters, digits, `-`, and `_` so they are safe to use as file names
/// and to display in status lines.
fn validate_profile_name(name: &str) -> Result<(), ProfileError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(ProfileError::MissingName);
    }
    let ok = trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if !ok {
        return Err(ProfileError::InvalidName(trimmed.to_string()));
    }
    Ok(())
}

/// Parses a run-mode string (`always` / `auto` / `never`) case-insensitively.
///
/// The `line` and `key` parameters are only used to build a
/// [`ProfileError::InvalidValue`] pointing at the offending input, so
/// callers get actionable feedback rather than a generic "bad value" message.
fn parse_run_mode(value: &str, line: usize, key: &str) -> Result<RunMode, ProfileError> {
    match value.to_ascii_lowercase().as_str() {
        "always" => Ok(RunMode::Always),
        "auto" => Ok(RunMode::Auto),
        "never" => Ok(RunMode::Never),
        other => Err(ProfileError::InvalidValue {
            line,
            key: key.to_string(),
            reason: format!("expected always|auto|never, got {other:?}"),
        }),
    }
}

/// Parses an `attach = slug_or_url|account_key` value into an [`AttachEntry`].
///
/// Both sides of the `|` separator must be non-empty after trimming. The
/// left side is routed through [`templates::resolve_template`] so callers can
/// mix well-known slugs and raw URLs freely. On failure the returned error
/// carries the 1-based `line` number for user-facing messages.
fn parse_attach(value: &str, line: usize) -> Result<AttachEntry, ProfileError> {
    let (left, right) = value
        .split_once('|')
        .ok_or(ProfileError::MalformedAttach { line })?;
    let slug_or_url = left.trim();
    let key = right.trim();
    if slug_or_url.is_empty() || key.is_empty() {
        return Err(ProfileError::MalformedAttach { line });
    }
    let url = templates::resolve_template(slug_or_url)
        .map_err(|e| ProfileError::UnknownTemplate { line, source: e })?;
    Ok(AttachEntry {
        url,
        account_key: key.to_string(),
    })
}

/// Returns `line` with any trailing `# comment` removed.
///
/// The first `#` wins, so `attach = foo|bar # note` yields `attach = foo|bar `.
/// Lines without a `#` are returned unchanged.
fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        Some(i) => &line[..i],
        None => line,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_attach_profile() {
        let text = "
            # a profile
            name = home-desktop
            attach = primegrid|KEY_A
            attach = https://asteroidsathome.net/boinc/|KEY_B
        ";
        let p = parse_profile(text).expect("should parse");
        assert_eq!(p.name, "home-desktop");
        assert_eq!(p.attach.len(), 2);
        assert_eq!(p.attach[0].url, "https://www.primegrid.com/");
        assert_eq!(p.attach[0].account_key, "KEY_A");
        assert_eq!(p.attach[1].url, "https://asteroidsathome.net/boinc/");
    }

    #[test]
    fn parse_modes() {
        let text = "
            name = laptop
            run_mode = auto
            network_mode = never
            gpu_mode = always
        ";
        let p = parse_profile(text).expect("should parse");
        assert_eq!(p.run_mode, Some(RunMode::Auto));
        assert_eq!(p.network_mode, Some(RunMode::Never));
        assert_eq!(p.gpu_mode, Some(RunMode::Always));
    }

    #[test]
    fn rejects_missing_name() {
        let err = parse_profile("run_mode = auto").expect_err("no name");
        assert!(matches!(err, ProfileError::MissingName));
    }

    #[test]
    fn rejects_empty_profile() {
        let err = parse_profile("name = foo").expect_err("empty");
        match err {
            ProfileError::EmptyProfile(name) => assert_eq!(name, "foo"),
            other => panic!("expected EmptyProfile, got {other:?}"),
        }
    }

    #[test]
    fn rejects_invalid_run_mode_with_line_info() {
        let err = parse_profile("name=foo\nrun_mode = sometimes").expect_err("bad mode");
        match err {
            ProfileError::InvalidValue { line, key, .. } => {
                assert_eq!(line, 2);
                assert_eq!(key, "run_mode");
            }
            other => panic!("expected InvalidValue, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_key() {
        let err = parse_profile("name=foo\nbogus = 1").expect_err("unknown");
        match err {
            ProfileError::UnknownKey { line, key } => {
                assert_eq!(line, 2);
                assert_eq!(key, "bogus");
            }
            other => panic!("expected UnknownKey, got {other:?}"),
        }
    }

    #[test]
    fn rejects_malformed_attach() {
        let err = parse_profile("name=foo\nattach = primegrid").expect_err("missing key");
        match err {
            ProfileError::MalformedAttach { line } => assert_eq!(line, 2),
            other => panic!("expected MalformedAttach, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_template_in_attach() {
        let err = parse_profile("name=foo\nattach = not-real|KEY").expect_err("unknown template");
        match err {
            ProfileError::UnknownTemplate { line, .. } => assert_eq!(line, 2),
            other => panic!("expected UnknownTemplate, got {other:?}"),
        }
    }

    #[test]
    fn rejects_invalid_name() {
        let err = PresetProfile::new("bad name!").expect_err("invalid");
        assert!(matches!(err, ProfileError::InvalidName(_)));
    }

    #[test]
    fn round_trips_through_text() {
        let mut p = PresetProfile::new("round-trip").expect("valid");
        p.run_mode = Some(RunMode::Always);
        p.gpu_mode = Some(RunMode::Never);
        p.attach.push(AttachEntry {
            url: "https://www.primegrid.com/".to_string(),
            account_key: "KEY".to_string(),
        });
        let text = p.to_text();
        let parsed = parse_profile(&text).expect("round-trip parse");
        assert_eq!(parsed, p);
    }
}
