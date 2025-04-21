use api::EEApi;
use futures::lock::Mutex;
use model::{Ship, SubsystemEvent};
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

pub mod api;
pub mod config;
pub mod model;

struct State {
    api: EEApi,
    ship: Mutex<Option<Ship>>,
    events: Vec<SubsystemEvent>,
    active_events: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rtor_events = config::load_events("rtor");
    let api = EEApi::new("http://localhost:8080/");
    let ship: Option<Ship> = None;
    let state = State {
        api,
        ship: Mutex::new(ship),
        events: rtor_events,
        active_events: Mutex::new(Vec::new()),
    };

    Ok(())
}

async fn update_events(state: Arc<State>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.tick().await;
    loop {
        if let Ok(ship) = state.api.get_ship().await {
            let mut ship_lock = state.ship.lock().await;
            // TODO add event creation logic
            ship_lock.replace(ship);
        }
    }
}

async fn mqtt_create_subscribe(state: Arc<State>) -> Result<AsyncClient, String> {
    let mut mqttoptions = MqttOptions::new("ura-serv", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("Reactor", QoS::AtMostOnce).await.unwrap();
    task::spawn(async move { handle_mqtt_msg(eventloop, state) });

    Ok(client)
}

async fn handle_mqtt_msg(mut ev_loop: EventLoop, state: Arc<State>) {
    loop {
        match ev_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
            }
            Ok(Event::Incoming(i)) => {
                println!("Incoming = {i:?}");
            }
            Ok(Event::Outgoing(o)) => println!("Outgoing = {o:?}"),
            Err(e) => {
                println!("Error = {e:?}");
            }
        }
    }
}
