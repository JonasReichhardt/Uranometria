use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::model::{Ship, ShipDto};

#[derive(Deserialize, Debug)]
pub struct EEApi {
    pub state: String,
}

impl EEApi {
    pub fn new(address: &str) -> Self {
        let path = PathBuf::from("./requests.json");
        let fcont = fs::read_to_string(&path).expect("[MAIN] could not read file");
        println!(
            "[CONFIG] loaded {}",
            path.file_name().unwrap().to_str().unwrap()
        );
        let mut api: EEApi =
            serde_json::from_str(&fcont).expect("[CONFIG] could not parse settings");
        let mut temp = String::new();
        temp.push_str(address);
        temp.push_str(&api.state);
        api.state = temp;
        return api;
    }

    pub async fn get_ship(&self) -> Result<Ship, reqwest::Error> {
        println!("{}", self.state);
        Ok(Ship::from(
            reqwest::get(&self.state).await?.json::<ShipDto>().await?,
        ))
    }
}
