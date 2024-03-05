# Uranometria

## Goals
Create a LARP experience inside of EmptyEpsilon with is portable and customizable.

## Todos
+ Create prototype for a peripheral system (e.g reactor, missile system etc.)

## Ideas
Implement a complete custom backend with MQTT and other interfaces with bridges the gap between the HTTP API of EmptyEpsilon and peripherals.

### Reactor
If the reactor is damaged under 50% of it's health the peripheral gets notified via a request to initiate a repair event. This means the reactor cannot be repaired until some tasks at the peripheral are done. In extreme scenarios repeair crews inside EmptyEpsilon can be disabled completly. In this case a second or third member has to be added to the engineering team to complete for in-battle repairs.



**Repair events:**

| Problem                             | Fix                                                 | Severity | Status effect                                                |
| ----------------------------------- | --------------------------------------------------- | -------- | ------------------------------------------------------------ |
| Secondary fuel pump outage detected | Rewire secondary fuel pump at reactor control panel | Low      | If main fuel pump is damaged a reactor outage is imminent.   |
| Reactor coolant leak                | Restock coolant and replace coolant pipes           | High     | Ship leaks coolant until problem is resolved                 |
| Reactor controller malfunction      | Reboot reactor                                      | Low      | Reactor performance is inhibited to a certain percentage and if raised reactor might damage itself. |
|                                     |                                                     |          |                                                              |
|                                     |                                                     |          |                                                              |
|                                     |                                                     |          |                                                              |

#### Physical appearance



#### Hardware



#### Software

