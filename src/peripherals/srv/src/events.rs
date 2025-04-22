use crate::State;
use crate::model::{Severity, SubsystemEvent};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

pub async fn exec_event(event: &SubsystemEvent, state: &Arc<State>, inactive: bool) {
    match &event.name[..] {
        "rtorCtrlMal" => rtor_ctrl_malfunc(event, state, inactive).await,
        "rtorSecFuelOut" => rtor_sec_fuel_out(state, inactive).await,
        "rtorCoolLeak" => rtor_cool_leak(state, inactive).await,
        _ => {
            println!("[EVENTS] Cannot execute {}", event.name)
        }
    }
    let msg;
    if inactive {
        msg = format!("Detected {}", event.desc);
        let (log_color, alert_color) = convert_to_ee_severity(&event.severity);
        let _ = state.api.update_ship_log(&msg, log_color).await;
        let _ = state.api.set_alert_level(alert_color).await;
    } else {
        msg = format!("Resolved {}", event.desc);
        let _ = state.api.update_ship_log(&msg, "Gray").await;
        let _ = state.api.set_alert_level("Normal").await;
    }
    println!("{}", msg);
}

fn convert_to_ee_severity(severity: &Severity) -> (&'static str, &'static str) {
    match severity {
        Severity::LOW => ("Normal", "Normal"),
        Severity::MED => ("Yellow", "YELLOW ALERT"),
        Severity::HIGH => ("Red", "RED ALERT"),
    }
}

async fn rtor_cool_leak(state: &Arc<State>, inactive: bool) {
    if inactive {
        let state_ref = state.clone();
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            interval.tick().await;
            loop {
                // Use scopes to drop the locks after using them
                {
                    let active_event_lock = state_ref.events.0.lock().await;
                    if active_event_lock.is_none() {
                        break;
                    }
                }
                {
                    let mut ship_lock = state_ref.ship.lock().await;
                    let new_cool = ship_lock.as_mut().unwrap().coolant * 0.9;
                    let _ = state_ref.api.set_max_coolant(new_cool).await;
                    println!("Reduced coolant to {}", new_cool);
                }
                interval.tick().await;
            }
            println!("[EVENTS] Stopped leak loop");
        });
    } else {
        let lock = state.ship.lock().await;
        let _ = state
            .api
            .set_subsystem_health(
                "reactor",
                lock.as_ref().unwrap().reactor.health + 0.5,
                false,
            )
            .await;
    }
}
async fn rtor_sec_fuel_out(state: &Arc<State>, inactive: bool) {
    if inactive {
        let _ = state.api.set_power_factor("reactor", -20.0).await;
    } else {
        let _ = state.api.set_power_factor("reactor", -25.0).await;
    }
}

async fn rtor_ctrl_malfunc(event: &SubsystemEvent, state: &Arc<State>, inactive: bool) {
    if inactive {
        let _ = state.api.set_subsystem_health("reactor", 0.5, true).await;
    } else {
        let msg = format!("Resolved {}", event.desc);
        let _ = state.api.update_ship_log(&msg, "Yellow").await;
        let _ = state.api.set_subsystem_health("reactor", 1.0, true).await;
        let lock = state.ship.lock().await;
        let _ = state
            .api
            .set_subsystem_health(
                "reactor",
                lock.as_ref().unwrap().reactor.health + 0.5,
                false,
            )
            .await;
    }
}
