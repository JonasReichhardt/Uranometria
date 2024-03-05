import network
import machine
from time import sleep_ms
import urequests as requests

def setup():
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)

    if not wlan.isconnected():
        print('connecting to network...')
        wlan.connect('Wlan_Reichhardt', 'dkp%67fg')
        while not wlan.isconnected():
            pass
        print('network config:', wlan.ifconfig())

def setLimit(amount):
    d = 'return getPlayerShip(-1):setSystemHealth("reactor",'+str(amount)+')'
    print(d)
    try:
        res = requests.post(url='http://192.168.0.105:8080/exec.lua',data=d)
        res.close()
    except TypeError:
        print("ok")

def main():
    pin = machine.Pin(4, machine.Pin.IN,machine.Pin.PULL_UP)
    isBlocked = False
    val = 0
    while True:
        if pin.value() is 0:
            if isBlocked:
                val = 1
                isBlocked = False
            else:
                val = 0.25
                isBlocked = True
            setLimit(val)
        sleep_ms(500)

main()







    
