use std::collections::{HashMap, HashSet};
use std::net::ToSocketAddrs;
use tokio::task;

/// Resolve a set of subdomains to their IP addresses.
/// Returns a map of subdomain -> Vec<IP>.
/// Subdomains that fail to resolve are excluded.
pub async fn resolve_subdomains(
    subdomains: &HashSet<String>,
    verbose: bool,
) -> HashMap<String, Vec<String>> {
    let mut handles = Vec::new();

    for subdomain in subdomains.iter() {
        let sub = subdomain.clone();
        let verbose = verbose;

        let handle = task::spawn_blocking(move || {
            let lookup = format!("{}:0", sub);
            match lookup.to_socket_addrs() {
                Ok(addrs) => {
                    let ips: Vec<String> = addrs.map(|a| a.ip().to_string()).collect();
                    if ips.is_empty() {
                        None
                    } else {
                        Some((sub, ips))
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("  [-] {} did not resolve: {}", sub, e);
                    }
                    None
                }
            }
        });

        handles.push(handle);
    }

    let mut resolved = HashMap::new();
    for handle in handles {
        if let Ok(Some((sub, ips))) = handle.await {
            resolved.insert(sub, ips);
        }
    }

    resolved
}
