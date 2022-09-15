import board  # type: ignore (this is a CircuitPython built-in)
import digitalio
from joystick_xl.inputs import Axis, Button
from joystick_xl.joystick import Joystick

joystick = Joystick()
sel_hvli = Button(board.GP9,False)
sel_homing = Button(board.GP10,False)
load_tube1 = Button(board.GP11,False)
load_tube2 = Button(board.GP12,False)
unload_tube1 = Button(None,False)
unload_tube2 = Button(None,False)
fire = Button(board.GP13,False)

unload = digitalio.DigitalInOut(board.GP15)
unload.direction = digitalio.Direction.INPUT
unload.pull = digitalio.Pull.DOWN

joystick.add_input(sel_hvli,sel_homing,load_tube1,load_tube2,unload_tube1, unload_tube2,fire)

while True:
    if unload.value is True:
        if load_tube1.is_pressed:
            unload_tube1.source_value = True
        else:
            unload_tube1.source_value = False
        if load_tube2.is_pressed:
            unload_tube2.source_value = True
        else:
            unload_tube2.source_value = False
        load_tube1.bypass = True
        load_tube2.bypass = True
    else:
        unload_tube1.source_value = False
        unload_tube2.source_value = False
        load_tube1.bypass = False
        load_tube2.bypass = False
    joystick.update()