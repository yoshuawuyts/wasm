mod home;
mod interfaces;
mod package_detail;
pub(crate) mod packages;
mod settings;

pub(crate) use home::HomeView;
pub(crate) use interfaces::InterfacesView;
pub(crate) use package_detail::PackageDetailView;
pub(crate) use packages::PackagesView;
pub(crate) use settings::SettingsView;
