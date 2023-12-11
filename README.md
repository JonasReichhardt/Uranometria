# Goals

Hardware controllers for [EmptyEpsilon](https://github.com/daid/EmptyEpsilon) stations.
Peripheral devices for LARP gameplay.

## Stations
# Implementation

Each station is controlled by a [Raspberry Pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) which emulates a joystick for the OS with the [JoystickXXL]() library for [CircuitPython]().

_Note:_ on the Science station a Adafruit Feather RP2040 is used due to the fact that the board of the Pico uses one of the four analog inputs of its RP2040 for voltage sensor but for the station four analog inputs are needed to avoid using a external ADC.

Additionally the Helms stations includes a [Thrustmaster T.Flight Hotas X](https://www.thrustmaster.com/en-us/products/t-flight-hotas-x/) for maneuvering the spaceship.

## Peripherals
# TODO
Look into [this](https://github.com/OdysseusLarp/odysseus-mct) repo
