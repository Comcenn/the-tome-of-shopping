use std::io;

use tokio::sync::mpsc;

pub fn repl_loop(tx: mpsc::Sender<String>) {
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }

        let line = line.trim();
        if line == "exit" || line == "quit" {
            break;
        }

        tx.blocking_send(line.to_string()).unwrap();
    }
}
