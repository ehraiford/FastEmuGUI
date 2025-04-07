This is intended to be a reusable GUI to use when developing an emulator.
The idea is that you could connect an emulator you are writing to this to report register values, log messages, track execution timings, and see the frame buffer. 
The goal is to make it as unintrusive, quick to set up, resusable, and performant as possible. As such, a number of choices have been made to those ends:
  * This compiles to a C Dynamic Library. This allows it to be performant but also usable from any language that works with the C API.
  * This only accepts commands; it cannot issue them to the emulator. I'm still of two minds on this one. The GUI would be a great place to issue commands from, but it'd require more set up on the emulator side. With it just accepting commands, there's really no set up required besides invoking the start function.
  * It is written in Rust. Rust is the language I'm most confident I can write performant code with (plus, there are a number of crates that I'd like to get experience with)
  * It is very ugly. I'm not a GUI developer. If given the choice between improving the GUI's visuals or improving it's internals, I'm opting for the internals 9 times out of 10.
This is being developed alongside a GameBoy emulator I'm writing in C++. However, the emulator is the larger focus of the effort so updates to this will be minimal until I get the GameBoy more mature (at the very least until I can get it to pass some basic test functionality). That said, there are a number of features that I have planned to add in the future:
  * Message Logging
  * Top Level Execution Timing Tracking
  * Instruction Execution Timing
  * A not-thrown-together UI
