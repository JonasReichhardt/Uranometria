import board
import digitalio
from joystick_xl.inputs import Button
from joystick_xl.joystick import Joystick

joystick = Joystick()

select_wpn = [Button(board.GP13,False),Button(board.GP10,False),Button(board.GP11,False),Button(board.GP12,False)]

load_tubes = [Button(board.GP9,False),Button(board.GP6,False),Button(board.GP7,False),Button(board.GP8,False),Button(board.GP20,False)]

unload_tubes = [Button(None,False),Button(None,False),Button(None,False),Button(None,False),Button(None,False)]

fire = [Button(board.GP16,False),Button(board.GP17,False),Button(board.GP15,False),Button(board.GP14,False),Button(board.GP19,False)]

unload = digitalio.DigitalInOut(board.GP18)
unload.direction = digitalio.Direction.INPUT
unload.pull = digitalio.Pull.DOWN

safety = digitalio.DigitalInOut(board.GP21)
safety.direction = digitalio.Direction.INPUT
safety.pull = digitalio.Pull.DOWN

for wpn in select_wpn:
    joystick.add_input(wpn)
for tube in load_tubes:
    joystick.add_input(tube)
for tube in unload_tubes:
    joystick.add_input(tube)
for fire_btn in fire:
    joystick.add_input(fire_btn)

while True:
    if unload.value is True:
        for i in range(len(load_tubes)):
            unload_tubes[i].source_value = load_tubes[i].is_pressed
            load_tubes[i].bypass = True
    else:
        for i in range(len(load_tubes)):
            unload_tubes[i].source_value = False
            load_tubes[i].bypass = False
    for i in range(len(fire)):
        fire[i].bypass = not safety.value
    joystick.update()
