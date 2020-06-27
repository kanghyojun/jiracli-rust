use std::error::Error;

use async_trait::async_trait;
use reqwest::{Client, Url};

use crate::app_config::{Config, Token};

#[async_trait]
pub trait Transport {
    fn base_url(&self) -> String;

    async fn req_get(&self, path: &str) -> Result<String, Box<dyn Error>>;
}

pub struct HttpTransport {
    pub config: Config,

    pub token: Token,
}

#[async_trait]
impl Transport for HttpTransport {
    fn base_url(&self) -> String {
        format!(
            "https://api.atlassian.com/ex/jira/{}/rest/api/3/",
            self.config.company_id
        )
    }

    async fn req_get(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let url = Url::parse(&self.base_url())?.join(path)?;
        let resp = client
            .get(url.as_str())
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}

pub struct JiraClient<T>
where
    T: Transport,
{
    pub transport: T,
}

impl<T: Transport> JiraClient<T> {
    pub async fn get_issue(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let resp = self.transport.req_get(&format!("./issue/{}", key)).await?;

        Ok(resp)
    }
}
