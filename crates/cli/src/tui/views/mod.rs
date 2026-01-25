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

pub(crate) fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
