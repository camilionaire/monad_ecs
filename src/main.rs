use monadic::{stdo, state::{State, get, put}};
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
struct AppState {
    solar: Sun,
}
#[derive(Clone, Debug)]
struct Sun {
    time_ch: Instant,
    status: SunShine,
}

#[derive(Clone, Debug)]
enum SunShine {
    Shining,
    Cool,
}

type St = Sun;

fn initial_state() -> St {
    let a_state: State<'_, St, _> = stdo! {
        shine <- get();
        _ <- put(shine);
        pure ("hello")
    };
    let result = a_state.initial_state(Sun{ time_ch: Instant::now(), 
        status: SunShine::Cool });
    println!("our initial state of our sun is: {:?}", result);

    result.1
}

fn update_sun(init_st: St) -> St {
    if init_st.time_ch.elapsed() > Duration::from_secs(1) {
        let new_time = Instant::now();
        let new_state: State<'_, St, _> = stdo! {
            shine <- get();
            let new_shine = match shine.status {
                SunShine::Cool => (Sun{ time_ch: new_time, status: SunShine::Shining }),
                SunShine::Shining => (Sun{ time_ch: new_time, status: SunShine::Cool }),
            };
            _ <- put(new_shine);
            // z <- get();
            pure ("Changing Sun State")
        };
        let newer = new_state.initial_state(init_st);
        
        println!("our new state of our sun is: {:?}", newer);

        newer.1
    } else { init_st }
}
fn main() {
    let start = Instant::now();
    println!("Hello World!, this is the MAIN.RS!!!");
    let duration = start.elapsed();
    println!("We've been running this for {:?} amount of time", duration);




    let mut result = initial_state();
    println!("our initial state of our sun is: {:?}", result);

    // let newer = update_sun(result);
    // println!("our new state of our sun is: {:?}", newer);

    // let newest = update_sun(newer);
    // println!("our new state of our sun is: {:?}", newest);

    loop {
        result = update_sun(result);
    }
}