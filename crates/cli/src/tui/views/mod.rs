mod interfaces;
mod local;
mod package_detail;
pub mod packages;
mod search;
mod settings;

pub use interfaces::InterfacesView;
pub use local::LocalView;
pub use package_detail::PackageDetailView;
pub use packages::PackagesView;
pub use search::{SearchView, SearchViewState};
pub use settings::SettingsView;

pub(crate) use wasm_package_manager::format_size;
