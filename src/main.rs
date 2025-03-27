use std::usize;

use cpu::{CPU, cpu_run};
use loader::naive_load;

mod common;
mod cpu;
mod loader;
mod memory;
mod uniprocessor;

fn main() {
    naive_load();
    cpu_run(usize::MAX);
    println!("{:?}", CPU.exclusive_access());
}
