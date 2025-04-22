use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

use crate::model::{Ship, ShipDto};

#[derive(Deserialize, Debug)]
pub struct EEApi {
    pub ee_http_server: String,
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
        api
    }

    pub async fn get_ship(&self) -> Result<Ship, reqwest::Error> {
        Ok(Ship::from(
            reqwest::get(&self.state).await?.json::<ShipDto>().await?,
        ))
    }

    pub async fn set_alert_level(&self, level: &str) -> Result<(), ()> {
        let client = Client::new();
        if let Ok(res) = client
            .post(format!("{}exec.lua", &self.ee_http_server))
            .body(format!(
                "return getPlayerShip(-1):commandSetAlertLevel('{}')",
                level
            ))
            .send()
            .await
        {
            match res.error_for_status() {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }

    pub async fn set_subsystem_health(
        &self,
        system_name: &str,
        health: f32,
        set_max: bool,
    ) -> Result<(), ()> {
        let client = Client::new();
        let max_str = if set_max { "Max" } else { "" };
        if let Ok(res) = client
            .post(format!("{}exec.lua", &self.ee_http_server))
            .body(format!(
                "return getPlayerShip(-1):setSystemHealth{}('{}',{})",
                max_str, system_name, health
            ))
            .send()
            .await
        {
            match res.error_for_status() {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }

    pub async fn get_max_coolant(&self) -> Result<f32, ()> {
        let key = "cool";
        match reqwest::get(format!(
            "{}get.lua?{}=getMaxCoolant()",
            self.ee_http_server, key
        ))
        .await
        {
            Ok(reps) => {
                let json = reps.json::<HashMap<String, String>>().await.unwrap();
                if let Some(val) = json.get(key) {
                    Ok(val.parse::<f32>().unwrap())
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }

    pub async fn set_power_factor(&self, subsystem: &str, factor: f32) -> Result<(), ()> {
        let client = Client::new();
        if let Ok(res) = client
            .post(format!("{}exec.lua", &self.ee_http_server))
            .body(format!(
                "return getPlayerShip(-1):setSystemPowerFactor('{}',{})",
                subsystem, factor
            ))
            .send()
            .await
        {
            match res.error_for_status() {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }

    pub async fn get_power_factor(&self, subsystem: &str) -> Result<f32, ()> {
        let key = "pwr";
        match reqwest::get(format!(
            "{}get.lua?{}=getSystemPowerFactor('{}')",
            self.ee_http_server, key, subsystem
        ))
        .await
        {
            Ok(reps) => {
                let json = reps.json::<HashMap<String, f32>>().await.unwrap();
                Ok(json.get(key).unwrap().to_owned())
            }
            Err(_) => Err(()),
        }
    }

    pub async fn set_max_coolant(&self, coolant: f32) -> Result<(), ()> {
        let client = Client::new();
        if let Ok(res) = client
            .post(format!("{}exec.lua", &self.ee_http_server))
            .body(format!(
                "return getPlayerShip(-1):setMaxCoolant({})",
                coolant
            ))
            .send()
            .await
        {
            match res.error_for_status() {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }

    pub async fn update_ship_log(&self, msg: &str, color: &str) -> Result<(), ()> {
        let client = Client::new();
        if let Ok(res) = client
            .post(format!("{}exec.lua", &self.ee_http_server))
            .body(format!(
                "return getPlayerShip(-1):addToShipLog('{}','{}')",
                msg, color
            ))
            .send()
            .await
        {
            match res.error_for_status() {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }
}
