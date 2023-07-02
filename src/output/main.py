import ustruct
import usocket
import neopixel
import network
from machine import Pin

ROOT_LAYER = "HH12sHI16s"
FRAME_LAYER = "HI64sBHBBH"
DMP_LAYER = "HBBHHH"
FMT = ">" + ROOT_LAYER + FRAME_LAYER + DMP_LAYER
LED_RESET = False

# Pixel configuration for 
#https://o.lnwfile.com/_/o/_raw/li/hd/gs.jpg
PIXELNUM = 8
pin = Pin(4,Pin.OUT)
n = neopixel.NeoPixel(pin,PIXELNUM)

def decodeE131(packet):
    if len(packet) < 125:
        return None
    preamble_size, postamble_size, packet_identifier, root_flags_length, root_vector, sender_id, \
    frame_flags_Length, frame_vector, source_name, priority, synchronization_address, sequence_number, options, universe, \
    dpm_flags_length, dpm_vector, address_data_type, first_property_address, addres_increment, property_count \
     = ustruct.unpack(FMT, packet[:125])
    if preamble_size != 0x10 or postamble_size != 0x00:
        return None
    if packet_identifier != b'\x41\x53\x43\x2d\x45\x31\x2e\x31\x37\x00\x00\x00':
        return None
    if root_vector != 4 or frame_vector != 2 or dpm_vector != 2 or address_data_type != 0xA1:
        return None
    # There are a few more things that could be checked, like lengths and stuff.
    # But at this point we should be pretty sure this is DMX data.
    return packet[126:]

def do_connect():
    import network
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    if not wlan.isconnected():
        print('connecting to network...')
        wlan.connect('ssid', 'pwd')
        while not wlan.isconnected():
            pass
    print('network config:', wlan.ifconfig())

def reset_leds():
    print("reset leds")
    for i in range(PIXELNUM):
        n[i] = (0,0,0)
    n.write()
    
def set_leds(data):
    n[0] = (data[0],data[1],data[2])
    n[1] = n[0]
    
    n[2] = (data[3],data[4],data[5])
    n[3] = n[2]
    
    n[4] = (data[6],data[7],data[8])
    n[5] = n[4]
    
    n[6] = (data[9],data[10],data[11])
    n[7] = n[6]
    n.write()

do_connect()
reset_leds()
s = usocket.socket(usocket.AF_INET, usocket.SOCK_DGRAM, usocket.IPPROTO_IP)
s.bind(('', 5568))

while True:
    data, client = s.recvfrom(2048)
    data = decodeE131(data)
    if data is not None:
        print(data)
        set_leds(data)
        LED_RESET = False
    elif not LED_RESET:
        reset_leds()
        LED_RESET = True
