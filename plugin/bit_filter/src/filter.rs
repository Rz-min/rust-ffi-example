//
use anyhow::Result;
use lib::FilterService;

pub struct BitCoinFilter {
    name: String,

}

impl BitCoinFilter  {
    pub fn new() -> Self {
        Self {
            name: "BitCoinFilter".into(),
        }
    
    }
}

#[async_trait::async_trait]
impl FilterService for BitCoinFilter  {
    fn name(&self) -> String {
        "BitCoinFilter".into()
    }

    async fn filter(&self, input: String) ->  Result<()> {
        Ok(())
    }

    async fn create_filter(&mut self) -> Result<()> {
        Ok(())
    }

    async fn delete_filter(&mut self) -> Result<()> {
        Ok(())
    }
    
    async fn update_filter(&mut self) -> Result<()> {
        Ok(())
    }
}