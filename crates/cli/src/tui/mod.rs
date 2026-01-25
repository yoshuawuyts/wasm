mod app;
mod views;

use app::App;
use tokio::sync::mpsc;
use wasm_package_manager::{ImageEntry, Manager, Reference, StateInfo};

/// Events sent from the TUI to the Manager
#[derive(Debug)]
pub enum AppEvent {
    /// Request to quit the application
    Quit,
    /// Request the list of packages
    RequestPackages,
    /// Request state info
    RequestStateInfo,
    /// Pull a package from a registry
    Pull(String),
    /// Delete a package by its reference
    Delete(String),
}

/// Events sent from the Manager to the TUI
#[derive(Debug)]
pub enum ManagerEvent {
    /// Manager has finished initializing
    Ready,
    /// List of packages
    PackagesList(Vec<ImageEntry>),
    /// State information
    StateInfo(StateInfo),
    /// Result of a pull operation
    PullResult(Result<(), String>),
    /// Result of a delete operation
    DeleteResult(Result<(), String>),
}

pub async fn run() -> anyhow::Result<()> {
    // Create channels for bidirectional communication
    let (app_sender, app_receiver) = mpsc::channel::<AppEvent>(32);
    let (manager_sender, manager_receiver) = mpsc::channel::<ManagerEvent>(32);

    // Run the TUI in a blocking task (separate thread) since it has a synchronous event loop
    let tui_handle = tokio::task::spawn_blocking(move || {
        let terminal = ratatui::init();
        let res = App::new(app_sender, manager_receiver).run(terminal);
        ratatui::restore();
        res
    });

    // Run the manager on the current task using LocalSet (Manager is not Send)
    let local = tokio::task::LocalSet::new();
    local
        .run_until(run_manager(app_receiver, manager_sender))
        .await?;

    // Wait for TUI to finish
    tui_handle.await??;

    Ok(())
}

async fn run_manager(
    mut receiver: mpsc::Receiver<AppEvent>,
    sender: mpsc::Sender<ManagerEvent>,
) -> Result<(), anyhow::Error> {
    let manager = Manager::open().await?;
    sender.send(ManagerEvent::Ready).await.ok();

    while let Some(event) = receiver.recv().await {
        match event {
            AppEvent::Quit => break,
            AppEvent::RequestPackages => {
                if let Ok(packages) = manager.list_all() {
                    sender.send(ManagerEvent::PackagesList(packages)).await.ok();
                }
            }
            AppEvent::RequestStateInfo => {
                let state_info = manager.state_info();
                sender.send(ManagerEvent::StateInfo(state_info)).await.ok();
            }
            AppEvent::Pull(reference_str) => {
                let result = match reference_str.parse::<Reference>() {
                    Ok(reference) => manager.pull(reference).await.map_err(|e| e.to_string()),
                    Err(e) => Err(format!("Invalid reference: {}", e)),
                };
                sender.send(ManagerEvent::PullResult(result)).await.ok();
                // Refresh the packages list after pull
                if let Ok(packages) = manager.list_all() {
                    sender.send(ManagerEvent::PackagesList(packages)).await.ok();
                }
            }
            AppEvent::Delete(reference_str) => {
                let result = match reference_str.parse::<Reference>() {
                    Ok(reference) => manager.delete(reference).await.map(|_| ()).map_err(|e| e.to_string()),
                    Err(e) => Err(format!("Invalid reference: {}", e)),
                };
                sender.send(ManagerEvent::DeleteResult(result)).await.ok();
                // Refresh the packages list after delete
                if let Ok(packages) = manager.list_all() {
                    sender.send(ManagerEvent::PackagesList(packages)).await.ok();
                }
            }
        }
    }

    Ok(())
}
