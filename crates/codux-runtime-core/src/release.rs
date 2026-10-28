pub const DEFAULT_REPOSITORY: &str = "magalab/codux";

// Recognize the old upstream endpoint so existing installations migrate to this fork.
const LEGACY_REPOSITORY: &str = "duxweb/codux";

pub const REPOSITORY: &str = match option_env!("CODUX_RELEASE_REPOSITORY") {
    Some(repository) if !repository.is_empty() => repository,
    _ => DEFAULT_REPOSITORY,
};

pub fn github_release_url(path: &str) -> String {
    format!("https://github.com/{REPOSITORY}{path}")
}

pub fn github_raw_url(path: &str) -> String {
    format!("https://raw.githubusercontent.com/{REPOSITORY}/main{path}")
}

pub fn is_managed_update_endpoint(endpoint: &str) -> bool {
    [REPOSITORY, DEFAULT_REPOSITORY, LEGACY_REPOSITORY]
        .iter()
        .any(|repository| {
            [
                format!("https://github.com/{repository}/releases/latest/download/latest.json"),
                format!("https://github.com/{repository}/releases/download/beta/latest.json"),
                format!(
                    "https://raw.githubusercontent.com/{repository}/main/updates/stable/latest.json"
                ),
                format!(
                    "https://raw.githubusercontent.com/{repository}/main/updates/beta/latest.json"
                ),
            ]
            .iter()
            .any(|managed| managed == endpoint)
        })
}
