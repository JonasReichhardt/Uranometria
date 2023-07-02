import board
from joystick_xl.inputs import Axis, Button
from joystick_xl.joystick import Joystick

joystick = Joystick()

a = Axis(board.A0, min=400,max=65300,deadband=5000)
b = Axis(board.A1,min=400,max=65300,deadband=5000)
c = Axis(board.A2,min=400,max=65300,deadband=5000)
d = Axis(board.A3,min=400,max=65300,deadband=5000)

scan_btn = Button(board.D13,False)
sclt_btn = Button(board.D12,False)

joystick.add_input(
    a,b,c,d,scan_btn,sclt_btn
)

while True:
    joystick.update()
