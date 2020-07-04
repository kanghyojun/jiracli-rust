mod app_config;
mod http;

use structopt::StructOpt;

use app_config::{from_path, save, Config};

#[derive(Debug, StructOpt)]
#[structopt(name = "jiracli-rust", about = "Jira CLI")]
enum Opt {
    Issue {
        #[structopt(name = "KEY")]
        key: String,
    },
    Authorize {},
    Init {
        #[structopt(long)]
        client_id: String,

        #[structopt(long)]
        client_secret: String,

        #[structopt(long)]
        company_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Issue { key } => {
            let transport = http::HttpTransport {
                config: from_path()?,
                token: from_path()?,
            };
            let jira_client = http::JiraClient { transport };
            let resp = jira_client.get_issue(&key).await?;
            println!("{:#?}", resp);
        }
        Opt::Authorize {} => {
            println!("TODO: authorized ...");
        }
        Opt::Init {
            client_id,
            client_secret,
            company_id,
        } => {
            let conf = Config {
                client_id,
                client_secret,
                company_id,
            };

            println!("Save config...");

            match save(conf) {
                Ok(_) => println!("Done!"),
                Err(_) => println!("Error occured, Try agian."),
            }
        }
    }

    Ok(())
}
