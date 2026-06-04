use super::Provider;
use std::collections::HashSet;
use std::error::Error;

/// Provider that queries HackerTarget's free subdomain finder API.
/// No API key required (rate-limited to ~100 requests/day).
pub struct HackerTarget;

impl Provider for HackerTarget {
    fn name(&self) -> &str {
        "HackerTarget"
    }

    fn enumerate(
        &self,
        domain: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HashSet<String>, Box<dyn Error + Send + Sync>>> + Send + '_>> {
        let domain = domain.to_string();
        Box::pin(async move {
            let url = format!("https://api.hackertarget.com/hostsearch/?q={}", domain);

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (compatible; RustSubdomainTool/0.1)")
                .build()?;

            let resp = client.get(&url).send().await?;
            let body = resp.text().await?;

            // HackerTarget returns lines like: subdomain.example.com,1.2.3.4
            // or an error message starting with "error"
            if body.starts_with("error") || body.contains("API count exceeded") {
                return Ok(HashSet::new());
            }

            let mut subdomains = HashSet::new();
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                // Format: hostname,ip
                if let Some(hostname) = line.split(',').next() {
                    let clean = hostname.trim().to_lowercase();
                    if !clean.is_empty() && clean.ends_with(&domain) {
                        subdomains.insert(clean);
                    }
                }
            }

            Ok(subdomains)
        })
    }
}
