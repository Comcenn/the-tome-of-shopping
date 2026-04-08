use cli::{
    api::{api_client::ShoppingListClient, email_client::EmailClient},
    channel,
    executor::{async_executor::run_async_executor, create_runtime},
    interface::spawn_repl_thread,
};

fn main() -> anyhow::Result<()> {
    let async_runtime = create_runtime();
    let (tx, rx) = channel();

    let api_client = ShoppingListClient::build("http://localhost:3000")?;
    let email_client = EmailClient;

    // run repl loop in a thread
    spawn_repl_thread(tx);

    async_runtime.block_on(run_async_executor(rx, &api_client, &email_client))?;

    Ok(())
}
