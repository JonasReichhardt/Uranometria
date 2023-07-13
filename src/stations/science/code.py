import board
from joystick_xl.inputs import Axis, Button
from joystick_xl.joystick import Joystick

joystick = Joystick()

param1 = Axis(board.A3, min=350,max=65000,deadband=5000)
param2 = Axis(board.A1,min=350,max=65000,deadband=5000)
param3 = Axis(board.A0,min=350,max=65000,deadband=5000)
param4 = Axis(board.A2,min=350,max=65000,deadband=5000)

scan_btn = Button(board.D13,False)
sclt_btn = Button(board.D12,False)

joystick.add_input(param1,param2,param3,param4,sclt_btn,scan_btn)

while True:
    joystick.update()
