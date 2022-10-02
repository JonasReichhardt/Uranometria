from select import select
import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn
from joystick_xl.inputs import Axis, Button
from joystick_xl.joystick import Joystick


#i2c = busio.I2C(scl=board.GP13, sda=board.GP12)
#ads = ADS.ADS1115(i2c)
#ads.gain = 2/3

joystick = Joystick()

#x = Axis(AnalogIn(ads, ADS.P0, ADS.P1), max=14000, invert=True)
y = Axis(board.A2,min=400,max=65300,deadband=5000)
z = Axis(board.A0,min=400,max=65300,deadband=5000)
k = Axis(board.A1,min=400,max=65300,deadband=5000)

scan_btn = Button(board.GP16,False)
sclt_btn = Button(board.GP17,False)

joystick.add_input(
    #x,
    y,z,k,scan_btn,sclt_btn
)

while True:
    joystick.update()
