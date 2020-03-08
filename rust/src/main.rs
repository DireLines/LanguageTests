use std::any::type_name;
use std::env;
mod other;

fn main() {
    println!("pi: {}", other::pi());
    let fname = env::args().nth(1).expect("no filename supplied");
    let strang: String = other::read_to_string(&fname).unwrap();
    println!("strang: {}", strang);
    let s1 = "hello dolly".to_string();
    let s2 = &s1;
    println!("s2: {}", s2);

    let f = |x| x * x;
    println!("{:?}", f(10));
    let f2 = |x| x * x;
    println!("{:?}", f2(3.4));
}
