use std::time::{Duration, Instant};
use monadic::{mdo, monad::{Bind, Monad}};
use num::Integer;



fn main() {
    let start = Instant::now();
    println!("Hello, world!");
    let duration = start.elapsed();
    println!("Printing 'Hello, world!' took about {:?} seconds?", duration);

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

    println!("result: {:?}", ys)

}

