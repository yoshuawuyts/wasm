mod app;

use app::App;
use tokio::sync::mpsc;
use wasm_package_manager::Manager;

/// Events sent from the TUI to the Manager
#[derive(Debug)]
pub enum AppEvent {
    /// Request to quit the application
    Quit,
}

/// Events sent from the Manager to the TUI
#[derive(Debug)]
pub enum ManagerEvent {
    /// Manager has finished initializing
    Ready,
}

pub async fn run() -> anyhow::Result<()> {
    // Create channels for bidirectional communication
    let (app_sender, app_receiver) = mpsc::channel::<AppEvent>(32);
    let (manager_sender, manager_receiver) = mpsc::channel::<ManagerEvent>(32);

    // Spawn the Manager task
    let manager_handle = tokio::spawn(run_manager(app_receiver, manager_sender));

    // Run the TUI on the main thread
    let terminal = ratatui::init();
    let res = App::new(app_sender, manager_receiver).run(terminal);
    ratatui::restore();
    res?;

    // Wait for the manager task to finish
    manager_handle.await??;

    Ok(())
}

async fn run_manager(
    mut receiver: mpsc::Receiver<AppEvent>,
    sender: mpsc::Sender<ManagerEvent>,
) -> Result<(), anyhow::Error> {
    let _manager = Manager::open().await?;
    sender.send(ManagerEvent::Ready).await.ok();

    while let Some(event) = receiver.recv().await {
        match event {
            AppEvent::Quit => break,
        }
    }

    Ok(())
}
