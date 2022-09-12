# Axon Native

## *WIP README, REPO PRONE TO CHANGE*

This repository handles the both the bindings to a Typescript, as well as a custom serial protocol for communicating to an Arduino. 

Before communicating over UART serial, a handshake between the this instance and the serial device is established.  Utilizing this, any data sent over serial can be read either in a Rust program, or Typescript if needed via `index.node`.

