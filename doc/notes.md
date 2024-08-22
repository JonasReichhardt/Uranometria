# Uranometria

## Goals
Create a LARP experience inside of EmptyEpsilon with is portable and customizable.

## Todos
+ Create prototype for a peripheral system (e.g reactor, missile system etc.)

## Ideas
Implement a complete custom backend with MQTT and other interfaces with bridges the gap between the HTTP API of EmptyEpsilon and peripherals.

### Reactor
If the ingame reactor is damaged under 50%/25% of it's health there is a chance a reactor malfunction with severity low/high occurs. While a reaction malfunction is present the reactor cannot be repaired over 50%/25% health via the ingame repair crews. In extreme scenarios repeair crews inside EmptyEpsilon can be disabled completly. Resolving the malfunction on the reactor prop heals the reactor by 50% for high severity and 25% for low severity. 

#### Repair events

| Problem                             | Fix                                                 | Severity | Status effect                                                |
| ----------------------------------- | --------------------------------------------------- | -------- | ------------------------------------------------------------ |
| Secondary fuel pump outage detected | Rewire secondary fuel pump at reactor control panel | Low      | If main fuel pump is damaged a reactor outage is imminent.   |
| Reactor coolant leak                | Restock coolant and replace coolant pipes           | High     | Ship leaks coolant until problem is resolved                 |
| Reactor controller malfunction      | Reboot reactor                                      | Low      | Reactor performance is inhibited to a certain percentage and if raised reactor might damage itself. |


#### Physical appearance



#### Hardware



#### Software

