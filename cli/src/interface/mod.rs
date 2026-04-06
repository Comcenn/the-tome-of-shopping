use tokio::sync::mpsc;

use crate::interface::repl::repl_loop;

pub mod repl;

pub fn spawn_repl_thread(tx: mpsc::Sender<String>) {
    std::thread::spawn(move || repl_loop(tx));
}
