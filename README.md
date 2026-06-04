# Rust Subdomain Tool (RST)

<img align="right" src="Rust Subdomain Tool.png" width="200" alt="Rust Subdomain Tool logo" style="border-radius: 15px; margin-left: 20px; margin-bottom: 20px;">

**Rust Subdomain Tool** is a fast, async subdomain enumeration CLI built in Rust. It queries multiple passive data sources concurrently to discover subdomains — no brute-forcing, no noise, just clean OSINT recon.

Built for security researchers, bug bounty hunters, and pentesters who want speed without sacrificing reliability.

<br>
 <p align="right">Rust Subdomain Tool&emsp;&ensp;</p>

_____

## Features

- 🔥 **Async & Concurrent** — All providers queried simultaneously using Tokio
- 🎲 **Randomized Banners** — Shuffled ASCII art on every launch (like msfconsole)
- 🌐 **4 Passive Providers** — crt.sh, Web Archive, AlienVault OTX, HackerTarget
- 🔍 **DNS Resolution** — Optionally resolve subdomains to IP addresses
- 📁 **File Export** — Save results to a text file
- 📊 **JSON Output** — Pipe-friendly structured output
- 🔐 **GPG Signed Releases** — Every release is cryptographically signed
- 🦀 **Single Binary** — No runtime dependencies, just drop and run

## Installation

### From Source

```bash
git clone https://github.com/SAMIR897/Rust_Subdomain_tool.git
cd Rust_Subdomain_tool
cargo build --release
```

The binary will be at `target/release/rst`.

### Quick Install (Unix)

```bash
cargo install --git https://github.com/SAMIR897/Rust_Subdomain_tool.git
```

## Usage

```bash
# Basic scan
rst example.com

# Scan with DNS resolution
rst example.com -r

# Save results to file
rst example.com -o subdomains.txt

# JSON output (pipe-friendly)
rst example.com --json

# Verbose mode (show provider status)
rst example.com -v

# Combine flags
rst example.com -r -v -o results.txt
```

## Providers

| Provider | Source | API Key | Description |
|----------|--------|---------|-------------|
| **crt.sh** | Certificate Transparency | ❌ None | Queries CT logs for issued certificates |
| **Web Archive** | Wayback Machine CDX | ❌ None | Extracts hostnames from archived URLs |
| **AlienVault OTX** | Passive DNS | ❌ None | OTX threat intelligence passive DNS |
| **HackerTarget** | Host Search | ❌ None | Free subdomain & host discovery API |

> All providers are free and require **no API keys** for basic usage.

## Example Output

```
      +@@@@@@@W.    .o@@@@@@o.  +@@@@@@@@@@o
      &@&++++o@W    &@&++++++:  +@@++++++++:
      &@&    :W@    &@o         ...o@&...
      &@@@@@@@W.    .W@@@@@@W:     o@&
      &@&+++o@W.          .+@&     o@&
      &@&    &@o    &@&++++o@&     o@&
      o@o    +@&    .o@@@@@@o.     +@+
                                                v0.1.0
                                             @SAMIR897

  Rust-powered asset and target surface discovery hidden under layers.

  [~] Target: tesla.com
  [~] Providers: crt.sh, Web Archive, AlienVault OTX, HackerTarget
  [~] Resolve: no

  ━━━ Found 51 unique subdomains for tesla.com ━━━

  • accounts.tesla.com
  • auth.tesla.com
  • billing.tesla.com
  • fleet-api.prd.na.vn.cloud.tesla.com
  • ...

  ⏱  Completed in 4.21s — 51 unique subdomains
```

## Project Structure

```
src/
├── main.rs                 # Banner, CLI entry point, orchestration
├── cli.rs                  # Clap argument definitions
├── resolver.rs             # Async DNS resolution
├── output.rs               # Terminal, file, and JSON output
└── providers/
    ├── mod.rs              # Provider trait & concurrent runner
    ├── crtsh.rs            # crt.sh Certificate Transparency
    ├── webarchive.rs       # Wayback Machine CDX API
    ├── alienvault.rs       # AlienVault OTX passive DNS
    └── hackertarget.rs     # HackerTarget host search
```

## Roadmap

- [ ] Add more providers (Shodan, SecurityTrails, Censys)
- [ ] Active brute-force mode with wordlists
- [ ] Subdomain monitoring & alerting (Slack/Discord)
- [ ] WASM plugin system for custom providers
- [ ] TUI dashboard with `ratatui`
- [ ] HTTP probing (built-in httpx alternative)

## License

MIT

## Author

**[@SAMIR897](https://github.com/SAMIR897)**
