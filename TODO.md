# TODO

## Observations

- Implement a _client-server_ architecture over **TCP/IP**

- The system is composed of the main models: Cars & CSs (Charging Stations)

- When the battery of a vehicle hits the critical level, the system must
  point it to the nearest CS, taking in account: the distance between the
  vehicle and the CP, as well as the current occupation of the CP

- The system must distribute the demand across the CPs to reduce the waiting
  time of the drivers

- The driver is capable of making a reservation of a CP before he arrives
  at it. The reservation must be withdrawn once the vehicle is charged, making
  the CP available for another driver

- The billing must be registered on the driver's user account, allowing him to verify and
  perform the payments via PIX or any other form of electronic payment

## Requirements

- [ ] Establish what informations is going to be exchanged between the systems

- [ ] Make sure the data can be sent/received through the systems
