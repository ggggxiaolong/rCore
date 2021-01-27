//! 内存管理模块
//!
//! 负责空间分配和虚拟地址映射

// 因为模块内包含许多基础设施类别，实现了许多以后可能会用到的函数，
// 所以在模块范围内不提示「未使用的函数」等警告
#![allow(dead_code)]

pub mod config;
pub mod heap;
pub mod address;
pub mod frame;
pub mod range;
mod mapping;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init(){
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };
    println!("mod memory initialized")
}