#![feature(exclusive_range_pattern)]
#![feature(ptr_const_cast)]

mod alloc;
mod safeptr;
mod data;
mod types;
mod memory;
mod array;
mod bytecode;
mod context;
mod constants;
mod error;
mod printer;
mod op;
mod vm;

fn main() {
    println!("Hello, world!");
}
