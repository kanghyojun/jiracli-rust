use std::error::Error;

mod http;
mod app_config;

use app_config::{Config, Token, JsonData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let transport = http::HttpTransport {
        config: Config::from_path()?,
        token: Token::from_path()?,
    };
    let jira_client = http::JiraClient { transport: transport, };
    let resp = jira_client.get_issue("SOA-402").await?;

    println!("{:#?}", resp);
    Ok(())
}
