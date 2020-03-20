use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;
fn sum_of_squares_parallel(input: &[i64]) -> i64 {
    input.par_iter().map(|&i| i * i).sum()
}

fn sum_of_squares_serial(input: &[i64]) -> i64 {
    input.iter().map(|&i| i * i).sum()
}

fn main() {
	println!("start initializing");
    let arr = randoms(50000000);
    println!("done initializing");
    let serial_time = Instant::now();
    let _ = sum_of_squares_serial(&arr);
    println!("serial   took {:} \tmicroseconds", serial_time.elapsed().as_micros());
    let parallel_time = Instant::now();
    let _ = sum_of_squares_parallel(&arr);
    println!("parallel took {:} \tmicroseconds", parallel_time.elapsed().as_micros());
}
fn randoms(howmany: i32) -> Vec<i64> {
    (0..howmany).into_par_iter().map(|_| {
    	let mut rng = rand::thread_rng();
    	rng.gen_range(1, 21)
    }).collect()
}
