use super::Provider;
use std::collections::HashSet;
use std::error::Error;

/// Provider that queries the Wayback Machine CDX API.
/// No API key required.
pub struct WebArchive;

impl Provider for WebArchive {
    fn name(&self) -> &str {
        "Web Archive"
    }

    fn enumerate(
        &self,
        domain: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HashSet<String>, Box<dyn Error + Send + Sync>>> + Send + '_>> {
        let domain = domain.to_string();
        Box::pin(async move {
            let url = format!(
                "https://web.archive.org/cdx/search/cdx?url=*.{}/*&output=text&fl=original&collapse=urlkey&limit=5000",
                domain
            );

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?;

            let resp = client.get(&url).send().await?;
            let body = resp.text().await?;

            let mut subdomains = HashSet::new();
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                // Extract the hostname from the full URL
                // URLs look like: http://sub.example.com/path
                if let Some(host) = extract_host(line) {
                    let clean = host.to_lowercase();
                    if clean.ends_with(&domain) && !clean.starts_with('*') {
                        subdomains.insert(clean);
                    }
                }
            }

            Ok(subdomains)
        })
    }
}

/// Extract the hostname from a URL string.
fn extract_host(url: &str) -> Option<String> {
    // Try to strip the scheme
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    // Take everything before the first '/' or ':'
    let host = without_scheme
        .split('/')
        .next()?
        .split(':')
        .next()?;

    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}
