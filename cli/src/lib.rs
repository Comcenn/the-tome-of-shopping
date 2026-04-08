use tokio::sync::mpsc;

pub mod api;
pub mod commands;
pub mod credentials;
pub mod executor;
pub mod interface;

pub fn channel() -> (mpsc::Sender<String>, mpsc::Receiver<String>) {
    mpsc::channel(32)
}
