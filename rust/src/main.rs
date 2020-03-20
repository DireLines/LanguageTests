use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let init_time = Instant::now();
    let arr = randoms(5000000);
    println!(
        "init     took {:} \tmilliseconds",
        init_time.elapsed().as_millis()
    );
    let serial_time = Instant::now();
    let s = sum_of_squares_serial(&arr);
    println!(
        "serial   took {:} \tmilliseconds",
        serial_time.elapsed().as_millis()
    );
    let parallel_time = Instant::now();
    let p = sum_of_squares_parallel(&arr);
    println!(
        "parallel took {:} \tmilliseconds",
        parallel_time.elapsed().as_millis()
    );
    assert!(s == p);
}

fn randoms(howmany: i32) -> Vec<i64> {
    (0..howmany)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen_range(1, 21)
        })
        .collect()
}

fn sum_of_squares_parallel(input: &[i64]) -> i64 {
    input.par_iter().map(|&i| i * i).sum()
}

fn sum_of_squares_serial(input: &[i64]) -> i64 {
    input.iter().map(|&i| i * i).sum()
}
