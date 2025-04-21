use serde::Deserialize;

#[derive(Debug)]
pub struct Ship {
    pub hull: f32,
    pub coolant: f32,
    pub reactor: Subsystem,
}

#[derive(Debug)]
pub struct Subsystem {
    pub health: f32,
    pub heat: f32,
    pub power: f32,
    pub coolant: f32,
    pub max_health: f32,
}

#[derive(Deserialize, Debug)]
pub struct SubsystemEvent {
    name: String,
    desc: String,
    severity: Severity,
}

#[derive(Deserialize, Debug)]
pub enum Severity {
    LOW,
    MED,
    HIGH,
}

#[derive(Deserialize, Debug)]
pub struct ShipDto {
    hull: f32,
    cool: f32,
    rhealth: f32,
    rheat: f32,
    rpwr: f32,
    rcool: f32,
    rmhealth: f32,
}

impl From<ShipDto> for Ship {
    fn from(value: ShipDto) -> Self {
        Ship {
            hull: value.hull,
            coolant: value.cool,
            reactor: Subsystem {
                health: value.rhealth,
                heat: value.rheat,
                power: value.rpwr,
                coolant: value.rcool,
                max_health: value.rmhealth,
            },
        }
    }
}
