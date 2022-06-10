use keyboard_query::{DeviceQuery, DeviceState};
use monadic::{
    state::{get, put, State},
    state_trans::{get as stt_get, put as stt_put, StateT},
    stdo, stt_mdo,
};
use std::{
    // thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
struct Sun {
    status: SunShine,
}

#[derive(Clone, Debug)]
struct Projectile {
    x: i32,
    _y: i32,
}

#[derive(Clone, Debug)]
struct AppState {
    time: Instant,
    sun: Sun,
    proj: Projectile,
}

#[derive(Clone, Copy, Debug)]
enum SunShine {
    Shining,
    Cool,
}

type St = AppState;

fn init_state() -> St {
    AppState {
        time: Instant::now(),
        sun: Sun {
            status: SunShine::Cool,
        },
        proj: Projectile { x: 9, _y: 0 },
    }
}

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

fn key_loop() {
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<u16> = Vec::new();
    let mut st = init_state();

    loop {
        let keys = device_state.get_keys();

        if prev_keys != keys && !keys.is_empty() {
            match keys[0] {
                32 => {
                    println!("space bar pressed at time {:?}", Instant::now());
                    prev_keys = keys;
                    continue;
                }
                27 => {
                    break;
                }
                _ => {
                    prev_keys = keys;
                    continue;
                }
            }
        }

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

fn main() {
    // let st = init_state2();
    // println!("our results: \n{:?}", st);

    // let res = update_state().initial_state(init_state());

    // let res = loop_monad().initial_state(init_state());
    // println!("our results: \n{:?}", res);
    key_loop();
}
