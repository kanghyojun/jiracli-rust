use std::error::Error;

use reqwest::{Client, Response, Url};

use crate::app_config::{Config, Token};

pub struct JiraClient {
    pub config: Config,
    pub token: Token,
}

impl JiraClient {
    fn base_url(&self) -> String {
        String::from(format!(
            "https://api.atlassian.com/ex/jira/{}/rest/api/3/",
            self.config.company_id
        ))
    }

    async fn req_get(&self, path: &str) -> Result<Response, Box<dyn Error>> {
        let client = Client::new();
        let url = Url::parse(&self.base_url())?.join(path)?;
        let resp = client
            .get(url.as_str())
            .bearer_auth(&self.token.access_token)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_issue(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let resp = self
            .req_get(&format!("./issue/{}", key))
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}
