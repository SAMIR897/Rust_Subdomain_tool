use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::collections::{HashMap, HashSet};
use std::io::stdout;
use std::path::PathBuf;

/// Print results to the terminal with colors.
pub fn print_results(
    domain: &str,
    subdomains: &HashSet<String>,
    resolved: Option<&HashMap<String, Vec<String>>>,
) {
    let mut stdout = stdout();
    let mut sorted: Vec<&String> = subdomains.iter().collect();
    sorted.sort();

    let _ = execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print(format!(
            "\n━━━ Found {} unique subdomains for {} ━━━\n\n",
            sorted.len(),
            domain
        )),
        ResetColor
    );

    for sub in &sorted {
        if let Some(ref res) = resolved {
            if let Some(ips) = res.get(*sub) {
                let _ = execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!("  ✓ {}", sub)),
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("  →  {}\n", ips.join(", "))),
                    ResetColor
                );
            }
            // If resolving was requested but this sub didn't resolve, skip it
        } else {
            let _ = execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print(format!("  • {}\n", sub)),
                ResetColor
            );
        }
    }

    let _ = execute!(stdout, Print("\n"));
}

/// Save results to a file.
pub fn save_to_file(
    subdomains: &HashSet<String>,
    resolved: Option<&HashMap<String, Vec<String>>>,
    path: &PathBuf,
) -> std::io::Result<()> {
    let mut sorted: Vec<&String> = subdomains.iter().collect();
    sorted.sort();

    let mut content = String::new();
    for sub in &sorted {
        if let Some(ref res) = resolved {
            if let Some(ips) = res.get(*sub) {
                content.push_str(&format!("{} -> {}\n", sub, ips.join(", ")));
            }
        } else {
            content.push_str(&format!("{}\n", sub));
        }
    }

    std::fs::write(path, content)?;
    let mut stdout = stdout();
    let _ = execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("  [✓] Results saved to {}\n\n", path.display())),
        ResetColor
    );

    Ok(())
}

/// Output results as JSON to stdout.
pub fn print_json(
    domain: &str,
    subdomains: &HashSet<String>,
    resolved: Option<&HashMap<String, Vec<String>>>,
) {
    let mut sorted: Vec<&String> = subdomains.iter().collect();
    sorted.sort();

    if let Some(res) = resolved {
        let entries: Vec<serde_json::Value> = sorted
            .iter()
            .filter_map(|sub| {
                res.get(*sub).map(|ips| {
                    serde_json::json!({
                        "subdomain": sub,
                        "ips": ips,
                    })
                })
            })
            .collect();

        let output = serde_json::json!({
            "domain": domain,
            "total": entries.len(),
            "results": entries,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        let output = serde_json::json!({
            "domain": domain,
            "total": sorted.len(),
            "subdomains": sorted,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}
