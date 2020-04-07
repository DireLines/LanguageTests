use ndarray::arr2;
use ndarray::Array;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    let v = randoms(100);
    for (i, n) in v.iter().enumerate() {
        println!("{:}   {:}", i, n);
    }
}

fn randoms(howmany: u32) -> Vec<i64> {
    (0..howmany)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen_range(1, 21)
        })
        .collect()
}
