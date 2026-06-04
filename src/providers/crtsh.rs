use super::Provider;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;

/// Provider that queries crt.sh Certificate Transparency logs.
/// No API key required.
pub struct CrtSh;

#[derive(Deserialize)]
struct CrtShEntry {
    name_value: String,
}

impl Provider for CrtSh {
    fn name(&self) -> &str {
        "crt.sh"
    }

    fn enumerate(
        &self,
        domain: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HashSet<String>, Box<dyn Error + Send + Sync>>> + Send + '_>> {
        let domain = domain.to_string();
        Box::pin(async move {
            let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .user_agent("Mozilla/5.0 (compatible; RustSubdomainTool/0.1)")
                .build()?;

            let resp = client.get(&url).send().await?;

            // crt.sh can return HTML error pages or huge payloads.
            // Read as text first, then try to parse JSON.
            let body = resp.text().await?;

            let entries: Vec<CrtShEntry> = match serde_json::from_str(&body) {
                Ok(e) => e,
                Err(_) => {
                    // If JSON parsing fails, return empty rather than erroring
                    return Ok(HashSet::new());
                }
            };

            let mut subdomains = HashSet::new();
            for entry in entries {
                // crt.sh can return wildcard and newline-separated entries
                for name in entry.name_value.split('\n') {
                    let clean = name.trim().to_lowercase();
                    // Skip wildcards and empty strings
                    if !clean.is_empty() && !clean.starts_with('*') && clean.ends_with(&domain) {
                        subdomains.insert(clean);
                    }
                }
            }

            Ok(subdomains)
        })
    }
}
