use serde::Deserialize;

#[derive(Debug)]
pub struct Ship {
    hull: f32,
    coolant: f32,
    reactor: Subsystem,
}

#[derive(Debug)]
pub struct Subsystem {
    health: f32,
    heat: f32,
    power: f32,
    coolant: f32,
    max_health: f32,
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
