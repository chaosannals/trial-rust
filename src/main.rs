extern crate rand;

use rand::prelude::*;
use rand::distributions::{Uniform};

fn make_set(length: i32) -> Vec<i32> {
    let mut r: Vec<i32> = vec![];
    let u = Uniform::new(0, 10000000);
    for _ in 0..length {
        r.push(u.sample(&mut rand::thread_rng()));
    }
    return r;
}

fn main() {
    let a: Vec<i32> = make_set(1000000);
    let b: Vec<i32> = make_set(1000000);
}
