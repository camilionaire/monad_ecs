# Grad Project for Adv Fn Prog
### Camilo Schaser-Hughes
### June 10, 2022
### CS510 - Prof. Casamento 

---

## Monadic Simulation of Entity Component System

### Installation:

1. [GitHub with Code](https://github.com/camilionaire/monad_ecs.git)
2. To Build: `cargo build`
3. To Run: `cargo run`

### Game Play :
 * `cargo run` to start
 * `SPACEBAR` to initiate unfinished jump feature
 * `ESC` to end program

### Dependencies:
 * monadic = "0.5.5" ( contains macros to monadic action blocks for State monad among others)
 * keyboard_query = "0.1.0" ( allows for the input and reading of keyboard events)
 * a couple other dependencies are installed but are not used in current iteration
 * NOTE: for keyboard_query on Unix Env may need to install `libx11-dev` or `xorg-x11-server-devel`
