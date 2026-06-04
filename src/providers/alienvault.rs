use super::Provider;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;

/// Provider that queries AlienVault OTX passive DNS.
/// No API key required for basic lookups.
pub struct AlienVault;

#[derive(Deserialize)]
struct OtxResponse {
    passive_dns: Vec<OtxDnsEntry>,
}

#[derive(Deserialize)]
struct OtxDnsEntry {
    hostname: String,
}

impl Provider for AlienVault {
    fn name(&self) -> &str {
        "AlienVault OTX"
    }

    fn enumerate(
        &self,
        domain: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HashSet<String>, Box<dyn Error + Send + Sync>>> + Send + '_>> {
        let domain = domain.to_string();
        Box::pin(async move {
            let url = format!(
                "https://otx.alienvault.com/api/v1/indicators/domain/{}/passive_dns",
                domain
            );

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (compatible; RustSubdomainTool/0.1)")
                .build()?;

            let resp = client.get(&url).send().await?;
            let body = resp.text().await?;

            let parsed: OtxResponse = match serde_json::from_str(&body) {
                Ok(r) => r,
                Err(_) => return Ok(HashSet::new()),
            };

            let mut subdomains = HashSet::new();
            for entry in parsed.passive_dns {
                let clean = entry.hostname.trim().to_lowercase();
                if !clean.is_empty() && clean.ends_with(&domain) {
                    subdomains.insert(clean);
                }
            }

            Ok(subdomains)
        })
    }
}
