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

fn init_state() -> St {
    AppState {
        time: Instant::now(),
        sun: Sun {
            status: SunShine::Cool,
        },
        proj: Projectile { x: 9, y: 0 },
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
        let new_proj = Projectile{ x: state.proj.x - 1, y: 0 };

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
            if keys[0] == 32 {
                println!("space bar pressed at time {:?}", Instant::now());
            } else if keys[0] == 27 {
                break;
            }
        }

        if st.time.elapsed() > Duration::from_secs(1) {
            let full_st = update_state().initial_state(st);
            st = full_st.1;
            println!("this are our state: {:?}", st);
        }

        prev_keys = keys;

    }
}


fn main() {
    key_loop();
}
