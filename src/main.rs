use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};
use rand::seq::SliceRandom;
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the terminal size
    let (cols, _) = terminal::size().unwrap_or((80, 24));
    let mut stdout = stdout();

    // Collection of 4 different ASCII banners
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
        
        // Banner 2: The Slant/Block (Cleaned up and highly readable as RST TOOL)
        r#"
           ___   ___  ______   ______  ____  ____  __  
          / _ \ / __//_  __/  /_  __/ / __ \/ __ \/ /  
         / , _/_\ \   / /      / /   / /_/ / /_/ / /__ 
        /_/|_|/___/  /_/      /_/    \____/\____/____/ 
        "#,

        // Banner 3: Professional Block/Pixel Art (ANSI Shadow)
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

        // Banner 4: The Hacker Crab (Ferris Style with full name)
        r#"
            __       __
           / <`     '> \
          (  / @   @ \  )
           \(_ _\_/_ _)/
         (\ `-/     \-' /)
          "===\     /==="
           .==">___<"==.

 [ R U S T - S U B D O M A I N - T O O L ]
        "#,
    ];

    // Pick a random banner
    let mut rng = rand::thread_rng();
    let selected_banner = banners.choose(&mut rng).unwrap();

    // Fallback to a tiny logo if the terminal is extremely narrow
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

    // Right-align Version and GitHub Username
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

    // Tool Help / Execution Instructions
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("Usage: rust_subdomain_tool [options]\n"),
        Print("Use the -h or --help flag to see the flags and subcommands\n\n"),
        ResetColor
    )?;

    Ok(())
}
