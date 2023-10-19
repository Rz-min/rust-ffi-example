//
use anyhow::Result;

#[async_trait::async_trait]
pub trait FilterService {
    fn name(&self) -> String;
    async fn filter(&self, input: String) -> Result<()>;

    async fn create_filter(&mut self) -> Result<()>;
    async fn delete_filter(&mut self) -> Result<()>;
    async fn update_filter(&mut self) -> Result<()>;

}