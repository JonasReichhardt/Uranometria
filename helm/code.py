import board
import digitalio
from joystick_xl.inputs import Button
from joystick_xl.joystick import Joystick

joystick = Joystick()

warp = [Button(board.GP18,False),Button(board.GP19,False),Button(board.GP20,False),Button(board.GP21,False),Button(board.GP22,False)]
exit_warp = Button(board.GP16,False)

dock_button = Button(board.GP17,False)

led_pins = [board.GP10,board.GP11,board.GP12, board.GP13]
leds = []

for pin in led_pins:
    led_pin = digitalio.DigitalInOut(pin)
    led_pin.direction = digitalio.Direction.OUTPUT
    leds.push(led_pin)

for btn in warp:
    joystick.add_input(btn)
joystick.add_input(exit_warp,dock_button)


while True:
    for i in range(len(warp)-1):
        leds[i].value = warp[i].is_pressed
    joystick.update()