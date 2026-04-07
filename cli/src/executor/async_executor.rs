use std::io::{self, Write};

use clap::Parser;
use shared::ShoppingListRepository;
use tokio::sync::mpsc;

use crate::commands::{cli::Cli, handler::handle_command};

pub async fn run_async_executor<R: ShoppingListRepository>(
    mut rx: mpsc::Receiver<String>,
    client: &R,
) -> anyhow::Result<()> {
    // Print the first prompt
    print!("shopping-list> ");
    io::stdout().flush().unwrap();

    while let Some(line) = rx.recv().await {
        let parts = shlex::split(&line).unwrap_or_default();
        let args = std::iter::once("repl").chain(parts.iter().map(|part| part.as_str()));
        match Cli::try_parse_from(args) {
            Ok(cmd) => {
                if let Some(page) = handle_command(client, cmd).await? {
                    println!("{}", page.render());
                    println!();
                }
            }
            Err(e) => eprintln!("{e}"),
        }
        print!("shopping-list> ");
        io::stdout().flush().unwrap();
    }
    Ok(())
}
