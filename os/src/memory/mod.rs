pub mod config;
pub mod heap;
pub mod address;
pub mod frame;
pub mod range;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init(){
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };
    println!("mod memory initialized")
}