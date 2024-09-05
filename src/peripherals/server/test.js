import EEApi from './api.js'

var state = await EEApi.getShipState()
console.log(state)
var l = state.coolant*0.5
console.log(l)
EEApi.setMaxCoolant(l)
setTimeout(async()=> {console.log(await EEApi.getShipState());EEApi.setMaxCoolant(10.0)},1000)
