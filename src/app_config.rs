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

pub trait JsonReadable {
    fn reader() -> JsonReader;
}

impl JsonReadable for Token {
    fn reader() -> JsonReader {
        JsonReader {
            base_path: data_dir().unwrap(),
            fname: "token.json",
        }
    }
}

impl JsonReadable for Config {
    fn reader() -> JsonReader {
        JsonReader {
            base_path: config_dir().unwrap(),
            fname: "config.json",
        }
    }
}

pub fn from_path<T>() -> Result<T>
where
    T: JsonReadable + DeserializeOwned
{
    let reader = T::reader();
    let d: T = serde_json::from_str(&reader.to_str()?)?;

    Ok(d)
}

pub fn save<T>(payload: T) -> Result<()>
where
    T: JsonReadable + Serialize
{
    let raw = serde_json::to_string(&payload)?;
    let r = fs::write(T::reader().file_path(), raw)?;

    Ok(r)
}
