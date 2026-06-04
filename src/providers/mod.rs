pub mod alienvault;
pub mod crtsh;
pub mod hackertarget;
pub mod webarchive;

use std::collections::HashSet;
use std::error::Error;
use futures::future::join_all;

/// Trait that every subdomain data source must implement.
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;

    /// Query the provider for subdomains of `domain`.
    /// Returns a set of discovered subdomain strings.
    fn enumerate(
        &self,
        domain: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HashSet<String>, Box<dyn Error + Send + Sync>>> + Send + '_>>;
}

/// Run all providers concurrently and merge their results.
pub async fn run_all(domain: &str, verbose: bool) -> HashSet<String> {
    let providers: Vec<Box<dyn Provider>> = vec![
        Box::new(crtsh::CrtSh),
        Box::new(webarchive::WebArchive),
        Box::new(alienvault::AlienVault),
        Box::new(hackertarget::HackerTarget),
    ];

    let futures = providers.iter().map(|p| {
        let name = p.name().to_string();
        let verbose = verbose;
        async move {
            if verbose {
                eprintln!("  [*] Querying {} ...", name);
            }
            match p.enumerate(domain).await {
                Ok(results) => {
                    if verbose {
                        eprintln!("  [+] {} returned {} subdomains", name, results.len());
                    }
                    results
                }
                Err(e) => {
                    eprintln!("  [!] {} failed: {}", name, e);
                    HashSet::new()
                }
            }
        }
    });

    let all_results = join_all(futures).await;

    let mut merged: HashSet<String> = HashSet::new();
    for set in all_results {
        merged.extend(set);
    }

    merged
}
