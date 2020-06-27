mod app_config;
mod http;

use app_config::{Config, JsonData, Token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = http::HttpTransport {
        config: Config::from_path()?,
        token: Token::from_path()?,
    };
    let jira_client = http::JiraClient { transport };
    let resp = jira_client.get_issue("SOA-402").await?;

    println!("{:#?}", resp);
    Ok(())
}
