use keyboard_query::{DeviceQuery, DeviceState};
use monadic::{
    state::{get, put, State},
    stdo,
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
struct Sun {
    status: SunShine,
}

#[derive(Clone, Debug)]
struct Projectile {
    x: i32,
    y: i32,
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

fn initial_state() -> St {
    let a_state: State<'_, St, _> = stdo! {
        shine <- get();
        _ <- put(shine);
        pure ("hello")
    };

    let result = a_state.initial_state(AppState {
        time: Instant::now(),
        sun: Sun {
            status: SunShine::Cool,
        },
        proj: Projectile { x: 9, y: 0 },
    });
    println!("our initial state of our sun is: {:?}", result);

    result.1
}

fn update_state(init_st: St) -> St {
    if init_st.time.elapsed() > Duration::from_secs(1) {
        let new_time = Instant::now();
        let new_state: State<'_, St, _> = stdo! {
            state <- get();
            let new_shine = match state.sun.status {
                SunShine::Cool => (Sun{ status: SunShine::Shining }),
                SunShine::Shining => (Sun{ status: SunShine::Cool }),
            };
            let new_proj = Projectile{ x: state.proj.x - 1, y: 0 };

            _ <- put(AppState { time: new_time, sun: new_shine, proj: new_proj });
            pure ("Changing Game State")
        };
        let newer = new_state.initial_state(init_st);

        println!("our new state of our sun is: {:?}", newer);

        newer.1
    } else {
        init_st
    }
}

fn main() {
    let start = Instant::now();
    println!("Hello World!, this is the MAIN.RS!!!");
    let duration = start.elapsed();
    println!("We've been running this for {:?} amount of time", duration);

    let mut result = initial_state();
    println!("our initial state of our sun is: {:?}", result);
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<u16> = Vec::new();

    loop {
        let keys = device_state.get_keys();
        if prev_keys != keys && !keys.is_empty() && keys[0] == 32 {
            println!("space bar pressed at time {:?}", Instant::now());
        }
        result = update_state(result);
        prev_keys = keys;
    }
}
