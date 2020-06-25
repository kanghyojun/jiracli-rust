use std::error::Error;

mod http;
mod app_config;

use http::JiraClient;
use app_config::{Config, Token, JsonData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let jira_client = JiraClient {
        config: Config::from_path()?,
        token: Token::from_path()?,
    };
    let resp = jira_client.get_issue("SOA-402").await?;

    println!("{:#?}", resp);
    Ok(())
}
