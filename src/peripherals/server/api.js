import fetch from 'node-fetch';
import Ship  from './state.js'

const addr = "http://localhost:8080/"
const msg = "[EEApi] "

const s = "getSystem"
const r = "reactor"
const stateReq = "hull=getHull()&rhealth="+s+"Health('"+r+"')&rheat="+s+"Heat('"+r+"')&rpwr="+s+"Power('"+r+"')&rcool="+s+"Coolant('"+r+"')&rmhealth="+s+"HealthMax('"+r+"')"
const alertLvls = ["Normal","YELLOW ALERT","RED ALERT"]

export default class EEApi {
    static async setReactorHealth(health){
        const response = await fetch(addr+"exec.lua", {method: 'POST', body: "return getPlayerShip(-1):setSystemHealth('reactor',"+health+")"});
        if(response.ok){
            console.log(msg+"Reactor health set to "+health+"]")
        }
    }

    static async setReactorMaxHealth(maxhealth){
        
    }

    static async getShipState(){
        const response = await fetch(addr+"get.lua?"+stateReq)
        if(!response.ok){ return null }
        let data = await response.json()
        if(data.ERROR != undefined){return null}
        
        let ship = new Ship()
        
        ship.hull = data.hull
        ship.reactor[0]=(data.rhealth)
        ship.reactor[1]=(data.rheat)
        ship.reactor[2]=(data.rpwr)
        ship.reactor[3]=(data.rcool)
        ship.reactor[4]=(data.rmhealth)

        return ship
    }

    static async setAlertLevel(level){
        if(level < 0 || level > 2){
            console.log(msg+"alert level does not exist")
            return
        }
        const response = await fetch(addr+"exec.lua", {method: 'POST', body: "return getPlayerShip(-1):commandSetAlertLevel('"+alertLvls[level]+"')"});
        if(response.ok){
            let data = await response.json()
            if(data.ERROR != undefined){
                console.log(data.ERROR)
            }
        }

        const r = await fetch(addr+"exec.lua", {method: 'POST', body: "return getPlayerShip(-1):getAlertLevel()"});
        if(r.ok){
            let data = await r.json()
            console.log(data)
        }
    }

    static async addToShipLog(msg,color){
        const response = await fetch(addr+"exec.lua", {method: 'POST', body: "return getPlayerShip(-1):addToShipLog('"+msg+"','"+color+"')"});
        if(response.ok){
            let data = await response.json()
            if(data.ERROR != undefined){
                console.log(data.ERROR)
            }
        }
    }
}