mod interfaces;
mod known_package_detail;
mod local;
mod package_detail;
pub mod packages;
mod search;
mod settings;

pub use interfaces::InterfacesView;
pub use known_package_detail::KnownPackageDetailView;
pub use local::{LocalView, LocalViewState, LocalWasmFile};
pub use package_detail::PackageDetailView;
pub use packages::PackagesView;
pub use search::{SearchView, SearchViewState};
pub use settings::SettingsView;

pub(crate) use wasm_package_manager::format_size;
