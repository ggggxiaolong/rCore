pub mod config;
mod heap;
mod address;

pub fn init(){
    heap::init();
    println!("mod memory initialized")
}