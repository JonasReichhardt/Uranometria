import mqtt from 'mqtt'
import EEApi from './api.js'
import Ship from './state.js'
import Events from './events.js'
import fs from 'fs'

const reactor_events = JSON.parse(fs.readFileSync('./events.json'))
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
        EEApi.setReactorHealth(1)
      }
  }
});

async function main() {
  updateState()
  setInterval(updateState, 1000)
}

async function updateState() {
  let nship = await EEApi.getShipState()
  if (nship == null) {
    console.log("[main] could not get update from server")
    return
  }

  ship.hull = nship.hull
  ship.reactor = nship.reactor
  events = checkEvents(ship)
}

function checkEvents(ship) {
  let events = []
  let event = reactor_events.filter(e => e.severity == 0)[0] //TODO randomize selection
  Events[event.name](event)
  // reactor
  if (ship.reactor[0] < 0.25) {
    let event = reactor_events.filter(e => e.severity == 1)[0] //TODO randomize selection
    Events[event.name](event)
    events.push(event)
  } else if (ship.reactor[0] < 0.5) {
    let event = reactor_events.filter(e => e.severity == 0)[0] //TODO randomize selection
    Events[event.name](event)
    events.push(event)
  }
}



main()