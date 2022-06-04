use std::time::{Duration, Instant};
// this is what I'm using for the first 3 examples.
use monadic::{mdo, monad::{Bind, Monad}, mio::{stdout_flush, print_str, read_line}};
// not sure why this is needed but here for monadics
use num::Integer;

// for the reader monad example
use monadic::{rdrdo, rdrt_mdo, 
    // reader::{Reader, ask, local}, 
    reader_trans::{ReaderT, ask, local}};
use partial_application::partial;
use std::collections::HashMap;

/// this stuff is all for the reader monad
type Env = HashMap<String, i32>;

fn immutable_insert( k_slice: &str, v: i32, dict: Env) -> Env {
    let mut dict1 = dict.clone();
    dict1.insert( String::from(k_slice), v);
    dict1
}

fn my_initial_env() -> Env {
    immutable_insert("a", 1, HashMap::new())
}


fn main() {


    let start = Instant::now();
    println!("Hello, world!");
    let duration = start.elapsed();
    println!("Printing 'Hello, world!' took about {:?} seconds?", duration);

    ////////////////////////////////////////////////////////////////////////////////
    // our mod! monads
    ////////////////////////////////////////////////////////////////////////////////
    let xs = mdo!{
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd();
        let z = match  x.is_even() {
            true => &y + 1, 
            _ => &y - 1,
        };
        pure (x, z)
    }.collect::<Vec<_>>();

    println!("result: {:?}", xs);

    let ys = mdo! {
        
        &y <- &vec![1, 2, 3, 4];
        guard y.is_odd();
        let z = y + 1;
        pure (y, z)
    }.collect::<Vec<_>>();

    println!("result: {:?}", ys);

    let res = mdo! {

        x <- pure 1;
        let y = x + 1;

        _ <- print_str("enter integer <i32>  ");
        _ <- stdout_flush();

        li1 <- read_line();
        z <- li1.trim().parse::<i32>();

        pure (y, z, li1.clone())
    }.collect::<Vec<_>>();

    println!("our result: {:?}", res);


    ////////////////////////////////////////////////////////////////////////////////
    // our reader monad example
    ////////////////////////////////////////////////////////////////////////////////
    // contains some stuff up at the top of the page as well.

    let modify_env = partial!(immutable_insert => "b", 2, _);

    // let bloc1: Reader<'_, Env, _> = rdrdo! {

    //     env1 <- ask();

    //     // subbloc w/ mod env
    //     pair <- local(modify_env, rdrdo! {
    //         x <- pure 9;
    //         y <- ask();
    //         pure (x, y)
    //     });

    //     pure (env1.clone(), pair.0, pair.1)
    // };

    // let resi = bloc1.initial_env(my_initial_env());
    // println!("results: {:?}", resi);

    let bloc2 = rdrt_mdo! {

        env1 <- ask();

        pair <- local(modify_env, rdrt_mdo! {

            x <- lift_iter 5..9;

            guard x.is_odd();

            let z = x + 1;
            y <- ask();

            pure (z, y)
        });

        pure (env1.clone(), pair.0, pair.1)
    };

    let resul = bloc2.initial_env(my_initial_env());

    println!("results: {:?}", resul);
    
}
