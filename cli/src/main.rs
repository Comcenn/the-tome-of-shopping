use cli::{
    api::{api_client::ShoppingListClient, email_client::EmailClient},
    channel,
    credentials::{Credentials, prompt_for_credentials},
    executor::{async_executor::run_async_executor, create_runtime},
    interface::spawn_repl_thread,
};

fn main() -> anyhow::Result<()> {
    let async_runtime = create_runtime();
    let (tx, rx) = channel();

    // Load or prompt for credentials
    let mut creds = match Credentials::load()? {
        Some(c) => c,
        None => {
            println!("No credentials found — please log in.");
            let creds = prompt_for_credentials()?;
            creds.save()?;
            creds
        }
    };

    let api_client = ShoppingListClient::build("http://localhost:3000")?;
    let email_client = EmailClient;

    spawn_repl_thread(tx);

    async_runtime.block_on(run_async_executor(
        rx,
        &api_client,
        &email_client,
        &mut creds,
    ))?;

    Ok(())
}
