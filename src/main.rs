use clap::{App, Arg, SubCommand};

mod app_config;
mod http;

use app_config::{from_path, save, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Jira CLI")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("issue")
                .about("Get issue with key")
                .arg(
                    Arg::with_name("KEY")
                        .help("Jira Issue key. e.g. JIRA-1")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("authorize").about("Authorize your jira"))
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("client_id")
                        .long("client-id")
                        .value_name("CLIENT_ID")
                        .help("Jira OAuth client ID")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("client_secret")
                        .long("client-secret")
                        .value_name("CLIENT_SECRET")
                        .help("Jira OAuth client secret")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("company_id")
                        .long("company-id")
                        .value_name("COMPANY_ID")
                        .help("Jira atlassian company id")
                        .required(true)
                        .takes_value(true),
                )
                .about("Initialize your jira app configuration."),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("issue") {
        let transport = http::HttpTransport {
            config: from_path()?,
            token: from_path()?,
        };
        let jira_client = http::JiraClient { transport };
        let key = matches.value_of("KEY").unwrap();
        let resp = jira_client.get_issue(key).await?;
        println!("{:#?}", resp);
    } else if let Some(_) = matches.subcommand_matches("authorize") {
        println!("TODO: authorized ...");
    } else if let Some(matches) = matches.subcommand_matches("init") {
        let conf = Config {
            client_id: matches.value_of("client_id").unwrap().to_string(),
            client_secret: matches.value_of("client_secret").unwrap().to_string(),
            company_id: matches.value_of("company_id").unwrap().to_string(),
        };

        println!("Save config...");

        match save(conf) {
            Ok(_) => println!("Done!"),
            Err(_) => println!("Error occured, Try agian."),
        }
    }

    Ok(())
}
