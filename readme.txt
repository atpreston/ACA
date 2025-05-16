Requires cargo to compile - execute `cargo run`  in the main directory to compile and run the simulator.

Alternatively, execute the binary file `coursework` in the main directory


Change values including:
- Name of program
- Size of memory
- Number of general purpose registers
- Number of execution units
- Number of reservation slots per reservation station

by changing the constants at the top of `./src/processor.rs`


Controls:
- 'Space' - step 1 cycle through the processor
- 'q' - kill the program (use Ctrl + C for constant running)
- 'r' - begin constant running at 2Hz
- 'Enter' - Run with no coded limit on clock speed