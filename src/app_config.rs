use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use dirs::{config_dir, data_dir};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

const DIR: &str = "jiraoauth";

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub company_id: String,
    pub client_id: String,
    pub client_secret: String,
}

pub struct JsonReader {
    base_path: PathBuf,
    fname: &'static str,
}

impl JsonReader {
    fn to_str(&self) -> Result<String> {
        let raw = fs::read_to_string(self.file_path())?;

        Ok(raw)
    }

    fn file_path(&self) -> PathBuf {
        self.base_path.join(DIR).join(self.fname)
    }
}

pub trait JsonData {
    fn from_path() -> Result<Self>
    where
        Self: Sized;

    fn json_reader() -> JsonReader;

    fn save(&self) -> Result<()>
    where
        Self: DeserializeOwned;
}

impl JsonData {
    fn save(&self) -> Result<()> {
        let raw = serde_json::to_string(&self)?;
        let r = fs::write(Self::json_reader().file_path(), raw)?;

        Ok(r)
    }
}

impl JsonData for Token {
    fn json_reader() -> JsonReader {
        JsonReader {
            base_path: data_dir().unwrap(),
            fname: "token.json",
        }
    }

    fn from_path() -> Result<Token> {
        let reader = Token::json_reader();
        let d: Token = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}

impl JsonData for Config {
    fn json_reader() -> JsonReader {
        JsonReader {
            base_path: config_dir().unwrap(),
            fname: "config.json",
        }
    }

    fn from_path() -> Result<Config> {
        let reader = Config::json_reader();
        let d: Config = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}

impl Config {
    pub fn save(&self) -> Result<()> {}
}
