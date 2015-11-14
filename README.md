# Conway's Game of Life + Conrod GUI Library = Conrod's Game of Life

The goal here is to show off some features of the (Conrod)[https://github.com/PistonDevelopers/conrod] library using (Conway's Game of Life)[https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life] as an example.

### Build

1. Make sure you have Rust 1.3 or later installed.
2. Install the Freetype library required by Conrod. You can download it (here)[http://www.freetype.org/download.html] or install using Homebrew on OSX: `brew install freetype` 
3. `cargo run`


Note that this currently only runs on OSX because the Font path is hard coded to the OSX System font path. Should be a simple change to run it on another OS.

