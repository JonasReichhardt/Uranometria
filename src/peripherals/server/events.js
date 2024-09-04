import EEApi from './api.js'

export default class Events {
    static rtorCtrlMal(event) {
        EEApi.setAlertLevel(2)
        EEApi.addToShipLog(event.desc,'Yellow')            
    }
    static rtorCoolLeak(event) {
        EEApi.setAlertLevel(2)
        EEApi.addToShipLog(event.desc,'Red')        
    }
}