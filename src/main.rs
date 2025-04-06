use std::{env, usize};

use cpu::{CPU, cpu_run};
use loader::{binary_file_load, test_load};

mod common;
mod cpu;
mod loader;
mod memory;
mod uniprocessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        test_load();
    } else {
        binary_file_load(args[1].clone());
    }
    cpu_run(usize::MAX);
    println!("{:?}", CPU.exclusive_access());
}
