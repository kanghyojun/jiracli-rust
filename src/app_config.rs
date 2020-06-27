use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use dirs::{config_dir, data_dir};
use serde::{Deserialize, Serialize};

const DIR: &str = "jiraoauth";

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub company_id: String,
}

pub trait JsonData {
    fn from_path() -> Result<Self>
    where
        Self: Sized;
}

trait ReadAppDir {
    fn read_from_appdir_to_string(&self, fname: &'static str) -> Result<String>;
}

impl ReadAppDir for PathBuf {
    fn read_from_appdir_to_string(&self, fname: &'static str) -> Result<String> {
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
    fn to_str(&self) -> Result<String> {
        let raw = self.base_path.read_from_appdir_to_string(self.fname)?;

        Ok(raw)
    }
}

impl JsonData for Token {
    fn from_path() -> Result<Token> {
        let reader = JsonReader {
            base_path: data_dir().unwrap(),
            fname: "token.json",
        };
        let d: Token = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}

impl JsonData for Config {
    fn from_path() -> Result<Config> {
        let reader = JsonReader {
            base_path: config_dir().unwrap(),
            fname: "config.json",
        };
        let d: Config = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}
