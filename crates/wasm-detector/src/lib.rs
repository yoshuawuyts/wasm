//! A library to detect local `.wasm` files in a repository.
//!
//! This crate provides functionality to find WebAssembly files while:
//! - Respecting `.gitignore` rules
//! - Including well-known `.wasm` locations that are typically ignored
//!   (e.g., `target/wasm32-*`, `pkg/`, `dist/`)
//!
//! # Example
//!
//! ```no_run
//! use wasm_detector::WasmDetector;
//! use std::path::Path;
//!
//! let detector = WasmDetector::new(Path::new("."));
//! for result in detector {
//!     match result {
//!         Ok(entry) => println!("Found: {}", entry.path().display()),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Well-known directories that typically contain `.wasm` files but are often ignored.
///
/// These directories are scanned separately without respecting `.gitignore` rules
/// to ensure important wasm output locations are always included.
pub const WELL_KNOWN_WASM_DIRS: &[&str] = &[
    // Rust wasm targets (the target directory is scanned for wasm32-* subdirs)
    "target", // wasm-pack output
    "pkg",    // JavaScript/jco output
    "dist",
];

/// Patterns to match within the target directory for wasm-specific subdirectories.
const TARGET_WASM_PREFIXES: &[&str] = &["wasm32-"];

/// A discovered WebAssembly file entry.
#[derive(Debug, Clone)]
pub struct WasmEntry {
    path: PathBuf,
}

impl WasmEntry {
    /// Create a new WasmEntry from a path.
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Returns the path to the `.wasm` file.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the file name of the `.wasm` file.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|s| s.to_str())
    }

    /// Consumes the entry and returns the underlying path.
    #[must_use]
    pub fn into_path(self) -> PathBuf {
        self.path
    }
}

/// A detector that finds `.wasm` files in a directory tree.
///
/// The detector:
/// - Respects `.gitignore` rules by default
/// - Automatically includes well-known `.wasm` locations that are typically ignored
/// - Returns an iterator over discovered `.wasm` files
///
/// # Example
///
/// ```no_run
/// use wasm_detector::WasmDetector;
/// use std::path::Path;
///
/// let detector = WasmDetector::new(Path::new("."));
/// let wasm_files: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();
/// println!("Found {} wasm files", wasm_files.len());
/// ```
#[derive(Debug, Clone)]
pub struct WasmDetector {
    root: PathBuf,
    include_hidden: bool,
    follow_symlinks: bool,
}

impl WasmDetector {
    /// Create a new detector that will search from the given root directory.
    #[must_use]
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            include_hidden: false,
            follow_symlinks: false,
        }
    }

    /// Set whether to include hidden files and directories.
    ///
    /// By default, hidden files are excluded.
    #[must_use]
    pub fn include_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }

    /// Set whether to follow symbolic links.
    ///
    /// By default, symbolic links are not followed.
    #[must_use]
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }

    /// Detect `.wasm` files and return all results as a vector.
    ///
    /// This is a convenience method that collects all results.
    /// For large directories, consider using the iterator interface instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the detection fails to complete.
    pub fn detect(&self) -> Result<Vec<WasmEntry>, ignore::Error> {
        self.into_iter().collect()
    }

    /// Find all well-known wasm directories that exist in the root.
    fn find_well_known_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // Check for pkg/ and dist/ directories
        for dir_name in &["pkg", "dist"] {
            let dir_path = self.root.join(dir_name);
            if dir_path.is_dir() {
                dirs.push(dir_path);
            }
        }

        // Check for target/wasm32-* directories
        let target_dir = self.root.join("target");
        if target_dir.is_dir()
            && let Ok(entries) = std::fs::read_dir(&target_dir)
        {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir()
                    && let Some(name) = path.file_name().and_then(|n| n.to_str())
                {
                    for prefix in TARGET_WASM_PREFIXES {
                        if name.starts_with(prefix) {
                            dirs.push(path);
                            break;
                        }
                    }
                }
            }
        }

        dirs
    }
}

impl IntoIterator for WasmDetector {
    type Item = Result<WasmEntry, ignore::Error>;
    type IntoIter = WasmDetectorIter;

    fn into_iter(self) -> Self::IntoIter {
        WasmDetectorIter::new(self)
    }
}

impl IntoIterator for &WasmDetector {
    type Item = Result<WasmEntry, ignore::Error>;
    type IntoIter = WasmDetectorIter;

    fn into_iter(self) -> Self::IntoIter {
        WasmDetectorIter::new(self.clone())
    }
}

/// Iterator over discovered `.wasm` files.
///
/// This iterator combines results from multiple walks:
/// 1. A main walk that respects `.gitignore`
/// 2. Additional walks for well-known directories (ignoring `.gitignore`)
pub struct WasmDetectorIter {
    /// The main walker that respects gitignore
    main_walker: ignore::Walk,
    /// Walkers for well-known directories (ignoring gitignore)
    well_known_walkers: Vec<ignore::Walk>,
    /// Current index in well_known_walkers
    current_well_known_idx: usize,
    /// Set of paths already seen (to avoid duplicates)
    seen_paths: HashSet<PathBuf>,
    /// Whether we've finished the main walk
    main_walk_done: bool,
}

impl std::fmt::Debug for WasmDetectorIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmDetectorIter")
            .field("main_walk_done", &self.main_walk_done)
            .field("current_well_known_idx", &self.current_well_known_idx)
            .field("seen_paths_count", &self.seen_paths.len())
            .finish_non_exhaustive()
    }
}

impl WasmDetectorIter {
    fn new(detector: WasmDetector) -> Self {
        // Build the main walker that respects gitignore
        let main_walker = WalkBuilder::new(&detector.root)
            .hidden(!detector.include_hidden)
            .follow_links(detector.follow_symlinks)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build();

        // Build walkers for well-known directories (ignoring gitignore)
        let well_known_dirs = detector.find_well_known_dirs();
        let well_known_walkers: Vec<_> = well_known_dirs
            .into_iter()
            .map(|dir| {
                WalkBuilder::new(dir)
                    .hidden(!detector.include_hidden)
                    .follow_links(detector.follow_symlinks)
                    .git_ignore(false) // Don't respect gitignore for well-known dirs
                    .git_global(false)
                    .git_exclude(false)
                    .build()
            })
            .collect();

        Self {
            main_walker,
            well_known_walkers,
            current_well_known_idx: 0,
            seen_paths: HashSet::new(),
            main_walk_done: false,
        }
    }

    /// Try to get the next .wasm file from the main walker
    fn next_from_main(&mut self) -> Option<Result<WasmEntry, ignore::Error>> {
        loop {
            match self.main_walker.next() {
                Some(Ok(entry)) => {
                    let path = entry.path();
                    if path.is_file() && path.extension().is_some_and(|ext| ext == "wasm") {
                        let path_buf = path.to_path_buf();
                        self.seen_paths.insert(path_buf.clone());
                        return Some(Ok(WasmEntry::new(path_buf)));
                    }
                    // Continue to next entry
                }
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    self.main_walk_done = true;
                    return None;
                }
            }
        }
    }

    /// Try to get the next .wasm file from well-known walkers
    fn next_from_well_known(&mut self) -> Option<Result<WasmEntry, ignore::Error>> {
        while self.current_well_known_idx < self.well_known_walkers.len() {
            if let Some(walker) = self.well_known_walkers.get_mut(self.current_well_known_idx) {
                loop {
                    match walker.next() {
                        Some(Ok(entry)) => {
                            let path = entry.path();
                            if path.is_file() && path.extension().is_some_and(|ext| ext == "wasm") {
                                let path_buf = path.to_path_buf();
                                // Skip if we've already seen this path
                                if self.seen_paths.contains(&path_buf) {
                                    continue;
                                }
                                self.seen_paths.insert(path_buf.clone());
                                return Some(Ok(WasmEntry::new(path_buf)));
                            }
                            // Continue to next entry
                        }
                        Some(Err(e)) => return Some(Err(e)),
                        None => {
                            // Move to next well-known walker
                            self.current_well_known_idx += 1;
                            break;
                        }
                    }
                }
            } else {
                self.current_well_known_idx += 1;
            }
        }
        None
    }
}

impl Iterator for WasmDetectorIter {
    type Item = Result<WasmEntry, ignore::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // First, exhaust the main walker
        if !self.main_walk_done
            && let Some(result) = self.next_from_main()
        {
            return Some(result);
        }

        // Then, go through well-known directories
        self.next_from_well_known()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    /// Create a test directory structure with some .wasm files
    fn setup_test_dir() -> TempDir {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let root = temp_dir.path();

        // Create some regular .wasm files
        fs::create_dir_all(root.join("src")).unwrap();
        File::create(root.join("src/module.wasm")).unwrap();

        // Create .wasm files in target directory (typically gitignored)
        fs::create_dir_all(root.join("target/wasm32-wasip2/release")).unwrap();
        File::create(root.join("target/wasm32-wasip2/release/app.wasm")).unwrap();

        fs::create_dir_all(root.join("target/wasm32-unknown-unknown/debug")).unwrap();
        File::create(root.join("target/wasm32-unknown-unknown/debug/lib.wasm")).unwrap();

        // Create .wasm files in pkg directory (wasm-pack output)
        fs::create_dir_all(root.join("pkg")).unwrap();
        File::create(root.join("pkg/my_crate_bg.wasm")).unwrap();

        // Create .wasm files in dist directory (jco output)
        fs::create_dir_all(root.join("dist")).unwrap();
        File::create(root.join("dist/component.wasm")).unwrap();

        // Create a non-.wasm file
        File::create(root.join("src/main.rs")).unwrap();

        temp_dir
    }

    #[test]
    fn test_detector_finds_wasm_files() {
        let temp_dir = setup_test_dir();
        let detector = WasmDetector::new(temp_dir.path());

        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        // Should find all .wasm files
        assert!(
            results.len() >= 5,
            "Expected at least 5 .wasm files, found {}",
            results.len()
        );

        // Verify all results have .wasm extension
        for entry in &results {
            assert!(
                entry.path().extension().is_some_and(|e| e == "wasm"),
                "Expected .wasm extension for {:?}",
                entry.path()
            );
        }
    }

    #[test]
    fn test_detector_finds_target_wasm_files() {
        let temp_dir = setup_test_dir();
        let detector = WasmDetector::new(temp_dir.path());

        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        // Check that we found files in target directories
        let target_files: Vec<_> = results
            .iter()
            .filter(|e| e.path().to_string_lossy().contains("target"))
            .collect();

        assert!(
            target_files.len() >= 2,
            "Expected at least 2 files in target directory, found {}",
            target_files.len()
        );
    }

    #[test]
    fn test_detector_finds_pkg_wasm_files() {
        let temp_dir = setup_test_dir();
        let detector = WasmDetector::new(temp_dir.path());

        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        // Check that we found files in pkg directory
        let pkg_files: Vec<_> = results
            .iter()
            .filter(|e| e.path().to_string_lossy().contains("pkg"))
            .collect();

        assert_eq!(pkg_files.len(), 1, "Expected 1 file in pkg directory");
    }

    #[test]
    fn test_detector_finds_dist_wasm_files() {
        let temp_dir = setup_test_dir();
        let detector = WasmDetector::new(temp_dir.path());

        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        // Check that we found files in dist directory
        let dist_files: Vec<_> = results
            .iter()
            .filter(|e| e.path().to_string_lossy().contains("dist"))
            .collect();

        assert_eq!(dist_files.len(), 1, "Expected 1 file in dist directory");
    }

    #[test]
    fn test_wasm_entry_methods() {
        let entry = WasmEntry::new(PathBuf::from("/path/to/module.wasm"));

        assert_eq!(entry.path(), Path::new("/path/to/module.wasm"));
        assert_eq!(entry.file_name(), Some("module.wasm"));
        assert_eq!(entry.into_path(), PathBuf::from("/path/to/module.wasm"));
    }

    #[test]
    fn test_detector_with_gitignore() {
        use std::process::Command;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let root = temp_dir.path();

        // Initialize a git repository so that .gitignore is respected
        Command::new("git")
            .args(["init"])
            .current_dir(root)
            .output()
            .expect("Failed to initialize git repository");

        // Create a .gitignore that ignores the "ignored" directory
        fs::write(root.join(".gitignore"), "ignored/\n").unwrap();

        // Create .wasm files in various locations
        fs::create_dir_all(root.join("src")).unwrap();
        File::create(root.join("src/visible.wasm")).unwrap();

        fs::create_dir_all(root.join("ignored")).unwrap();
        File::create(root.join("ignored/hidden.wasm")).unwrap();

        // Create files in well-known directories (should be included despite gitignore)
        fs::create_dir_all(root.join("target/wasm32-wasip2")).unwrap();
        File::create(root.join("target/wasm32-wasip2/app.wasm")).unwrap();

        let detector = WasmDetector::new(root);
        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        // Should find src/visible.wasm and target/wasm32-wasip2/app.wasm
        // but NOT ignored/hidden.wasm
        let paths: Vec<_> = results.iter().map(|e| e.path().to_owned()).collect();

        assert!(
            paths.iter().any(|p| p.ends_with("visible.wasm")),
            "Should find visible.wasm"
        );
        assert!(
            paths.iter().any(|p| p.ends_with("app.wasm")),
            "Should find app.wasm in target"
        );
        assert!(
            !paths.iter().any(|p| p.ends_with("hidden.wasm")),
            "Should NOT find hidden.wasm (gitignored)"
        );
    }

    #[test]
    fn test_detect_convenience_method() {
        let temp_dir = setup_test_dir();
        let detector = WasmDetector::new(temp_dir.path());

        let results = detector.detect().expect("Detect should succeed");

        assert!(
            results.len() >= 5,
            "Expected at least 5 .wasm files, found {}",
            results.len()
        );
    }

    #[test]
    fn test_detector_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let detector = WasmDetector::new(temp_dir.path());

        let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

        assert!(
            results.is_empty(),
            "Empty directory should yield no results"
        );
    }
}
