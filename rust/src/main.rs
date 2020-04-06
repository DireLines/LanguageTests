use ndarray::arr2;
use ndarray::Array;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
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
