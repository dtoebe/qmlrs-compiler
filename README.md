# qmlrs-compiler
WIP (very early) compiler for qmlrs (https://github.com/cyndis/qmlrs) qml bindings for Rust

## By Work in Progress I really mean it _Partially Working_

Basically I am very new to rust and wanted to create a compiler to compile QML files in to the compiled Rust binaries, simular to the compiler that Go-QML has.

I am taking a simple approch that this will take the QML file and add it to the Rust source code as a String.

Everything at this point is subject to change.

### What I would like to do:

- [ ] Compile QML files into Rust binaries
- [ ] Compile images with rust binaries
- [ ] Create a lib so you can integrate it into a build step in Cargo
- [ ] See where this takes me...

- [ ] Also see if there is a way I can compile from a buffer instead of writing files, then compiling...

### Currently working on:

Right now I am working on the command line options.  Infact You can only print the help screen.  If you remove the `return` in the main it will go back to normal working order.


### Contributing:

Like I said I am very new to Rust, and will take any suggestions on how to move this project forward, or how I can improve my code.

_Side note: For those thinking to get started with Rust, or new to it.  Once you get past fighting with rustc over ownership issues, and begin to understand it.  Rust is a blast to write. And the community is awesome.  **I highly recommend it**_
