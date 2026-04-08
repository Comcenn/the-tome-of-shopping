use std::io;

use tokio::sync::mpsc;

use crate::credentials::extract_user;

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

        if line.starts_with("login") {
            if let Some(user) = extract_user(&line) {
                let password = rpassword::prompt_password("Password: ").unwrap();
                let full = format!("login --username {} --password {}", user, password);
                tx.blocking_send(full).unwrap();
            }
        } else {
            tx.blocking_send(line.to_string()).unwrap();
        }

    }
}
