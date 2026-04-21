use crate::boinc::api::write::BoincWriteApi;
use crate::boinc::rpc_client::BoincRpcClient;
use crate::error::AppResult;

#[derive(Debug, Clone)]
pub struct AttachProject {
    pub url: String,
    pub account_key: String,
}

pub async fn attach_projects_from_env(rpc: &mut BoincRpcClient) -> AppResult<usize> {
    let projects = discover_attach_projects();
    if projects.is_empty() {
        return Ok(0);
    }

    let mut write_api = BoincWriteApi::new(rpc);
    let mut attached = 0usize;
    for project in projects {
        let _ = write_api
            .project_attach(project.url.as_str(), project.account_key.as_str())
            .await?;
        let _ = write_api.project_update(project.url.as_str()).await?;
        attached += 1;
    }
    Ok(attached)
}

fn discover_attach_projects() -> Vec<AttachProject> {
    let mut out = Vec::new();

    if let Some(key) = read_env_trimmed("BOINCRS_PRIMEGRID_ACCOUNT_KEY") {
        out.push(AttachProject {
            url: "https://www.primegrid.com/".to_string(),
            account_key: key,
        });
    }
    if let Some(key) = read_env_trimmed("BOINCRS_ASTEROIDS_ACCOUNT_KEY") {
        out.push(AttachProject {
            url: "https://asteroidsathome.net/boinc/".to_string(),
            account_key: key,
        });
    }
    if let Some(spec) = read_env_trimmed("BOINCRS_ATTACH_PROJECTS") {
        for pair in spec.split(';') {
            if let Some(project) = parse_project_pair(pair) {
                out.push(project);
            }
        }
    }
    out
}

fn parse_project_pair(pair: &str) -> Option<AttachProject> {
    let mut parts = pair.split('|');
    let url = parts.next()?.trim();
    let key = parts.next()?.trim();
    if url.is_empty() || key.is_empty() {
        return None;
    }
    Some(AttachProject {
        url: url.to_string(),
        account_key: key.to_string(),
    })
}

fn read_env_trimmed(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

