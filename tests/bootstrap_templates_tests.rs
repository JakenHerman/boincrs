use async_trait::async_trait;
use boincrs::boinc::bootstrap::{parse_template_pair, parse_url_pair, AttachProject};
use boincrs::boinc::profiles::{parse_profile, AttachEntry, PresetProfile};
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::templates::{all_templates, find_template, resolve_template};
use boincrs::boinc::transport::BoincTransport;
use boincrs::error::AppResult;

struct CountingTransport {
    writes: Vec<Vec<u8>>,
}

impl CountingTransport {
    fn new() -> Self {
        Self { writes: Vec::new() }
    }
}

#[async_trait]
impl BoincTransport for CountingTransport {
    async fn send(&mut self, payload: &[u8]) -> AppResult<()> {
        self.writes.push(payload.to_vec());
        Ok(())
    }

    async fn receive(&mut self) -> AppResult<Vec<u8>> {
        Ok(b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03".to_vec())
    }
}

#[test]
fn registry_contains_primegrid_and_asteroids_at_minimum() {
    let slugs: Vec<&str> = all_templates().iter().map(|t| t.slug).collect();
    assert!(slugs.contains(&"primegrid"), "slugs: {slugs:?}");
    assert!(slugs.contains(&"asteroids"), "slugs: {slugs:?}");
}

#[test]
fn template_lookup_trims_and_normalizes_case() {
    let pg = find_template("  PrimeGrid ").expect("should find primegrid");
    assert_eq!(pg.url, "https://www.primegrid.com/");
}

#[test]
fn resolve_template_provides_clear_error_for_typo() {
    let err = resolve_template("primgrid").expect_err("typo should fail");
    let msg = err.to_string();
    assert!(msg.contains("unknown project template"), "msg: {msg}");
    assert!(
        msg.contains("primegrid"),
        "msg should hint known slugs: {msg}"
    );
}

#[test]
fn template_pair_round_trip_matches_profile_attach_entry() {
    let ap: AttachProject = parse_template_pair("rosetta|SECRET").expect("valid");
    assert_eq!(ap.url, "https://boinc.bakerlab.org/rosetta/");
    assert_eq!(ap.account_key, "SECRET");
}

#[test]
fn legacy_url_pair_rejects_obvious_typos() {
    assert!(parse_url_pair("htp://example.org/|KEY").is_err());
    assert!(parse_url_pair("https://example.org/|KEY").is_ok());
}

#[test]
fn profile_round_trip_persists_and_restores_preferences() {
    let mut profile = PresetProfile::new("test-profile").expect("valid name");
    profile.run_mode = Some(boincrs::boinc::models::RunMode::Always);
    profile.network_mode = Some(boincrs::boinc::models::RunMode::Auto);
    profile.gpu_mode = Some(boincrs::boinc::models::RunMode::Never);
    profile.attach.push(AttachEntry {
        url: "https://www.primegrid.com/".to_string(),
        account_key: "KEY1".to_string(),
    });
    profile.attach.push(AttachEntry {
        url: "https://asteroidsathome.net/boinc/".to_string(),
        account_key: "KEY2".to_string(),
    });

    let serialized = profile.to_text();
    assert!(serialized.contains("name = test-profile"));
    assert!(serialized.contains("run_mode = always"));
    assert!(serialized.contains("attach = https://www.primegrid.com/|KEY1"));

    let parsed = parse_profile(&serialized).expect("round-trip parse");
    assert_eq!(parsed, profile);
}

#[test]
fn profile_rejects_invalid_modes_with_line_and_key_info() {
    let err = parse_profile("name = foo\nrun_mode = sometimes").expect_err("invalid mode");
    let msg = err.to_string();
    assert!(msg.contains("line 2"), "msg: {msg}");
    assert!(msg.contains("run_mode"), "msg: {msg}");
}

#[test]
fn profile_with_comments_and_blank_lines_parses_cleanly() {
    let text = "
        # top-level comment
        name = laptop
        # attach the first project
        attach = primegrid|ABC   # trailing comment
        run_mode = auto

        # blank line above
        gpu_mode = never
    ";
    let profile = parse_profile(text).expect("should tolerate comments/whitespace");
    assert_eq!(profile.name, "laptop");
    assert_eq!(profile.attach.len(), 1);
    assert_eq!(profile.attach[0].url, "https://www.primegrid.com/");
    assert_eq!(profile.run_mode.map(|m| m.as_boinc_tag()), Some("auto"));
    assert_eq!(profile.gpu_mode.map(|m| m.as_boinc_tag()), Some("never"));
}

#[tokio::test]
async fn rpc_client_accepts_attach_and_update_calls() {
    let transport = CountingTransport::new();
    let mut client = BoincRpcClient::new(Box::new(transport), None);
    let mut write = boincrs::boinc::api::write::BoincWriteApi::new(&mut client);
    let _ = write
        .project_attach("https://www.primegrid.com/", "KEY")
        .await
        .expect("attach should succeed");
    let _ = write
        .project_update("https://www.primegrid.com/")
        .await
        .expect("update should succeed");
}
