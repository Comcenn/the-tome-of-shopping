pub mod async_executor;

pub fn create_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("failed to create async runtime")
}
