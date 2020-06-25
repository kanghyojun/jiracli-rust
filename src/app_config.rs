use std::error::Error;
use std::fs;
use std::path::PathBuf;

use dirs::{config_dir, data_dir};
use serde::{Deserialize, Serialize};

const DIR: &'static str = "jiraoauth";

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub company_id: String,
}

pub trait JsonData: Sized {
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
    fn from_path() -> Result<Token, Box<dyn Error>> {
        let reader = JsonReader {
            base_path: data_dir().unwrap(),
            fname: "token.json",
        };
        let d: Token = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}

impl JsonData for Config {
    fn from_path() -> Result<Config, Box<dyn Error>> {
        let reader = JsonReader {
            base_path: config_dir().unwrap(),
            fname: "config.json",
        };
        let d: Config = serde_json::from_str(&reader.to_str()?)?;

        Ok(d)
    }
}
