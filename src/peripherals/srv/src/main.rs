use api::EEApi;
use futures::lock::Mutex;
use model::{Ship, SubsystemEvent};
use rand::Rng;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::sync::Arc;
use std::time::Duration;
use tokio::{fs, task};

pub mod api;
pub mod config;
pub mod events;
pub mod model;

pub struct State {
    api: EEApi,
    ship: Mutex<Option<Ship>>,
    events: (Mutex<Option<usize>>, Vec<SubsystemEvent>),
}

#[tokio::main]
async fn main() {
    let rtor_events = config::load_events("rtor");
    let active_event = config::load_active_event();

    let api = EEApi::new("http://localhost:8080/");
    let ship: Option<Ship> = None;
    let state = Arc::new(State {
        api,
        ship: Mutex::new(ship),
        events: (Mutex::new(active_event), rtor_events),
    });
    let state_arc = state.clone();

    let mqtt_client = Arc::new(mqtt_subscribe(state.clone()).await);
    restart_event(state, mqtt_client.clone(), active_event).await;
    let handle =
        task::spawn(
            async move { update_events(state_arc, mqtt_client, Duration::from_secs(1)).await },
        );

    let _ = tokio::join!(handle);
}

async fn restart_event(state: Arc<State>, mqtt_client: Arc<AsyncClient>, idx: Option<usize>) {
    if let Some(event_idx) = idx {
        let random_event = &state.events.1[event_idx];
        events::exec_event(random_event, &state, true).await;
        let _ = mqtt_client
            .publish("Reactor", QoS::AtLeastOnce, false, event_idx.to_string())
            .await;
    }
}

async fn update_events(state: Arc<State>, mqtt_client: Arc<AsyncClient>, update_rate: Duration) {
    let mut interval = tokio::time::interval(update_rate);
    interval.tick().await;
    loop {
        if let Ok(ship) = state.api.get_ship().await {
            let mut ship_lock = state.ship.lock().await;
            ship_lock.replace(ship);
            let new_ship = ship_lock.as_mut().expect("WTF");
            let mut rtor_lock = state.events.0.lock().await;

            if new_ship.reactor.health < 0.5 && rtor_lock.is_none() {
                let event_idx = rand::rng().random_range(0..state.events.1.len());
                rtor_lock.replace(event_idx);
                fs::write("./persistence", event_idx.to_string())
                    .await
                    .expect("[MAIN] Persistence ");
                let random_event = &state.events.1[event_idx];
                events::exec_event(random_event, &state, true).await;
                let _ = mqtt_client
                    .publish("Reactor", QoS::AtLeastOnce, false, event_idx.to_string())
                    .await;
            }
        }
        interval.tick().await;
    }
}

async fn mqtt_subscribe(state: Arc<State>) -> AsyncClient {
    let mut mqttoptions = MqttOptions::new("ura-serv", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("Reactor", QoS::AtMostOnce)
        .await
        .expect("[MAIN] Could not subscribe to topic");
    task::spawn(async move { handle_mqtt_msg(eventloop, state).await });

    client
}

async fn handle_mqtt_msg(mut ev_loop: EventLoop, state: Arc<State>) {
    loop {
        match ev_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                if p.topic == "Reactor" && p.payload == "Repair" {
                    let mut rtor_lock = state.events.0.lock().await;
                    if rtor_lock.is_some() {
                        events::exec_event(&state.events.1[rtor_lock.unwrap()], &state, false)
                            .await;
                        rtor_lock.take();
                        fs::write("./persistence", "None")
                            .await
                            .expect("[MAIN] Persistence ");
                    }
                }
            }
            Ok(Event::Incoming(_)) => {}
            Ok(Event::Outgoing(_)) => {}
            Err(e) => {
                println!("[MQTT] Error {e:?}");
            }
        }
    }
}
