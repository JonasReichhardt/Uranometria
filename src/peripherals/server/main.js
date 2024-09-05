import mqtt from 'mqtt'
import EEApi from './api.js'
import Ship from './state.js'
import Events from './events.js'

const mqcl = mqtt.connect("mqtt://127.0.0.1:1883")

const topics = ["Reactor"]
let events = []
let ship = new Ship()

mqcl.on("connect", () => {
  mqcl.subscribe(topics[0], (err) => {
    if (err) { return; }
    console.log("SUB " + topics[0])
  });
});

mqcl.on("message", (topic, message) => {
  switch (topic) {
    case "Reactor":
      let data = JSON.parse('' + message);
      if (data.Reactor == "Repair") {
        for(var i in events){
          if(events[i].type === "Reactor"){
            Events[events[i].name](events[i],ship,false)
            console.log("[main] resolved "+events[i].name)
          }
          events = events.filter(e=>e.type!="Reactor")
        }
      }
  }
});

async function main() {
  EEApi.setReactorHealth(0.2)
  await updateState()
  setInterval(updateState, 1000)
}

async function updateState() {
  let nship = await EEApi.getShipState()
  if (nship == null) {
    console.log("[main] could not get update from server")
    return
  }
  ship = nship
  checkEvents(ship)
}

function checkEvents(ship) {
  /*let event = Events.all[Events.all.length * Math.random() | 0]
  Events[event.name](event)
  events.push(event)*/

  // reactor
  if (ship.reactor[0] < 0.25 && events.filter(e=>e.type === "Reactor").length ===0) {
    //let event = Events.all[Events.all.length * Math.random() | 0]
    let event = Events.all[1]
    events.push(event)
    Events[event.name](event,ship,true)
    console.log("[main] detected "+event.name)
  }
}

main()