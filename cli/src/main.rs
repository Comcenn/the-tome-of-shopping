use cli::{
    api::client::ShoppingListClient,
    channel,
    executor::{async_executor::run_async_executor, create_runtime},
    interface::spawn_repl_thread,
};

fn main() -> anyhow::Result<()> {
    let async_runtime = create_runtime();
    let (tx, rx) = channel();

    let client = ShoppingListClient::build("http://localhost:3000")?;

    // run repl loop in a thread
    spawn_repl_thread(tx);

    async_runtime.block_on(run_async_executor(rx, &client))?;

    Ok(())
}
