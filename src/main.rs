// used to get keyboard inputs
use keyboard_query::{DeviceQuery, DeviceState};
// used to simulate monadic action blocks
use monadic::{
    state::{get, put, State},
    state_trans::{get as stt_get, put as stt_put, StateT},
    stdo, stt_mdo,
};
// used to span time
use std::{
    // thread,
    time::{Duration, Instant},
};

// sun object
#[derive(Clone, Copy, Debug)]
struct Sun {
    status: SunShine,
}

// projectile object
#[derive(Clone, Debug)]
struct Projectile {
    x: i32,
    _y: i32,
}

// app state
#[derive(Clone, Debug)]
struct AppState {
    time: Instant,
    sun: Sun,
    proj: Projectile,
}

// sunshine enum for sun
#[derive(Clone, Copy, Debug)]
enum SunShine {
    Shining,
    Cool,
}

// necessary definition for monadic crate, must be St
type St = AppState;

// just returns an app state with the initial values
fn init_state() -> St {
    AppState {
        time: Instant::now(),
        sun: Sun {
            status: SunShine::Cool,
        },
        proj: Projectile { x: 9, _y: 0 },
    }
}

// something I was playing around with to try to make the 
// stateful changes happen.
fn _init_state2() -> State<'static, St, ()> {
    stdo! {
        _ <- put(AppState {
        time: Instant::now(),
        sun: Sun {
            status: SunShine::Cool,
        },
        proj: Projectile { x: 9, _y: 0 },
        });
        pure ()
    }
}

// updates the initial state and returns monadic action block.
fn update_state() -> State<'static, St, ()> {
    let new_time = Instant::now();
    let new_state: State<'_, St, _> = stdo! {
    state <- get();
    let new_shine = match state.sun.status {
        SunShine::Cool => (Sun{ status: SunShine::Shining }),
        SunShine::Shining => (Sun{ status: SunShine::Cool }),
    };
    let new_proj = Projectile{ x: state.proj.x - 1, _y: 0 };

    _ <- put(AppState { time: new_time, sun: new_shine, proj: new_proj });
    pure ()
    };
    new_state
}

// non functional main loop of program
fn key_loop() {
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<u16> = Vec::new();
    let mut st = init_state();

    loop {
        // looks for the spacebar being pressed
        // exits out of loop if escape key is pressed
        let keys = device_state.get_keys();
        if prev_keys != keys && !keys.is_empty() {
            match keys[0] {
                32 => { // the space bar key code
                    println!("space bar pressed at time {:?}", Instant::now());
                    prev_keys = keys;
                    continue;
                }
                27 => { // the esc button key code
                    break;
                }
                _ => {
                    prev_keys = keys;
                    continue;
                }
            }
        }

        // updating state happens every one second.
        if st.time.elapsed() > Duration::from_secs(1) {
            // trying out recursive update_state function
            // let full_st = update_state().initial_state(update_state().initial_state(st).1);
            let full_st = update_state().initial_state(st);
            st = full_st.1;
            println!("this is our state: {:?}", st);
        }

        prev_keys = keys;
    }
}

// my attempt at creating a monadic stateful solution for the program
// was trying to put a loop inside of the action block, but that didn't 
// work out.
fn _loop_monad() -> StateT<'static, St, Vec<((), AppState)>, ()> {
    // let device_state = DeviceState::new();
    // let mut prev_keys: Vec<u16> = Vec::new();

    let loop_st = stt_mdo! {

        state <- stt_get();
        _ <- stt_put(_og_upst(state));
        pure ()
    };
    loop_st
}

// original updating state function.  takes in a state, changes
// the values and then returns that state.
fn _og_upst(init_st: St) -> St {
    let new_time = Instant::now();
    let new_state: State<'_, St, _> = stdo! {
    state <- get();
    let new_shine = match state.sun.status {
        SunShine::Cool => (Sun{ status: SunShine::Shining }),
        SunShine::Shining => (Sun{ status: SunShine::Cool }),
    };
    let new_proj = Projectile{ x: state.proj.x - 1, _y: 0 };

    _ <- put(AppState { time: new_time, sun: new_shine, proj: new_proj });
    pure ()
    };
    new_state.initial_state(init_st).1
}

// main call
fn main() {
    key_loop();
}
