use anyhow::Ok;
use async_trait::async_trait;
use shared::repository::EmailRepository;

pub struct EmailClient;

#[async_trait]
impl EmailRepository for EmailClient {
    async fn send_email(&self, _email: &str, _message: String) -> anyhow::Result<()> {
        Ok(())
    }
}
