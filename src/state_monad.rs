use monadic::{stdo, stt_mdo, state::{State, get, put}, state_trans::{StateT, get as stt_get, put as stt_put}};
use num::Integer;

type St = i32;

fn main() {
    println!("Hello, world!");

    let bloc: State<'_, St, _> = stdo! {

        x <- pure 9;
        y <- get();

        _ <- put(1);
        z <- get();

        pure (x, y, z)
    };

    let res = bloc.initial_state(0);

    println!("result: {:?}", res);

    let bloc2 =  stt_mdo! {

        x <- lift_iter 5..9;
        guard x.is_odd();

        y <- stt_get();

        _ <- stt_put (1);
        z <- stt_get();

        let v = x + 1;

        pure (v, y, z)
    };

    let resi = bloc2.initial_state(0);

    println!("sst results: {:?}", resi);
}
