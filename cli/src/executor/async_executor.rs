use std::io::{self, Write};

use clap::Parser;
use shared::{ShoppingListRepository, repository::EmailRepository};
use tokio::sync::mpsc;

use crate::{
    commands::{cli::Cli, handler::handle_command},
    credentials::Credentials,
};

pub async fn run_async_executor<R, E>(
    mut rx: mpsc::Receiver<String>,
    api_client: &R,
    email_client: &E,
    creds: &mut Credentials,
) -> anyhow::Result<()>
where
    R: ShoppingListRepository,
    E: EmailRepository,
{
    // Print the first prompt
    print!("shopping-list> ");
    io::stdout().flush().unwrap();

    while let Some(line) = rx.recv().await {
        let parts = shlex::split(&line).unwrap_or_default();
        let args = std::iter::once("repl").chain(parts.iter().map(|part| part.as_str()));
        match Cli::try_parse_from(args) {
            Ok(cmd) => {
                if let Some(page) = handle_command(api_client, email_client, creds, cmd).await? {
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
