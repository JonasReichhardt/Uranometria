use crate::model::SubsystemEvent;
use std::fs;
use std::path::PathBuf;

pub fn load_events(module_name: &str) -> Vec<SubsystemEvent> {
    let path = PathBuf::from("./".to_string() + module_name + "_events.json");

    let fcont = fs::read_to_string(&path).expect("[CONFIG] could not read file");
    println!(
        "[CONFIG] loaded {}",
        path.file_name().unwrap().to_str().unwrap()
    );
    serde_json::from_str(&fcont).expect("[CONFIG] could not parse settings")
}

pub fn load_active_event() -> Option<usize> {
    let fcont = fs::read_to_string("./persistence").expect("[CONFIG] could not read file");
    fcont.trim().parse::<usize>().ok()
}
