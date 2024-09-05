import EEApi from './api.js'

let leakTimeout = ""

export default class Events {
    static all = [
        {
            "name": "rtorCtrlMal",
            "type": "Reactor",
            "desc": "Reactor controller malfunction",
            "severity": 0
        },
        {
            "name": "rtorCoolLeak",
            "type": "Reactor",
            "desc": "Reactor coolant leak",
            "severity": 1
        },
        {
            "name": "rtorSecFuelOut",
            "type": "Reactor",
            "desc": "Reactor secondary fuel pump outage",
            "severity": 0
        }
    ]

    static async rtorCtrlMal(event, ship,set) {
        if (set) {
            //EEApi.setAlertLevel(1) TODO fix alert level
            await EEApi.addToShipLog("[Detected] "+event.desc, 'Yellow')
            await EEApi.setReactorMaxHealth(0.5)
        }else{
            //EEApi.setAlertLevel(0) TODO fix alert level
            await EEApi.addToShipLog("[Resolved] "+event.desc, 'Yellow')
            await EEApi.setReactorMaxHealth(1)
            await EEApi.setReactorHealth(ship.reactor[0]+0.5)
        }
    }

    static async rtorCoolLeak(event, ship,set) {
        if(set){
            //EEApi.setAlertLevel(2) TODO fix alert level
            await EEApi.addToShipLog("[Detected] "+event.desc, 'Red')
            let leak = ship.coolant * 0.9
            await EEApi.setMaxCoolant(leak)
            leakTimeout = setInterval(async ()=>{
                let coolant = await EEApi.getMaxCoolant()
                await EEApi.setMaxCoolant(coolant*0.9)
            },5000)
        }else{
            //EEApi.setAlertLevel(0) TODO fix alert level
            await EEApi.addToShipLog("[Resolved] "+event.desc, 'Red')
            clearInterval(leakTimeout)
            await EEApi.setMaxCoolant(10.0)
            await EEApi.setReactorHealth(ship.reactor[0]+0.5)
        }
        
    }

    static rtorSecFuelOut(event) {
        
    }
}