mod cli;
mod output;
mod providers;
mod resolver;

use clap::Parser;
use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};
use rand::seq::SliceRandom;
use std::io::stdout;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ─── Banner ───────────────────────────────────────────────
    let (cols, _) = terminal::size().unwrap_or((80, 24));
    let mut stdout = stdout();

    let banners = [
        // Banner 1: The Classic Node/Network
        r#"
      +@@@@@@@W.    .o@@@@@@o.  +@@@@@@@@@@o
      &@&++++o@W    &@&++++++:  +@@++++++++:
      &@&    :W@    &@o         ...o@&...   
      &@@@@@@@W.    .W@@@@@@W:     o@&      
      &@&+++o@W.          .+@&     o@&      
      &@&    &@o    &@&++++o@&     o@&      
      o@o    +@&    .o@@@@@@o.     +@+      
        "#,
        // Banner 2: The Slant/Block
        r#"
           ___   ___  ______   ______  ____  ____  __  
          / _ \ / __//_  __/  /_  __/ / __ \/ __ \/ /  
         / , _/_\ \   / /      / /   / /_/ / /_/ / /__ 
        /_/|_|/___/  /_/      /_/    \____/\____/____/ 
        "#,
        // Banner 3: Professional ANSI Shadow
        r#"
██████╗ ██╗   ██╗███████╗████████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝
██████╔╝██║   ██║███████╗   ██║   
██╔══██╗██║   ██║╚════██║   ██║   
██║  ██║╚██████╔╝███████║   ██║   
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
███████╗██╗   ██╗██████╗ ██████╗  ██████╗ ███╗   ███╗ █████╗ ██╗██╗   ██╗
██╔════╝██║   ██║██╔══██╗██╔══██╗██╔═══██╗████╗ ████║██╔══██╗██║████╗  ██║
███████╗██║   ██║██████╔╝██║  ██║██║   ██║██╔████╔██║███████║██║██╔██╗ ██║
╚════██║██║   ██║██╔══██╗██║  ██║██║   ██║██║╚██╔╝██║██╔══██║██║██║╚██╗██║
███████║╚██████╔╝██████╔╝██████╔╝╚██████╔╝██║ ╚═╝ ██║██║  ██║██║██║ ╚████║
╚══════╝ ╚═════╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
████████╗ ██████╗  ██████╗ ██╗     
╚══██╔══╝██╔═══██╗██╔═══██╗██║     
   ██║   ██║   ██║██║   ██║██║     
   ██║   ██║   ██║██║   ██║██║     
   ██║   ╚██████╔╝╚██████╔╝███████╗
   ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝
        "#,
        // Banner 4: The Hacker Crab (Ferris Style)
        r#"
            __       __
           / <`     '> \
          (  / @   @ \  )
           \(_ _\_/_ _)/
         (\ `-/     \-' /)
          "===\     /==="
           .==">==="==.

 [ R U S T - S U B D O M A I N - T O O L ]
        "#,
    ];

    // Pick a random banner (shuffled like msfconsole)
    let mut rng = rand::thread_rng();
    let selected_banner = banners.choose(&mut rng).unwrap();

    let ascii_art = if cols > 76 {
        selected_banner
    } else {
        r#"
  [ R U S T ]
  [ S U B D O M A I N ]
  [ T O O L ]
        "#
    };

    // Print ASCII Art in Red
    execute!(
        stdout,
        SetForegroundColor(Color::Red),
        Print(ascii_art),
        Print("\n"),
        ResetColor
    )?;

    // Right-align Version and Author
    let version = "v0.1.0";
    let author = "@SAMIR897";

    let margin = 2;
    let version_padding = cols.saturating_sub(version.len() as u16 + margin);
    let author_padding = cols.saturating_sub(author.len() as u16 + margin);

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(" ".repeat(version_padding as usize)),
        Print(version),
        Print("\n"),
        Print(" ".repeat(author_padding as usize)),
        Print(author),
        Print("\n\n"),
        ResetColor
    )?;

    // Center the Bold Tagline
    let tagline = "Rust-powered asset and target surface discovery hidden under layers.";

    let tagline_padding = if cols > tagline.len() as u16 {
        (cols - tagline.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Yellow),
        Print(" ".repeat(tagline_padding as usize)),
        Print(tagline),
        Print("\n\n"),
        ResetColor,
        SetAttribute(Attribute::Reset)
    )?;

    // ─── Parse CLI ────────────────────────────────────────────
    let args = cli::Cli::parse();
    let domain = &args.domain;

    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print(format!("  [~] Target: {}\n", domain)),
        SetForegroundColor(Color::DarkGrey),
        Print("  [~] Providers: crt.sh, Web Archive, AlienVault OTX, HackerTarget\n"),
        Print(format!("  [~] Resolve: {}\n\n", if args.resolve { "yes" } else { "no" })),
        ResetColor
    )?;

    // ─── Enumerate ────────────────────────────────────────────
    let start = Instant::now();

    let subdomains = providers::run_all(domain, args.verbose).await;

    let elapsed = start.elapsed();

    if subdomains.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("  [!] No subdomains found.\n\n"),
            ResetColor
        )?;
        return Ok(());
    }

    // ─── Resolve (optional) ───────────────────────────────────
    let resolved = if args.resolve {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "  [~] Resolving {} subdomains ...\n",
                subdomains.len()
            )),
            ResetColor
        )?;
        Some(resolver::resolve_subdomains(&subdomains, args.verbose).await)
    } else {
        None
    };

    // ─── Output ───────────────────────────────────────────────
    if args.json {
        output::print_json(domain, &subdomains, resolved.as_ref());
    } else {
        output::print_results(domain, &subdomains, resolved.as_ref());
    }

    // Save to file if requested
    if let Some(ref path) = args.output {
        output::save_to_file(&subdomains, resolved.as_ref(), path)?;
    }

    // ─── Summary ──────────────────────────────────────────────
    if !args.json {
        let resolved_count = resolved
            .as_ref()
            .map(|r| r.len())
            .unwrap_or(subdomains.len());

        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "  ⏱  Completed in {:.2}s — {} unique subdomains",
                elapsed.as_secs_f64(),
                resolved_count
            )),
            Print("\n\n"),
            ResetColor
        )?;
    }

    Ok(())
}
