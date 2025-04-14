# WoodChipper
A VERY barebones dissasembler for chip8 "ROMs".

This primarily exists so that I can see what ROMs are roughly doing. You certainly couldn't recreate a ROM with it's output.

It takes two arguments with the second being optional: (1) The path to the .ch8 file (2) The directory where the output should be written to.

With cargo installed, you can run the program like this:
```bash
cargo run <.ch8 file> <directory>
```
