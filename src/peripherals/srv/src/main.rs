use api::EEApi;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tokio::task;

pub mod api;
pub mod config;
pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rtor_events = config::load_events("rtor");
    let api = EEApi::new("http://localhost:8080/");
    println!("{:?}", api.get_ship().await.unwrap());
    Ok(())
}

async fn mqtt_create_subscribe() -> Result<AsyncClient, String> {
    let mut mqttoptions = MqttOptions::new("ura-serv", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("Reactor", QoS::AtMostOnce).await.unwrap();
    task::spawn(async move { handle_mqtt_msg(eventloop) });

    Ok(client)
}

async fn handle_mqtt_msg(mut ev_loop: EventLoop) {
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
