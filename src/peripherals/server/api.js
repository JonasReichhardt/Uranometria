import fetch from 'node-fetch';
import Ship from './state.js'

const addr = "http://localhost:8080/"
const msg = "[EEApi] "

const s = "getSystem"
const r = "reactor"
const stateReq = "cool=getMaxCoolant()&hull=getHull()&rhealth=" + s + "Health('" + r + "')&rheat=" + s + "Heat('" + r + "')&rpwr=" + s + "Power('" + r + "')&rcool=" + s + "Coolant('" + r + "')&rmhealth=" + s + "HealthMax('" + r + "')"
const alertLvls = ["Normal", "YELLOW ALERT", "RED ALERT"]

export default class EEApi {
    static async setReactorHealth(health) {
        const response = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):setSystemHealth('reactor'," + health + ")" });
        await this.catchError(response)
    }

    static async setReactorMaxHealth(maxhealth) {
        const response = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):setSystemHealthMax('reactor'," + maxhealth + ")" });
        await this.catchError(response)
    }

    static async setMaxCoolant(coolant){
        const response = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):setMaxCoolant(" + (coolant+0.0000001) + ")" });
        await this.catchError(response)
    }

    static async getMaxCoolant(){
        const response = await fetch(addr + "get.lua?cool=getMaxCoolant()")
        if (!response.ok) { return null }
        let data = await response.json()
        if (data.ERROR != undefined) { return null }
        return data.cool
    }

    static async getShipState() {
        const response = await fetch(addr + "get.lua?" + stateReq)
        if (!response.ok) { return null }
        let data = await response.json()
        if (data.ERROR != undefined) { return null }

        let ship = new Ship()
        ship.hull = data.hull
        ship.coolant = data.cool
        ship.reactor[0] = (data.rhealth)
        ship.reactor[1] = (data.rheat)
        ship.reactor[2] = (data.rpwr)
        ship.reactor[3] = (data.rcool)
        ship.reactor[4] = (data.rmhealth)

        return ship
    }

    static async setAlertLevel(level) {
        if (level < 0 || level > 2) {
            console.log(msg + "alert level does not exist")
            return
        }
        const response = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):commandSetAlertLevel('" + alertLvls[level] + "')" });
        await this.catchError(response)

        const r = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):getAlertLevel()" });
        if (r.ok) {
            let data = await r.json()
            console.log(data)
        }
    }

    static async addToShipLog(msg, color) {
        const response = await fetch(addr + "exec.lua", { method: 'POST', body: "return getPlayerShip(-1):addToShipLog('" + msg + "','" + color + "')" });
        await this.catchError(response)
    }

    static async catchError(response) {
        if (!response.ok) { return }
        let data = await response.json()
        if (data.ERROR == undefined) { return }
        console.log(data.ERROR)
    }
}