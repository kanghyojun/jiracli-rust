use clap::{App, Arg, SubCommand};

mod app_config;
mod http;

use app_config::{Config, JsonData, Token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Jira CLI")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("issue")
                .about("Get issue with key")
                .arg(Arg::with_name("KEY").help("Jira Issue key. e.g. JIRA-1")),
        )
        .get_matches();
    let transport = http::HttpTransport {
        config: Config::from_path()?,
        token: Token::from_path()?,
    };
    let jira_client = http::JiraClient { transport };

    if let Some(matches) = matches.subcommand_matches("issue") {
        let key = matches.value_of("KEY").unwrap();
        let resp = jira_client.get_issue(key).await?;
        println!("{:#?}", resp);
    }

    Ok(())
}
