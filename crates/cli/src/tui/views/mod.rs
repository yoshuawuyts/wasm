mod interfaces;
mod local;
mod package_detail;
pub(crate) mod packages;
mod search;
mod settings;

pub(crate) use interfaces::InterfacesView;
pub(crate) use local::LocalView;
pub(crate) use package_detail::PackageDetailView;
pub(crate) use packages::PackagesView;
pub(crate) use search::{SearchView, SearchViewState};
pub(crate) use settings::SettingsView;

pub(crate) use wasm_package_manager::format_size;
