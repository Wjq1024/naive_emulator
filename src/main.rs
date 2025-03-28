use std::usize;

use cpu::{CPU, cpu_run};
use loader::test_load;

mod common;
mod cpu;
mod loader;
mod memory;
mod uniprocessor;

fn main() {
    test_load();
    cpu_run(usize::MAX);
    println!("{:?}", CPU.exclusive_access());
}
