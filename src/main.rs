use std::error::Error;
use std::fs;
use std::path::PathBuf;

use dirs::{config_dir, data_dir};
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};

const DIR: &'static str = "jiraoauth";

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    company_id: String,
}

trait JsonData {}

impl JsonData
where Self: Sized {
    fn from_path() -> Result<Self, Box<dyn Error>>;
}

trait ReadAppDir {
    fn read_from_appdir_to_string(&self, fname: &'static str) -> Result<String, Box<dyn Error>>;
}

impl ReadAppDir for PathBuf {
    fn read_from_appdir_to_string(&self, fname: &'static str) -> Result<String, Box<dyn Error>> {
        let path = self.join(DIR).join(fname);
        let raw = fs::read_to_string(&path)?;

        Ok(raw)
    }
}

struct JsonReader {
    base_path: PathBuf,
    fname: &'static str,
}

impl JsonReader {
    fn to_str(&self) -> Result<String, Box<dyn Error>> {
        let raw = self.base_path.read_from_appdir_to_string(self.fname)?;

        Ok(raw)
    }
}

impl JsonData for Token {
    fn from_path() -> Result<Box<Token>, Box<dyn Error>> {
        let reader = JsonReader {
            base_path: data_dir().unwrap(),
            fname: "token.json",
        };
        let d: Token = serde_json::from_str(&reader.to_str()?)?;

        Ok(Box::new(d))
    }
}

impl JsonData for Config {
    fn from_path() -> Result<Box<Config>, Box<dyn Error>> {
        let reader = JsonReader {
            base_path: config_dir().unwrap(),
            fname: "config.json",
        };
        let d: Config = serde_json::from_str(&reader.to_str()?)?;

        Ok(Box::new(d))
    }
}

struct JiraClient {
    config: Config,
    token: Token,
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

    async fn get_issue(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let resp = self
            .req_get(&format!("./issue/{}", key))
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}

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
