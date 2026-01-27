use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL};
use std::path::Path;
use wasm_detector::WasmDetector;

/// Detect and manage local WASM files
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// List all local .wasm files in the current directory
    List(ListOpts),
}

#[derive(clap::Args)]
pub(crate) struct ListOpts {
    /// The directory to search (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
    /// Include hidden files and directories
    #[arg(long)]
    hidden: bool,
    /// Follow symbolic links
    #[arg(long)]
    follow_links: bool,
}

impl Opts {
    pub(crate) fn run(self) -> Result<()> {
        match self {
            Opts::List(opts) => {
                let path = Path::new(&opts.path);
                let detector = WasmDetector::new(path)
                    .include_hidden(opts.hidden)
                    .follow_symlinks(opts.follow_links);

                let mut wasm_files = Vec::new();
                for result in detector {
                    match result {
                        Ok(entry) => wasm_files.push(entry),
                        Err(e) => eprintln!("Warning: {}", e),
                    }
                }

                if wasm_files.is_empty() {
                    println!("No .wasm files found in '{}'", path.display());
                } else {
                    let mut table = Table::new();
                    table.load_preset(UTF8_FULL);
                    table.set_header(vec!["File Name", "Path"]);

                    for entry in &wasm_files {
                        let file_name = entry.file_name().unwrap_or("N/A");
                        let path = entry.path().display().to_string();
                        table.add_row(vec![file_name, &path]);
                    }

                    println!("\nFound {} .wasm file(s):\n", wasm_files.len());
                    println!("{}", table);
                }
                Ok(())
            }
        }
    }
}
