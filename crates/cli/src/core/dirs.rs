use std::env;
use std::path::PathBuf;

/// Get the location of the crate's data dir
pub(crate) fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("could not find a local data dir")
        .join("wasm")
}

/// Get the location of the crate's cache dir
pub(crate) fn artifact_dir() -> PathBuf {
    data_dir().join("artifacts")
}

/// Get the location of the current executable
pub(crate) fn executable_dir() -> String {
    match env::current_exe() {
        Ok(exe_path) => exe_path.display().to_string(),
        Err(_) => String::from("unknown executable dir"),
    }
}
