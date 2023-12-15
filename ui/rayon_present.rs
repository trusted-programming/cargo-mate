// run-rustfix
use rayon::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    (0..100)
        .into_par_iter()
        .for_each(|_| thread::sleep(Duration::from_secs(1)));
}